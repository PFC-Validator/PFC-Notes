use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, Map, MultiIndex};

use pfc_notes::{NoteEntry, SubTopicKey};

pub(crate) const TOPICSET_KEY: &str = "topic_001";
pub(crate) const WRITERS_KEY: &str = "writers_001";
pub(crate) const SUBTOPICS_KEY: &str = "topic_subtopics_001";
pub(crate) const SUBTOPICS_INDEX_KEY: &str = "topic_subtopics__subtopics";
pub(crate) const NOTE_KEY: &str = "topic_subtopics_notes_001";
pub(crate) const NOTE_INDEX_KEY: &str = "topic_subtopics__note";

//pub(crate) const NOTES_KEY: &str = "notes_001";

pub fn topic_subtopic_idx(_pk: &[u8], d: &SubTopicKey) -> String {
    d.topic.clone()
}

pub struct TopicSubtopicIndexes<'a> {
    pub topic: MultiIndex<'a, String, SubTopicKey, String>,
}

impl<'a> IndexList<SubTopicKey> for TopicSubtopicIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<SubTopicKey>> + '_> {
        let v: Vec<&dyn Index<SubTopicKey>> = vec![&self.topic];
        Box::new(v.into_iter())
    }
}

pub fn topic_subtopics<'a>() -> IndexedMap<'a, String, SubTopicKey, TopicSubtopicIndexes<'a>> {
    IndexedMap::new(
        SUBTOPICS_KEY,
        TopicSubtopicIndexes {
            topic: MultiIndex::new(topic_subtopic_idx, SUBTOPICS_KEY, SUBTOPICS_INDEX_KEY),
        },
    )
}

pub fn note_idx(_pk: &[u8], d: &NoteEntry) -> String {
    format!("{}/{}", d.topic, d.sub_topic)
}

pub struct NoteIndexes<'a> {
    pub subtopic: MultiIndex<'a, String, NoteEntry, String>,
}

impl<'a> IndexList<NoteEntry> for NoteIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<NoteEntry>> + '_> {
        let v: Vec<&dyn Index<NoteEntry>> = vec![&self.subtopic];
        Box::new(v.into_iter())
    }
}

pub fn notes<'a>() -> IndexedMap<'a, String, NoteEntry, NoteIndexes<'a>> {
    IndexedMap::new(
        NOTE_KEY,
        NoteIndexes {
            subtopic: MultiIndex::new(note_idx, NOTE_KEY, NOTE_INDEX_KEY),
        },
    )
}

pub const TOPICSMAP: Map<String, String> = Map::new(TOPICSET_KEY);

// key is actually NoteKey
//pub const NOTES: Map<String, NoteEntry> = Map::new(NOTES_KEY);
pub const WRITER: Map<Addr, String> = Map::new(WRITERS_KEY);
