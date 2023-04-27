use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use error::ContractError;
use pfc_notes::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::{
    execute::{
        exec_add_note, exec_add_writer, exec_rm_note, exec_rm_sub_topic, exec_rm_topic,
        exec_rm_writer,
    },
    query::{query_entries, query_note, query_sub_topics, query_topics, query_writer},
    state::WRITER,
};

mod error;
mod execute;
mod query;
mod state;

#[cfg(test)]
mod testing;

pub const CONTRACT_NAME: &str = "crates.io:pfc-notes-contract";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&msg.owner))?;
    for writer in msg.writers {
        let addr = deps.api.addr_validate(&writer.address)?;
        WRITER.save(deps.storage, addr, &writer.name)?
    }
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateOwnership(action) => {
            cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
            Ok(Response::default())
        },
        ExecuteMsg::AddWriter {
            address,
            name,
        } => {
            cw_ownable::assert_owner(deps.storage, &info.sender)?;
            exec_add_writer(deps, &address, &name)
        },
        ExecuteMsg::RemoveWriter {
            address,
        } => {
            cw_ownable::assert_owner(deps.storage, &info.sender)?;
            exec_rm_writer(deps, &address)
        },
        ExecuteMsg::AddNote {
            topic,
            sub_topic,
            name,
            note_type,
            note,
        } => exec_add_note(deps, &info.sender, &topic, &sub_topic, &name, note_type, &note),
        ExecuteMsg::RemoveNote {
            topic,
            sub_topic,
            name,
        } => exec_rm_note(deps, &info.sender, &topic, &sub_topic, &name),
        ExecuteMsg::RemoveSubTopic {
            topic,
            sub_topic,
        } => exec_rm_sub_topic(deps, &info.sender, &topic, &sub_topic),
        ExecuteMsg::RemoveTopic {
            topic,
        } => exec_rm_topic(deps, &info.sender, &topic),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Ownership {} => {
            let ownership = cw_ownable::get_ownership(deps.storage)?;
            to_binary(&ownership)
        },
        QueryMsg::Writers {
            start_after,
            limit,
        } => to_binary(&query_writer(deps, start_after, limit)?),
        QueryMsg::Topics {
            start_after,
            limit,
        } => to_binary(&query_topics(deps, start_after, limit)?),
        QueryMsg::SubTopics {
            topic,
            start_after,
            limit,
        } => to_binary(&query_sub_topics(deps, &topic, start_after, limit)?),
        QueryMsg::Entries {
            topic,
            sub_topic,
            start_after,
            limit,
        } => to_binary(&query_entries(deps, &topic, &sub_topic, start_after, limit)?),
        QueryMsg::Note {
            topic,
            sub_topic,
            name,
        } => to_binary(&query_note(deps, &topic, &sub_topic, &name)?),
    }
}
