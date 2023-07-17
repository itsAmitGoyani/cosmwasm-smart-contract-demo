#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Addr};
use serde_json::map;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Record, State, STATE};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set the initial owner of the root node to the contract creator
    let root_node = "0x0".to_string();
    let owner = env.contract.address;
    let resolver = Addr("".to_string());
    let ttl = 0;
    let state = State::default();
    
    STATE.save(deps.storage, &state)?;
    // STATE.records.save(&root_node, &record)?;

    Ok(Response::default())
    // let state = State {
    //     count: msg.count,
    //     owner: info.sender.clone(),
    // };
    // set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // STATE.save(deps.storage, &state)?;
    // 
    // Ok(Response::new()
    //     .add_attribute("method", "instantiate")
    //     .add_attribute("owner", info.sender)
    //     .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetRecord { node, owner, resolver, ttl } => {
            set_owner(node.clone(), owner.clone());
            set_resolver_and_ttl(node.clone(), resolver.clone(), *ttl);
            Ok(Response::default())
        }
        ExecuteMsg::SetSubnodeRecord { node, label, owner, resolver, ttl } => {
            set_subnode_owner(node.clone(), label.clone(), owner.clone());
            let subnode = format!("{}-{}", node, label);
            set_resolver_and_ttl(subnode, resolver.clone(), *ttl);
            Ok(Response::default())
        }
        ExecuteMsg::SetOwner { node, owner } => {
            let caller = _env.message.sender.clone();
            let current_owner = get_owner(node.clone())?;
            if current_owner != caller && !is_approved_for_all(current_owner.clone(), caller.clone()) {
                return Err(ContractError::Unauthorized {});
            }
            set_owner(node.clone(), owner.clone());
            Ok(Response::default())
        }
        ExecuteMsg::SetApprovalForAll { operator, approved } => {
            let owner = _env.message.sender.clone();
            set_approval_for_all(owner, operator.clone(), *approved);
            Ok(Response::default())
        }
    }
    // match msg {
    //     ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
    // }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner { node } => {
            let owner = get_owner(node.clone())?;
            Ok(to_binary(&owner)?)
        }
        QueryMsg::Resolver { node } => {
            let resolver = get_resolver(node.clone())?;
            Ok(to_binary(&resolver)?)
        }
        QueryMsg::TTL { node } => {
            let ttl = get_ttl(node.clone())?;
            Ok(to_binary(&ttl)?)
        }
        QueryMsg::RecordExists { node } => {
            let exists = record_exists(node.clone());
            Ok(to_binary(&exists)?)
        }
        QueryMsg::IsApprovedForAll { owner, operator } => {
            let is_approved = is_approved_for_all(owner.clone(), operator.clone());
            Ok(to_binary(&is_approved)?)
        }
    }
    // match msg {
    //     QueryMsg::GetCount {} => to_binary(&query::count(deps)?),
    // }
}

fn set_owner(node: String, owner: String) {
    let mut record = STATE.records.get(&node).unwrap_or_default();
    record.owner = owner;
    STATE.records.save(&node, &record);
}

fn set_resolver_and_ttl(node: String, resolver: String, ttl: u64) {
    let mut record = STATE.records.get(&node).unwrap_or_default();
    record.resolver = resolver;
    record.ttl = ttl;
    STATE.records.save(&node, &record);
}

fn set_subnode_owner(node: String, label: String, owner: String) {
    let subnode = format!("{}-{}", node, label);
    let mut record = STATE.records.get(&subnode).unwrap_or_default();
    record.owner = owner;
    STATE.records.save(&subnode, &record);
}

fn set_approval_for_all(owner: String, operator: String, approved: bool) {
    STATE.operators.save(&(owner, operator), &approved);
}

fn get_owner(node: String) -> StdResult<String> {
    let record = STATE.records.get(&node).ok_or_else(|| StdError::generic_err("Node not found"))?;
    Ok(record.owner.clone())
}

fn get_resolver(node: String) -> StdResult<String> {
    let record = STATE.records.get(&node).ok_or_else(|| StdError::generic_err("Node not found"))?;
    Ok(record.resolver.clone())
}

fn get_ttl(node: String) -> StdResult<u64> {
    let record = STATE.records.get(&node).ok_or_else(|| StdError::generic_err("Node not found"))?;
    Ok(record.ttl)
}

fn record_exists(node: String) -> bool {
    STATE.records.contains(&node)
}

fn is_approved_for_all(owner: String, operator: String) -> bool {
    operators
        .may_load(&(owner.clone(), operator))
        .unwrap_or_default()
        .unwrap_or_default()
}

// pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
//     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//         state.count += 1;
//         Ok(state)
//     })?;
// 
//     Ok(Response::new().add_attribute("action", "increment"))
// }
// 
// pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
//     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//         if info.sender != state.owner {
//             return Err(ContractError::Unauthorized {});
//         }
//         state.count = count;
//         Ok(state)
//     })?;
//     Ok(Response::new().add_attribute("action", "reset"))
// }

// impl ENSRegistry {
//     pub fn new() -> Self {
//         ENSRegistry {
//             records: map::Map::new("records"),
//             operators: map::Map::new("operators"),
//         }
//     }
// 
// 
//     
// }

// Implement the CosmWasm contract interface for ENSRegistry
// impl Contract for ENSRegistry {
//     fn instantiate(&mut self, _deps: &mut dyn Storage, _env: &Env, _msg: &InstantiateMsg) -> Result<Response, ContractError> {
//         // Set the initial owner of the root node to the contract creator
//         let root_node = "0x0".to_string();
//         let owner = _env.message.sender.clone();
//         let resolver = "".to_string();
//         let ttl = 0;
//         let record = Record { owner, resolver, ttl };
//         self.records.save(&root_node, &record)?;
// 
//         Ok(Response::default())
//     }
// 
//     fn execute(&mut self, _deps: &mut dyn Storage, _env: &Env, _msg: &ExecuteMsg) -> Result<Response, ContractError> {
//         match _msg {
//             ExecuteMsg::SetRecord { node, owner, resolver, ttl } => {
//                 self.set_owner(node.clone(), owner.clone());
//                 self.set_resolver_and_ttl(node.clone(), resolver.clone(), *ttl);
//                 Ok(Response::default())
//             }
//             ExecuteMsg::SetSubnodeRecord { node, label, owner, resolver, ttl } => {
//                 self.set_subnode_owner(node.clone(), label.clone(), owner.clone());
//                 let subnode = format!("{}-{}", node, label);
//                 self.set_resolver_and_ttl(subnode, resolver.clone(), *ttl);
//                 Ok(Response::default())
//             }
//             ExecuteMsg::SetOwner { node, owner } => {
//                 let caller = _env.message.sender.clone();
//                 let current_owner = self.owner(node.clone())?;
//                 if current_owner != caller && !self.is_approved_for_all(current_owner.clone(), caller.clone()) {
//                     return Err(ContractError::Unauthorized {});
//                 }
//                 self.set_owner(node.clone(), owner.clone());
//                 Ok(Response::default())
//             }
//             ExecuteMsg::SetApprovalForAll { operator, approved } => {
//                 let owner = _env.message.sender.clone();
//                 self.set_approval_for_all(owner, operator.clone(), *approved);
//                 Ok(Response::default())
//             }
//         }
//     }
// 
//     fn query(&self, _deps: &dyn Storage, _env: &Env, _msg: &QueryMsg) -> Result<Binary, ContractError> {
//         match _msg {
//             QueryMsg::Owner { node } => {
//                 let owner = self.owner(node.clone())?;
//                 Ok(to_binary(&owner)?)
//             }
//             QueryMsg::Resolver { node } => {
//                 let resolver = self.resolver(node.clone())?;
//                 Ok(to_binary(&resolver)?)
//             }
//             QueryMsg::TTL { node } => {
//                 let ttl = self.ttl(node.clone())?;
//                 Ok(to_binary(&ttl)?)
//             }
//             QueryMsg::RecordExists { node } => {
//                 let exists = self.record_exists(node.clone());
//                 Ok(to_binary(&exists)?)
//             }
//             QueryMsg::IsApprovedForAll { owner, operator } => {
//                 let is_approved = self.is_approved_for_all(owner.clone(), operator.clone());
//                 Ok(to_binary(&is_approved)?)
//             }
//         }
//     }
// }
