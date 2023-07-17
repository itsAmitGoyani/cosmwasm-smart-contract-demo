use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub node: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetOwner { node: String, owner: String },
    SetResolver { node: String, resolver: String },
    SetTTL { node: String, ttl: u64 },
    SetResolverAndTTL { node: String, resolver: String, ttl: u64 },
    SetSubnodeOwner { node: String, label: String, owner: String },
    SetSubnodeRecord { node: String, label: String, owner: String, resolver: String, ttl: u64 },
    SetRecord { node: String, owner: String, resolver: String, ttl: u64 },
    SetApprovalForAll { operator: String, approved: bool },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AddressResponse)]
    Owner { node: String },
    #[returns(AddressResponse)]
    Resolver { node: String },
    #[returns(TTLResponse)]
    TTL { node: String },
    #[returns(BoolResponse)]
    RecordExists { node: String },
    #[returns(BoolResponse)]
    IsApprovedForAll { owner: String, operator: String },
}

#[cw_serde]
pub struct AddressResponse {
    pub address: Option<String>,
}

#[cw_serde]
pub struct TTLResponse {
    pub ttl: Option<u64>,
}

#[cw_serde]
pub struct BoolResponse {
    pub value: bool,
}