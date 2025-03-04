use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    ClaimStreak {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetStreak returns the current streak for the given address
    #[returns(GetStreakResponse)]
    GetStreak { address: String },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetStreakResponse {
    pub streak: u32,
}
