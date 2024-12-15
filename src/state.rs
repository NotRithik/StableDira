use cosmwasm_std::{Addr, Decimal};

// What token is allowed to be used as collateral for Dira
pub const COLLATERAL_TOKEN_DENOM: cw_storage_plus::Item<String> =
    cw_storage_plus::Item::new("native-token-name");

// List of admin addresses that are allowed to change parameters of the contract
pub const ADMIN_ADDRESSES: cw_storage_plus::Item<Vec<Addr>> =
    cw_storage_plus::Item::new("admin-addresses");

// Admin changeable, below what health of the collateral for the stablecoin can
// a user's collateral be liquidated?
pub const LIQUIDATION_HEALTH: cw_storage_plus::Item<Decimal> =
    cw_storage_plus::Item::new("liquidation-health");

// Admin changeable, what is the lowest health at which a user can mint stablecoins
// has to be higher than the liquidation health
pub const MINTABLE_HEALTH: cw_storage_plus::Item<Decimal> = 
    cw_storage_plus::Item::new("mintable-health");

// Track collateral locked by each wallet
pub const LOCKED_COLLATERAL: cw_storage_plus::Map<Addr, Decimal> =
    cw_storage_plus::Map::new("locked-collaterals");


// Track dira minted by each wallet
pub const MINTED_DIRA: cw_storage_plus::Map<Addr, Decimal> =
    cw_storage_plus::Map::new("minted-dira");

// Collateral prices in dirham
pub const COLLATERAL_TOKEN_PRICE: cw_storage_plus::Item<Decimal> =
    cw_storage_plus::Item::new("collateral-price");

// Contract address of the cw20 Dira token
pub const CW20_DIRA_CONTRACT_ADDRESS: cw_storage_plus::Item<Addr> =
    cw_storage_plus::Item::new("cw20-dira-contract-address");