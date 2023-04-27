use crate::state::{notes, topic_subtopics, TOPICSMAP, WRITER};
use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::Bound;
use pfc_notes::{NoteEntry, NoteKey, NoteResponse, NoteWriter};

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
            .map(|item| item.map(|(k, _)| k.to_string()))
            .collect::<StdResult<Vec<String>>>()?,
    })
}

pub(crate) fn query_sub_topics(
    deps: Deps,
    topic: &str,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<NoteResponse<String>> {
    let limit_amt = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    Ok(NoteResponse {
        entries: topic_subtopics()
            .idx
            .topic
            .prefix(topic.into())
            .keys(deps.storage, start, None, Order::Ascending)
            .take(limit_amt)
            .map(|item| item.map(|k| k.to_string()))
            .collect::<StdResult<Vec<String>>>()?,
    })
}

pub(crate) fn query_entries(
    deps: Deps,
    topic: &str,
    sub_topic: &str,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<NoteResponse<String>> {
    let limit_amt = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    Ok(NoteResponse {
        entries: notes()
            .idx
            .subtopic
            .prefix(format!("{}_{}", topic, sub_topic))
            .keys(deps.storage, start, None, Order::Ascending)
            .take(limit_amt)
            .map(|item| item.map(|k| k.to_string()))
            .collect::<StdResult<Vec<String>>>()?,
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
