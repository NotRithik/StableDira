use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// use cosmwasm_schema::{cw_serde, QueryResponses};

use cosmwasm_std::{Addr, Decimal};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
// #[cw_serde]
pub struct InstantiateMsg {
    pub liquidation_health: Decimal,
    pub mintable_health: Decimal,
    pub collateral_token_denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // All functions related to locking / unlocking collateral tokens with the smart contract
    LockCollateral {},
    UnlockCollateral {
        collateral_amount_to_unlock: Decimal,
    },

    // All functions related to minting / returning rupees
    MintDira {
        dira_to_mint: Decimal,
    },
    RedeemDira {
        dira_to_redeem: Decimal,
    },

    // Liquidate someone's stablecoins if their stablecoin health goes below a certain health
    LiquidateStablecoins {
        liquidate_stablecoin_minter_address: String,
    },

    // Function to set collateral prices from oracles
    SetCollateralPriceInDirham {
        collateral_price_in_aed: Decimal,
    },

    // Function to set liquidation health and mintable health by admins
    SetLiquidationHealth {
        liquidation_health: Decimal,
    },
    SetMintableHealth {
        mintable_health: Decimal,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryLockedCollateral {
        wallet_address_to_query: Addr,
    },

    QueryStablecoinHealth {
        stablecoin_minter_address_to_query: Addr,
    },

    QueryCollateralPrice,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralResponse {
    pub collateral_locked: Decimal,
}
