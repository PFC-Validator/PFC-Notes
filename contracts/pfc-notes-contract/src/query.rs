use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::Bound;
use pfc_notes::{NoteEntry, NoteKey, NoteResponse, NoteWriter, SubTopicKey};

use crate::state::{notes, topic_subtopics, TOPICSMAP, WRITER};

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 30;

pub(crate) fn query_writer(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<NoteResponse<NoteWriter>> {
    let limit_amt = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = if let Some(start_after_str) = start_after {
        let start_after_addr = deps.api.addr_validate(&start_after_str)?;
        Some(Bound::exclusive(start_after_addr))
    } else {
        None
    };

    Ok(NoteResponse {
        entries: WRITER
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit_amt)
            .map(|item| {
                item.map(|(k, v)| NoteWriter {
                    address: k.to_string(),
                    name: v,
                })
            })
            .collect::<StdResult<Vec<NoteWriter>>>()?,
    })
}

pub(crate) fn query_topics(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<NoteResponse<String>> {
    let limit_amt = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    Ok(NoteResponse {
        entries: TOPICSMAP
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit_amt)
            .map(|item| item.map(|(k, _)| k))
            .collect::<StdResult<Vec<String>>>()?,
    })
}

pub(crate) fn query_sub_topics(
    deps: Deps,
    topic: &str,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<NoteResponse<SubTopicKey>> {
    let limit_amt = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = if let Some(start_sub) = start_after {
        let subtopic_key = SubTopicKey {
            topic: topic.to_string(),
            sub_topic: start_sub,
        };
        Some(Bound::exclusive(subtopic_key.to_string()))
    } else {
        None
    };

    Ok(NoteResponse {
        entries: topic_subtopics()
            .idx
            .topic
            .prefix(topic.into())
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit_amt)
            .map(|item| item.map(|(_, v)| v))
            .collect::<StdResult<Vec<SubTopicKey>>>()?,
    })
}

pub(crate) fn query_entries(
    deps: Deps,
    topic: &str,
    sub_topic: &str,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<NoteResponse<NoteEntry>> {
    let limit_amt = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    //  let start = start_after.map(Bound::exclusive);
    let start = if let Some(start_note) = start_after {
        let note_key = NoteKey {
            topic: topic.to_string(),
            sub_topic: sub_topic.to_string(),
            name: start_note,
        };
        Some(Bound::exclusive(note_key.to_string()))
    } else {
        None
    };
    Ok(NoteResponse {
        entries: notes()
            .idx
            .subtopic
            .prefix(format!("{}_{}", topic, sub_topic))
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit_amt)
            .map(|item| item.map(|(_k, v)| v))
            .collect::<StdResult<Vec<NoteEntry>>>()?,
    })
}

pub(crate) fn query_note(
    deps: Deps,
    topic: &str,
    sub_topic: &str,
    name: &str,
) -> StdResult<NoteEntry> {
    let note_key = NoteKey {
        topic: topic.to_string(),
        sub_topic: sub_topic.to_string(),
        name: name.to_string(),
    };
    notes().load(deps.storage, note_key.to_string())
}
