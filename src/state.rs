use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use std::fmt;

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CollateralToken {
    NativeToken,
    CW20Token(String),
    CW721Token(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CollateralTokenAmount {
    pub collateral_token: CollateralToken,
    pub collateral_amount: u128,
}

pub const ADMIN_ADDRESS: cw_storage_plus::Item<String> =
    cw_storage_plus::Item::new("admin-address");


pub const LIQUIDATION_HEALTH: cw_storage_plus::Item<f32> =
    // Admin changeable, below what health of the collateral for the stablecoin can
    // a user's collateral be liquidated?
    cw_storage_plus::Item::new("liquidation-health");


pub const ALLOWED_COLLATERALS: cw_storage_plus::Item<Vec<CollateralToken>> =
    cw_storage_plus::Item::new("allowed-collaterals");
pub const LOCKED_COLLATERALS: cw_storage_plus::Map<&str, Vec<CollateralTokenAmount>> =
    cw_storage_plus::Map::new("locked-collaterals");


pub const MINTED_RUPEES: cw_storage_plus::Map<&str, Vec<u128>> =
    // Track how many rupees minted by each wallet,
    // stored as number of paiseß
    cw_storage_plus::Map::new("minted-rupees");
pub const COLLATERAL_TOKEN_PRICES: cw_storage_plus::Map<CollateralToken, u128> =
    // Collateral prices in rupees, stored as number of paise per token
    cw_storage_plus::Map::new("collateral-prices");
