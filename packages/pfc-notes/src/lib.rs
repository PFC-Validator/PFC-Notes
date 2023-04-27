use std::str::FromStr;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw_ownable::{cw_ownable_execute, cw_ownable_query};

#[cw_serde]
pub enum NoteType {
    Json = 1,
    String = 2,
    Ipfs = 3,
}

impl ToString for NoteType {
    fn to_string(&self) -> String {
        match &self {
            NoteType::Ipfs => String::from("IPFS"),
            NoteType::String => String::from("String"),
            NoteType::Json => String::from("Json"),
        }
    }
}

impl FromStr for NoteType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IPFS" => Ok(NoteType::Ipfs),
            "JSON" => Ok(NoteType::Json),
            "String" => Ok(NoteType::String),
            _ => Err(()),
        }
    }
}

#[cw_serde]
pub struct NoteKey {
    pub topic: String,
    pub sub_topic: String,
    pub name: String,
}
impl ToString for NoteKey {
    fn to_string(&self) -> String {
        format!("{}/{}:{}", self.topic, self.sub_topic, self.name)
    }
}
#[cw_serde]
pub struct NoteEntry {
    pub writer: Addr,
    pub topic: String,
    pub sub_topic: String,
    pub name: String,
    pub note_type: NoteType,
    pub note: String,
}

#[cw_serde]
pub struct SubTopicKey {
    pub topic: String,
    pub sub_topic: String,
}
impl ToString for SubTopicKey {
    fn to_string(&self) -> String {
        format!("{}_{}", self.topic, self.sub_topic)
    }
}

#[cw_serde]
pub struct NoteWriter {
    pub address: String,
    pub name: String,
}

#[cw_serde]
pub struct Subtopic {
    pub topic: String,
    pub sub_topic: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    /// The account to be appointed the contract owner
    pub owner: String,
    pub writers: Vec<NoteWriter>,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    AddWriter {
        address: String,
        name: String,
    },
    RemoveWriter {
        address: String,
    },
    AddNote {
        topic: String,
        sub_topic: String,
        name: String,
        note_type: NoteType,
        note: String,
    },
    RemoveNote {
        topic: String,
        sub_topic: String,
        name: String,
    },
    RemoveSubTopic {
        topic: String,
        sub_topic: String,
    },
    RemoveTopic {
        topic: String,
    },
}

#[cw_serde]
pub struct NoteResponse<T> {
    pub entries: Vec<T>,
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(NoteResponse < NoteWriter >)]
    Writers {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(NoteResponse < String >)]
    Topics {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(NoteResponse < Subtopic >)]
    SubTopics {
        topic: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(NoteResponse < NoteKey >)]
    Entries {
        topic: String,
        sub_topic: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(NoteResponse < NoteEntry >)]
    Note {
        topic: String,
        sub_topic: String,
        name: String,
    },
}
