use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SetRecord { node: String, owner: String, resolver: String, ttl: u64 },
    SetSubnodeRecord { node: String, label: String, owner: String, resolver: String, ttl: u64 },
    SetOwner { node: String, owner: String },
    SetApprovalForAll { operator: String, approved: bool },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    Owner { node: String },
    Resolver { node: String },
    TTL { node: String },
    RecordExists { node: String },
    IsApprovedForAll { owner: String, operator: String },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}