use cosmwasm_std::{StdError, StdResult, Uint128};
use cw20::{Cw20Coin, Logo, MinterResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use cw20::Cw20ExecuteMsg as ExecuteMsg;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMarketingInfo {
    pub project: Option<String>,
    pub description: Option<String>,
    pub marketing: Option<String>,
    pub logo: Option<Logo>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(Default))]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<MinterResponse>,
    pub marketing: Option<InstantiateMarketingInfo>,
}

impl InstantiateMsg {
    // Remaining minting cap if it exists. {minter, cap} -> {cap}
    pub fn get_cap(&self) -> Option<Uint128> {
        self.mint.as_ref().and_then(|v| v.cap)
    }

    // Check name and symbol
    pub fn validate(&self) -> StdResult<()> {
        // Check name, symbol, decimals
        if !self.has_valid_name() {
            return Err(StdError::generic_err(
                "Name is not in the expected format (3-50 UTF-8 bytes)",
            ));
        }
        if !self.has_valid_symbol() {
            return Err(StdError::generic_err(
                "Ticker symbol is not in expected format [a-zA-Z\\-]{3,12}",
            ));
        }
        if self.decimals > 18 {
            return Err(StdError::generic_err("Decimals must not exceed 18"));
        }
        Ok(())
    }

    fn has_valid_name(&self) -> bool {
        // returns a byte slice, then its length is challenged. Allowed: [3,50]
        let bytes = self.name.as_bytes();
        if bytes.len() < 3 || bytes.len() > 50 {
            return false;
        }
        true
    }

    fn has_valid_symbol(&self) -> bool {
        // returns a byte slice, then we make sure the symbols are allowed. Allowed [3,12]
        let bytes = self.symbol.as_bytes();
        if bytes.len() < 3 || bytes.len() > 12 {
            return false;
        }
        for byte in bytes.iter() {
            if (*byte != 45) && (*byte < 65 || *byte > 90) && (*byte < 97 || *byte > 122) {
                return false;
            }
        }
        true
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns the current balance of the given address, 0 if unset.
    /// Return type: BalanceResponse.   // struct BalanceResponse { balance: Uint128,}
    Balance { address: String },
    /// Returns metadata on the contract - name, decimals, supply, etc.
    /// Return type: TokenInfoResponse.   // struct TokenInfoResponse {name: String, symbol: String, decimals: u8, total_supply: Uint128,}
    TokenInfo {},
    /// Only with "mintable" extension.
    /// Returns who can mint and the hard cap on maximum tokens after minting.
    /// Return type: MinterResponse.      // struct MinterResponse { minter: String, cap: Option<Uint128>,}
    Minter {},
    /// Only with "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    /// Return type: AllowanceResponse.   // struct AllAllowancesResponse {allowances: Vec<AllowanceInfo>,}
    ///                                   // struct AllowanceInfo { spender: String, allowance: Uint128, expires: Expiration,}
    Allowance { owner: String, spender: String },
    /// Only with "enumerable" extension (and "allowances")
    /// Returns all allowances this owner has approved. Supports pagination.
    /// Return type: AllAllowancesResponse.   // struct AllAllowancesResponse {allowances: Vec<AllowanceInfo>,}
    ///                                       // struct AllowanceInfo { spender: String, allowance: Uint128, expires: Expiration,}
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "enumerable" extension (and "allowances")
    /// Returns all allowances this spender has been granted. Supports pagination.
    /// Return type: AllSpenderAllowancesResponse.  // struct AllSpenderAllowancesResponse {allowances: Vec<SpenderAllowanceInfo>,}
    ///                                             // struct SpenderAllowanceInfo {owner: String, allowance: Uint128, expires: Expiration,}
    AllSpenderAllowances {
        spender: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "enumerable" extension
    /// Returns all accounts that have balances. Supports pagination.
    /// Return type: AllAccountsResponse.           // struct AllAccountsResponse {accounts: Vec<String>}
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "marketing" extension
    /// Returns more metadata on the contract to display in the client:
    /// - description, logo, project url, etc.
    /// Return type: MarketingInfoResponse
    MarketingInfo {},
    /// Only with "marketing" extension
    /// Downloads the embedded logo data (if stored on chain). Errors if no logo data is stored for this
    /// contract.
    /// Return type: DownloadLogoResponse.
    DownloadLogo {},
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MigrateMsg {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_instantiatemsg_name() {
        // name length allowed [3,50]
        // Too short
        let mut msg = InstantiateMsg {
            name: str::repeat("a", 2),
            ..InstantiateMsg::default()
        };
        assert!(!msg.has_valid_name());

        // In the correct length range
        msg.name = str::repeat("a", 3);
        assert!(msg.has_valid_name());

        // Too long
        msg.name = str::repeat("a", 51);
        assert!(!msg.has_valid_name());
    }

    #[test]
    fn validate_instantiatemsg_symbol() {
        // symbol lenght Allowed [3,12]
        // Too short
        let mut msg = InstantiateMsg {
            symbol: str::repeat("a", 2),
            ..InstantiateMsg::default()
        };
        assert!(!msg.has_valid_symbol());

        // In the correct length range
        msg.symbol = str::repeat("a", 3);
        assert!(msg.has_valid_symbol());

        // Too long
        msg.symbol = str::repeat("a", 13);
        assert!(!msg.has_valid_symbol());

        // Legal chars [65,90] U [97,122]
        // Has illegal char.
        let illegal_chars = [[64u8], [91u8], [123u8]];
        illegal_chars.iter().for_each(|c| {
            let c = std::str::from_utf8(c).unwrap();
            // the character has to be repeated at least three times to be able to use msg.has_valid_sympol() and not refused by length
            msg.symbol = str::repeat(c, 3);
            assert!(!msg.has_valid_symbol());
        });
    }
}
