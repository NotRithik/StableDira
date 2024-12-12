use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal};

pub const NATIVE_TOKEN_DENOM: cw_storage_plus::Item<String> =
    cw_storage_plus::Item::new("native-token-name");

pub const ADMIN_ADDRESS: cw_storage_plus::Item<Addr> =
    cw_storage_plus::Item::new("admin-address");


pub const LIQUIDATION_HEALTH: cw_storage_plus::Item<Decimal> =
    // Admin changeable, below what health of the collateral for the stablecoin can
    // a user's collateral be liquidated?
    cw_storage_plus::Item::new("liquidation-health");

pub const LOCKED_COLLATERAL: cw_storage_plus::Map<Addr, Decimal> =
    // Track collateral locked by each wallet
    cw_storage_plus::Map::new("locked-collaterals");


pub const MINTED_DIRA: cw_storage_plus::Map<Addr, Decimal> =
    // Track dira minted by each wallet
    cw_storage_plus::Map::new("minted-dira");

pub const COLLATERAL_TOKEN_PRICE: cw_storage_plus::Item<Decimal> =
    // Collateral prices in dirham
    cw_storage_plus::Item::new("collateral-price");