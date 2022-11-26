use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw20::{Cw20ReceiveMsg, Denom};
pub use cw_controllers::ClaimsResponse;
use cw_utils::Duration;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InstantiateMsg {
    /// denom of the token to stake
    pub denom: Denom,
    pub tokens_per_weight: Uint128,
    pub min_bond: Uint128,
    pub unbonding_period: Duration,

    // admin can only add/remove hooks, not change other parameters
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Bond will bond all staking tokens sent with the message and update membership weight
    Bond {},
    /// Unbond will start the unbonding process for the given number of tokens.
    /// The sender immediately loses weight from these tokens, and can claim them
    /// back to his wallet after `unbonding_period`
    Unbond { tokens: Uint128 },
    /// Claim is used to claim your native tokens that you previously "unbonded"
    /// after the contract-defined waiting period (eg. 1 week)
    Claim {},

    /// Change the admin
    UpdateAdmin { admin: Option<String> },
    /// Add a new hook to be informed of all membership changes. Must be called by Admin
    AddHook { addr: String },
    /// Remove a hook. Must be called by Admin
    RemoveHook { addr: String },

    /// This accepts a properly-encoded ReceiveMsg from a cw20 contract
    Receive(Cw20ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveMsg {
    /// Only valid cw20 message is to bond the tokens
    Bond {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Claims shows the tokens in process of unbonding for this address
    //  struct ClaimsResponse { pub claims: Vec<Claim>,}
    Claims {
        address: String,
    },
    // Show the number of tokens currently staked by this address.      
    // struct StakedResponse { pub stake: Uint128, pub denom: Denom,}
    Staked {
        address: String,
    },

    /// Return AdminResponse            struct AdminResponse {pub admin: Option<String>,}  
    Admin {},
    /// Return TotalWeightResponse      struct TotalWeightResponse {weight: u64,}   
    TotalWeight {},
    /// Returns MembersListResponse     struct MemberListResponse {pub members: Vec<Member>,}
    ListMembers {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Returns MemberResponse          struct MemberResponse { pub weight: Option<u64>,}
    Member {
        addr: String,
        at_height: Option<u64>,
    },
    /// Shows all registered hooks. Returns HooksResponse.  struct HooksResponse { pub hooks: Vec<String>,}
    Hooks {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakedResponse {
    pub stake: Uint128,
    pub denom: Denom,
}
