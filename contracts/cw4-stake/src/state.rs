use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw20::Denom;
use cw4::TOTAL_KEY;
use cw_controllers::{Admin, Claims, Hooks};
use cw_storage_plus::{Item, Map, SnapshotMap, Strategy};
use cw_utils::Duration;

// A Claim allows a given address to claim an amount of tokens after a release date. 
// When a claim is created: an address, amount and expiration are given.
// Claims(Map<&Addr, Vec<Claim>>)      struct Claim {amount: Uint128,release_at: Expiration,}
pub const CLAIMS: Claims = Claims::new("claims");

// Duration is a delta of time.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Config {
    /// denom of the token to stake
    pub denom: Denom,      //enum Denom {Native(String), Cw20(Addr),} 
                           // We can specify a String (coin denom) or an addr (contract address) for the Denomination
    pub tokens_per_weight: Uint128,     // Constant, will not change as we stake/bond new tokens.
    pub min_bond: Uint128,
    pub unbonding_period: Duration,
}

// ADMIN: Item< Option<Addr> >      struct Admin(Item<Option<Addr>>)   
pub const ADMIN: Admin = Admin::new("admin");

// HOOKS: Item< Vec<Addr> >         struct Hooks(Item< Vec<Addr> >)
pub const HOOKS: Hooks = Hooks::new("cw4-hooks");

pub const CONFIG: Item<Config> = Item::new("config");
pub const TOTAL: Item<u64> = Item::new(TOTAL_KEY);

// SnapshotMap => Map that maintains a snapshots of one or more checkpoints. 
// We can query historical data as well as current state. What data is snapshotted depends on the Strategy.
// The Stragegy can be EveryBlock, Never or Selected
// MEMBERS is a ledger where we register weights at a certain height, or remove the address entry for a certain height.
pub const MEMBERS: SnapshotMap<&Addr, u64> = SnapshotMap::new(
    cw4::MEMBERS_KEY,
    cw4::MEMBERS_CHECKPOINTS,
    cw4::MEMBERS_CHANGELOG,
    Strategy::EveryBlock,
);

pub const STAKE: Map<&Addr, Uint128> = Map::new("stake");
