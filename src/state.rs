use cosmwasm_std::{Addr, Decimal,Uint128};
use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

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

// Fee Switch Implementation , in Tier basis
#[cw_serde]
pub enum FeeTier {
    Low,
    Medium,
    High,
}

#[cw_serde]
pub struct FeeConfig {
    pub enabled: bool,
    pub tier: FeeTier,
}


impl FeeTier {
    // Classification of amount on tier basis
    pub fn from_amount(amount: Decimal) -> Self {
        if amount  < Decimal::from_ratio(Uint128::new(1000u128), Uint128::new(1u128)){
            FeeTier::Low
        } else if amount < Decimal::from_ratio(Uint128::new(10000u128), Uint128::new(1u128)) {
            FeeTier::Medium
        } else {
            FeeTier::High
        }
    }

        // Classification of percentage to treasury
        pub fn rate(&self) -> Decimal {
        match self {
            FeeTier::Low => Decimal::permille(3),
            FeeTier::Medium => Decimal :: permille(1_5),
            FeeTier::High => Decimal::permille(0_5),
        }
    }
}


pub const FEE_SWITCH: Item<FeeConfig> = Item::new("fee_switch");

