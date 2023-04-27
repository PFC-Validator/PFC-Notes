use cosmwasm_std::{Addr, DepsMut, Response, Storage};

use pfc_notes::{NoteEntry, NoteKey, NoteType, SubTopicKey};

use crate::error::ContractError;
use crate::state::{notes, topic_subtopics, TOPICSMAP, WRITER};

pub fn assert_is_writer(store: &dyn Storage, addr: &Addr) -> Result<(), ContractError> {
    WRITER.load(store, addr.clone())?;
    Ok(())
}
pub fn exec_add_note(
    deps: DepsMut,
    sender: &Addr,
    topic: &str,
    sub_topic: &str,
    name: &str,
    note_type: NoteType,
    note: &str,
) -> Result<Response, ContractError> {
    assert_is_writer(deps.storage, sender)?;
    let subtopic_key = SubTopicKey {
        topic: topic.to_string(),
        sub_topic: sub_topic.to_string(),
    };
    let note_key = NoteKey {
        topic: topic.to_string(),
        sub_topic: sub_topic.to_string(),
        name: name.to_string(),
    };
    let note_entry = NoteEntry {
        topic: topic.to_string(),
        sub_topic: sub_topic.to_string(),
        name: name.to_string(),
        writer: sender.clone(),
        note_type,
        note: note.to_string(),
    };
    TOPICSMAP.save(deps.storage, topic.to_string(), &topic.to_string())?;
    topic_subtopics().save(deps.storage, subtopic_key.to_string(), &subtopic_key)?;
    notes().save(deps.storage, note_key.to_string(), &note_entry)?;
    Ok(Response::default())
}

pub fn exec_rm_note(
    deps: DepsMut,
    sender: &Addr,
    topic: &str,
    sub_topic: &str,
    name: &str,
) -> Result<Response, ContractError> {
    assert_is_writer(deps.storage, sender)?;

    let note_key = NoteKey {
        topic: topic.to_string(),
        sub_topic: sub_topic.to_string(),
        name: name.to_string(),
    };
    notes().load(deps.storage, note_key.to_string())?;
    // it exists.. now delete the stuff
    notes().remove(deps.storage, note_key.to_string())?;

    Ok(Response::default())
}
pub fn exec_rm_sub_topic(
    deps: DepsMut,
    sender: &Addr,
    topic: &str,
    sub_topic: &str,
) -> Result<Response, ContractError> {
    assert_is_writer(deps.storage, sender)?;

    let subtopic_key = SubTopicKey {
        topic: topic.to_string(),
        sub_topic: sub_topic.to_string(),
    };

    topic_subtopics().remove(deps.storage, subtopic_key.to_string())?;

    Ok(Response::default())
}
pub fn exec_rm_topic(deps: DepsMut, sender: &Addr, topic: &str) -> Result<Response, ContractError> {
    assert_is_writer(deps.storage, sender)?;

    TOPICSMAP.remove(deps.storage, topic.to_string());
    Ok(Response::default())
}

pub fn exec_add_writer(
    deps: DepsMut,
    address: &str,
    note: &str,
) -> Result<Response, ContractError> {
    let addr = deps.api.addr_validate(address)?;
    WRITER.save(deps.storage, addr, &note.to_string())?;

    Ok(Response::default())
}

pub fn exec_rm_writer(deps: DepsMut, address: &str) -> Result<Response, ContractError> {
    let addr = deps.api.addr_validate(address)?;

    WRITER
        .may_load(deps.storage, addr.clone())
        .map(|r| r.map(ContractError::AddressDoesNotExist))?;
    WRITER.remove(deps.storage, addr);
    Ok(Response::default())
}
