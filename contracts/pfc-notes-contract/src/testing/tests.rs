// ----------------------------------- Tests -----------------------------------

use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Addr, StdError,
};
use cw2::ContractVersion;
use cw_ownable::Ownership;

use pfc_notes::{InstantiateMsg, NoteEntry, NoteType, NoteWriter};

use crate::error::ContractError;
use crate::execute::{exec_add_note, exec_add_writer, exec_rm_note, exec_rm_writer};
use crate::query::{query_entries, query_note, query_sub_topics, query_topics, query_writer};
use crate::{instantiate, CONTRACT_NAME, CONTRACT_VERSION};

//use super::*;

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies();

    // run instantiation logic
    instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("larry", &[]),
        InstantiateMsg {
            owner: "pumpkin".into(),
            writers: vec![],
        },
    )
    .unwrap();

    // correct cw2 version info should have been stored
    let version = cw2::get_contract_version(deps.as_ref().storage).unwrap();
    assert_eq!(
        version,
        ContractVersion {
            contract: CONTRACT_NAME.into(),
            version: CONTRACT_VERSION.into(),
        },
    );

    // correct ownership info should have been stored
    let ownership = cw_ownable::get_ownership(deps.as_ref().storage).unwrap();
    assert_eq!(
        ownership,
        Ownership {
            owner: Some(Addr::unchecked("pumpkin")),
            pending_owner: None,
            pending_expiry: None,
        },
    );
}

#[test]
fn writers() {
    let mut deps = mock_dependencies();

    // run instantiation logic
    instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("larry", &[]),
        InstantiateMsg {
            owner: "pumpkin".into(),
            writers: vec![
                NoteWriter {
                    address: String::from("pie"),
                    name: String::from("pie name"),
                },
                NoteWriter {
                    address: String::from("latte"),
                    name: String::from("spiced"),
                },
            ],
        },
    )
    .unwrap();
    let writers = query_writer(deps.as_ref(), None, None).unwrap();
    assert_eq!(writers.entries.len(), 2);
    assert_eq!(
        writers.entries.clone().into_iter().find(|p| p.address == String::from("pie")).unwrap(),
        NoteWriter {
            address: String::from("pie"),
            name: String::from("pie name")
        }
    );
    assert_eq!(
        writers.entries.into_iter().find(|p| p.address == String::from("latte")).unwrap(),
        NoteWriter {
            address: String::from("latte"),
            name: String::from("spiced")
        }
    );
    let writers = query_writer(deps.as_ref(), Some("latte".into()), None).unwrap();
    assert_eq!(writers.entries.len(), 1);
    assert_eq!(
        writers.entries.clone().into_iter().find(|p| p.address == String::from("pie")).unwrap(),
        NoteWriter {
            address: String::from("pie"),
            name: String::from("pie name")
        }
    );
    assert_eq!(writers.entries.into_iter().find(|p| p.address == String::from("latte")), None);

    exec_rm_writer(deps.as_mut(), "pie").unwrap();
    let writers = query_writer(deps.as_ref(), None, None).unwrap();
    assert_eq!(writers.entries.len(), 1);
    assert_eq!(
        writers.entries.clone().into_iter().find(|p| p.address == String::from("pie")),
        None
    );
    exec_add_writer(deps.as_mut(), "bread", "also tastes yuk").unwrap();
    let writers = query_writer(deps.as_ref(), None, None).unwrap();
    assert_eq!(writers.entries.len(), 2);
    assert_eq!(
        writers.entries.clone().into_iter().find(|p| p.address == String::from("pie")),
        None
    );
    assert_eq!(
        writers.entries.clone().into_iter().find(|p| p.address == String::from("bread")),
        Some(NoteWriter {
            address: "bread".into(),
            name: "also tastes yuk".into()
        })
    );
}

#[test]
fn notes() {
    let mut deps = mock_dependencies();

    // run instantiation logic
    instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("larry", &[]),
        InstantiateMsg {
            owner: "pumpkin".into(),
            writers: vec![
                NoteWriter {
                    address: String::from("pie"),
                    name: String::from("pie name"),
                },
                NoteWriter {
                    address: String::from("latte"),
                    name: String::from("spiced"),
                },
            ],
        },
    )
    .unwrap();

    // permissions
    let err = exec_add_note(
        deps.as_mut(),
        &Addr::unchecked("nope"),
        "topic",
        "sub",
        "name",
        NoteType::Ipfs,
        "ipfs://foobar",
    )
    .unwrap_err();
    match err {
        ContractError::Std(std) => match std {
            StdError::NotFound {
                ..
            } => {},
            _ => {
                eprintln!("{:?}", std);
                assert!(false, "wrong error")
            },
        },
        _ => {
            eprintln!("{:?}", err);
            assert!(false, "wrong error")
        },
    };
    let err =
        exec_rm_note(deps.as_mut(), &Addr::unchecked("nope"), "topic", "sub", "name").unwrap_err();
    match err {
        ContractError::Std(std) => match std {
            StdError::NotFound {
                ..
            } => {},
            _ => {
                eprintln!("{:?}", std);
                assert!(false, "wrong error")
            },
        },
        _ => {
            eprintln!("{:?}", err);
            assert!(false, "wrong error")
        },
    }
    // functionality
    exec_add_note(
        deps.as_mut(),
        &Addr::unchecked("pie"),
        "topic",
        "sub",
        "name",
        NoteType::Ipfs,
        "ipfs://foobar",
    )
    .unwrap();
    let note = query_note(deps.as_ref(), "topic", "sub", "name").unwrap();
    assert_eq!(
        note,
        NoteEntry {
            writer: Addr::unchecked("pie"),
            topic: "topic".to_string(),
            sub_topic: "sub".to_string(),
            name: "name".to_string(),
            note_type: NoteType::Ipfs,
            note: "ipfs://foobar".to_string(),
        }
    );
    let err = query_note(deps.as_ref(), "topic", "sub", "Nope").unwrap_err();
    match err {
        StdError::NotFound {
            ..
        } => {},
        _ => {
            eprintln!("{:?}", err);
            assert!(false, "wrong error")
        },
    }

    let err = query_note(deps.as_ref(), "topic", "nope", "name").unwrap_err();
    match err {
        StdError::NotFound {
            ..
        } => {},
        _ => {
            eprintln!("{:?}", err);
            assert!(false, "wrong error")
        },
    }
    exec_add_note(
        deps.as_mut(),
        &Addr::unchecked("pie"),
        "topic",
        "sub2",
        "name",
        NoteType::String,
        "string",
    )
    .unwrap();
    exec_add_note(
        deps.as_mut(),
        &Addr::unchecked("pie"),
        "topic2",
        "sub",
        "name",
        NoteType::Json,
        "{\"foo\":\"bar\"}",
    )
    .unwrap();
    exec_add_note(
        deps.as_mut(),
        &Addr::unchecked("pie"),
        "topic2",
        "sub",
        "name2",
        NoteType::String,
        "A String walks into a bar",
    )
    .unwrap();
    let topics = query_topics(deps.as_ref(), None, None).unwrap();
    assert_eq!(topics.entries.len(), 2);
    let topics = query_topics(deps.as_ref(), Some("topic".into()), None).unwrap();
    assert_eq!(topics.entries.len(), 1);
    let topics = query_topics(deps.as_ref(), Some("topic2".into()), None).unwrap();
    assert_eq!(topics.entries.len(), 0);
    let subs = query_sub_topics(deps.as_ref(), "topic".into(), None, None).unwrap();
    eprintln!("{:?}", subs.entries);
    assert_eq!(subs.entries.len(), 2);
    let subs = query_sub_topics(deps.as_ref(), "topic2".into(), None, None).unwrap();
    assert_eq!(subs.entries.len(), 1);
    let subs = query_sub_topics(deps.as_ref(), "topic".into(), Some("sub".into()), None).unwrap();
    assert_eq!(subs.entries.len(), 1);
}
