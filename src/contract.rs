use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Record, OPERATORS, RECORDS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use crypto::sha3::Sha3;
use cw2::set_contract_version;
use std::collections::HashMap;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-sei";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let record = Record {
        owner: info.sender.to_owned(),
        resolver: Addr::unchecked("".to_string()),
        ttl: 0,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let rec = HashMap::from([(msg.node, record)]);
    RECORDS.save(deps.storage, &rec)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetOwner { node, owner } => {
            execute::set_owner(deps, info, node, Addr::unchecked(owner))
        }
        ExecuteMsg::SetResolver { node, resolver } => {
            execute::set_resolver(deps, info, node, Addr::unchecked(resolver))
        }
        ExecuteMsg::SetTTL { node, ttl } => execute::set_ttl(deps, info, node, ttl),
        ExecuteMsg::SetResolverAndTTL {
            node,
            resolver,
            ttl,
        } => execute::set_resolver_and_ttl(deps, node, Addr::unchecked(resolver), ttl),
        ExecuteMsg::SetSubnodeOwner { node, label, owner } => {
            execute::set_subnode_owner(deps, info, node, label, Addr::unchecked(owner))
        }
        ExecuteMsg::SetSubnodeRecord {
            node,
            label,
            owner,
            resolver,
            ttl,
        } => execute::set_subnode_record(
            deps,
            node,
            label,
            Record {
                owner: Addr::unchecked(owner),
                resolver: Addr::unchecked(resolver),
                ttl,
            },
        ),
        ExecuteMsg::SetRecord {
            node,
            owner,
            resolver,
            ttl,
        } => execute::set_record(
            deps,
            node,
            Record {
                owner: Addr::unchecked(owner),
                resolver: Addr::unchecked(resolver),
                ttl,
            },
        ),
        ExecuteMsg::SetApprovalForAll { operator, approved } => {
            execute::set_approval_for_all(deps, info.sender, Addr::unchecked(operator), approved)
        }
    }
}

pub mod execute {
    use super::*;
    use cosmwasm_std::Storage;
    use crypto::digest::Digest;

    fn authorise(
        storage: &mut dyn Storage,
        info: MessageInfo,
        node: String,
    ) -> Result<(), ContractError> {
        let records = RECORDS.load(storage)?;
        match records.get(&node) {
            None => Err(ContractError::Unauthorized {}),
            Some(rec) => {
                if rec.owner != info.sender {
                    let operators = OPERATORS.load(storage)?;
                    match operators.get(&rec.owner) {
                        None => Err(ContractError::Unauthorized {}),
                        Some(rec) => match rec.get(&info.sender) {
                            None => Err(ContractError::Unauthorized {}),
                            Some(approved) => {
                                if !approved {
                                    Ok(())
                                } else {
                                    Err(ContractError::Unauthorized {})
                                }
                            }
                        },
                    }
                } else {
                    Ok(())
                }
            }
        }
    }

    pub fn set_owner(
        deps: DepsMut,
        info: MessageInfo,
        node: String,
        owner: Addr,
    ) -> Result<Response, ContractError> {
        authorise(deps.storage, info, node.to_owned())?;
        RECORDS.update(deps.storage, |mut record| -> Result<_, ContractError> {
            match record.get_mut(&node) {
                None => {
                    return Err(ContractError::Custom {
                        e: "Invalid node".to_string(),
                    });
                }
                Some(rec) => {
                    rec.owner = owner;
                }
            };
            Ok(record)
        })?;
        Ok(Response::new()
            .add_attribute("method", "execute")
            .add_attribute("action", "set owner"))
    }

    pub fn set_resolver(
        deps: DepsMut,
        info: MessageInfo,
        node: String,
        resolver: Addr,
    ) -> Result<Response, ContractError> {
        authorise(deps.storage, info, node.to_owned())?;
        RECORDS.update(deps.storage, |mut record| -> Result<_, ContractError> {
            match record.get_mut(&node) {
                None => {
                    return Err(ContractError::Custom {
                        e: "Invalid node".to_string(),
                    });
                }
                Some(rec) => {
                    rec.resolver = resolver;
                }
            };
            Ok(record)
        })?;
        Ok(Response::new()
            .add_attribute("method", "execute")
            .add_attribute("action", "set resolver"))
    }

    pub fn set_ttl(
        deps: DepsMut,
        info: MessageInfo,
        node: String,
        ttl: u64,
    ) -> Result<Response, ContractError> {
        authorise(deps.storage, info, node.to_owned())?;
        RECORDS.update(deps.storage, |mut record| -> Result<_, ContractError> {
            match record.get_mut(&node) {
                None => {
                    return Err(ContractError::Custom {
                        e: "Invalid node".to_string(),
                    });
                }
                Some(rec) => {
                    rec.ttl = ttl;
                }
            };
            Ok(record)
        })?;
        Ok(Response::new()
            .add_attribute("method", "execute")
            .add_attribute("action", "set TTL"))
    }

    pub fn set_resolver_and_ttl(
        deps: DepsMut,
        node: String,
        resolver: Addr,
        ttl: u64,
    ) -> Result<Response, ContractError> {
        RECORDS.update(deps.storage, |mut record| -> Result<_, ContractError> {
            match record.get_mut(&node) {
                None => {
                    return Err(ContractError::Custom {
                        e: "Invalid node".to_string(),
                    });
                }
                Some(rec) => {
                    rec.resolver = resolver;
                    rec.ttl = ttl;
                }
            };
            Ok(record)
        })?;
        Ok(Response::new()
            .add_attribute("method", "execute")
            .add_attribute("action", "set resolver and TTL"))
    }

    pub fn set_subnode_owner(
        deps: DepsMut,
        info: MessageInfo,
        node: String,
        label: String,
        owner: Addr,
    ) -> Result<Response, ContractError> {
        authorise(deps.storage, info, node.to_owned())?;
        RECORDS.update(deps.storage, |mut records| -> Result<_, ContractError> {
            let mut hasher = Sha3::keccak256();
            hasher.input_str(format!("{}{}", node, label).as_str());
            let subnode = hasher.result_str();
            let record = Record {
                owner: owner.to_owned(),
                resolver: Addr::unchecked("".to_string()),
                ttl: 0,
            };
            records.insert(subnode.to_owned(), record);
            Ok(records)
        })?;
        Ok(Response::new()
            .add_attribute("method", "execute")
            .add_attribute("action", "set subnode owner"))
    }

    pub fn set_subnode_record(
        deps: DepsMut,
        node: String,
        label: String,
        record: Record,
    ) -> Result<Response, ContractError> {
        RECORDS.update(deps.storage, |mut records| -> Result<_, ContractError> {
            let mut hasher = Sha3::keccak256();
            hasher.input_str(format!("{}{}", node, label).as_str());
            let subnode = hasher.result_str();
            records.insert(subnode.to_owned(), record);
            Ok(records)
        })?;
        Ok(Response::new()
            .add_attribute("method", "execute")
            .add_attribute("action", "set subnode owner"))
    }

    pub fn set_record(
        deps: DepsMut,
        node: String,
        record: Record,
    ) -> Result<Response, ContractError> {
        RECORDS.update(deps.storage, |mut records| -> Result<_, ContractError> {
            match records.get_mut(&node) {
                None => {
                    return Err(ContractError::Custom {
                        e: "Invalid node".to_string(),
                    });
                }
                Some(rec) => {
                    rec.owner = record.owner;
                    rec.resolver = record.resolver;
                    rec.ttl = record.ttl;
                }
            };
            Ok(records)
        })?;
        Ok(Response::new()
            .add_attribute("method", "execute")
            .add_attribute("action", "set record"))
    }

    pub fn set_approval_for_all(
        deps: DepsMut,
        sender: Addr,
        operator: Addr,
        approved: bool,
    ) -> Result<Response, ContractError> {
        OPERATORS.update(deps.storage, |mut operators| -> Result<_, ContractError> {
            match operators.get_mut(&sender) {
                None => {
                    return Err(ContractError::Unauthorized {});
                }
                Some(rec) => {
                    match rec.get(&operator) {
                        None => {
                            return Err(ContractError::Custom {
                                e: "Invalid action. Operator not found".to_string(),
                            });
                        }
                        Some(_) => {
                            rec.insert(operator, approved);
                        }
                    };
                }
            };
            Ok(operators)
        })?;
        Ok(Response::new()
            .add_attribute("method", "execute")
            .add_attribute("action", "set approval for all"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner { node } => to_binary(&query::owner(deps, node)?),
        QueryMsg::Resolver { node } => to_binary(&query::resolver(deps, node)?),
        QueryMsg::TTL { node } => to_binary(&query::ttl(deps, node)?),
        QueryMsg::RecordExists { node } => to_binary(&query::record_exists(deps, node)?),
        QueryMsg::IsApprovedForAll { owner, operator } => to_binary(&query::is_approved_for_all(
            deps,
            Addr::unchecked(owner),
            Addr::unchecked(operator),
        )?),
    }
}

pub mod query {
    use super::*;
    use crate::msg::{AddressResponse, BoolResponse, TTLResponse};

    pub fn owner(deps: Deps, node: String) -> StdResult<AddressResponse> {
        let records = RECORDS.load(deps.storage)?;
        match records.get(&node) {
            None => Ok(AddressResponse { address: None }),
            Some(rec) => Ok(AddressResponse {
                address: Some(rec.owner.to_owned().to_string()),
            }),
        }
    }

    pub fn resolver(deps: Deps, node: String) -> StdResult<AddressResponse> {
        let records = RECORDS.load(deps.storage)?;
        match records.get(&node) {
            None => Ok(AddressResponse { address: None }),
            Some(rec) => Ok(AddressResponse {
                address: Some(rec.resolver.to_owned().to_string()),
            }),
        }
    }

    pub fn ttl(deps: Deps, node: String) -> StdResult<TTLResponse> {
        let records = RECORDS.load(deps.storage)?;
        match records.get(&node) {
            None => Ok(TTLResponse { ttl: None }),
            Some(rec) => Ok(TTLResponse {
                ttl: Some(rec.ttl.to_owned()),
            }),
        }
    }

    pub fn record_exists(deps: Deps, node: String) -> StdResult<BoolResponse> {
        let records = RECORDS.load(deps.storage)?;
        match records.get(&node) {
            None => Ok(BoolResponse { value: false }),
            Some(_rec) => Ok(BoolResponse { value: true }),
        }
    }

    pub fn is_approved_for_all(deps: Deps, owner: Addr, operator: Addr) -> StdResult<BoolResponse> {
        let operators = OPERATORS.load(deps.storage)?;
        match operators.get(&owner) {
            None => Ok(BoolResponse { value: false }),
            Some(rec) => match rec.get(&operator) {
                None => Ok(BoolResponse { value: false }),
                Some(val) => Ok(BoolResponse {
                    value: val.to_owned(),
                }),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { node: "0000".to_string() };
        let info = mock_info("creator", &[]);

        // // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // // it worked, let's query the state
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: GetCountResponse = from_binary(&res).unwrap();
        // assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        // let mut deps = mock_dependencies();

        // let msg = InstantiateMsg {};
        // let info = mock_info("creator", &coins(2, "token"));
        // let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // // beneficiary can release it
        // let info = mock_info("anyone", &coins(2, "token"));
        // let msg = ExecuteMsg::Increment {};
        // let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // // should increase counter by 1
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: GetCountResponse = from_binary(&res).unwrap();
        // assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        // let mut deps = mock_dependencies();

        // let msg = InstantiateMsg {};
        // let info = mock_info("creator", &coins(2, "token"));
        // let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // // beneficiary can release it
        // let unauth_info = mock_info("anyone", &coins(2, "token"));
        // let msg = ExecuteMsg::Reset { count: 5 };
        // let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        // match res {
        //     Err(ContractError::Unauthorized {}) => {}
        //     _ => panic!("Must return unauthorized error"),
        // }

        // // only the original creator can reset the counter
        // let auth_info = mock_info("creator", &coins(2, "token"));
        // let msg = ExecuteMsg::Reset { count: 5 };
        // let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // // should now be 5
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: GetCountResponse = from_binary(&res).unwrap();
        // assert_eq!(5, value.count);
    }
}
