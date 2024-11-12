use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CollateralToken {
    NativeToken,
    CW20Token(String),
    CW721Token(String),
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub struct CollateralTokenAmount {
//     pub collateral_token: CollateralToken,
//     pub collateral_amount: u128,
// }

pub const NATIVE_TOKEN_DENOM: cw_storage_plus::Item<String> =
    cw_storage_plus::Item::new("native-token-name");

pub const ADMIN_ADDRESS: cw_storage_plus::Item<Addr> =
    cw_storage_plus::Item::new("admin-address");


pub const LIQUIDATION_HEALTH: cw_storage_plus::Item<f32> =
    // Admin changeable, below what health of the collateral for the stablecoin can
    // a user's collateral be liquidated?
    cw_storage_plus::Item::new("liquidation-health");


pub const ALLOWED_COLLATERALS: cw_storage_plus::Item<Vec<CollateralToken>> =
    cw_storage_plus::Item::new("allowed-collaterals");
pub const LOCKED_COLLATERALS: cw_storage_plus::Map<Addr, schemars::Map<CollateralToken, u128>> =
    cw_storage_plus::Map::new("locked-collaterals");


pub const MINTED_RUPEES: cw_storage_plus::Map<Addr, Vec<u128>> =
    // Track how many rupees minted by each wallet,
    // stored as number of paiseß
    cw_storage_plus::Map::new("minted-rupees");
pub const COLLATERAL_TOKEN_PRICES: cw_storage_plus::Map<CollateralToken, u128> =
    // Collateral prices in rupees, stored as number of paise per token
    cw_storage_plus::Map::new("collateral-prices");
