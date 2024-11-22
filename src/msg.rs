use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::CollateralToken;

use cosmwasm_schema::{cw_serde, QueryResponses};

use cosmwasm_std::Decimal;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
// #[cw_serde]
pub struct InstantiateMsg {
    pub liquidation_health: Decimal,
    pub allowed_collaterals: Vec<CollateralToken>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // All functions related to locking / unlocking collateral tokens with the smart contract

    LockCollateralTokens { collateral_tokens_to_lock: schemars::Map<CollateralToken, u128> },
    LockCollateralToken { collateral_token_to_lock: CollateralToken, collateral_amount_to_lock: u128 },

    UnlockCollateralTokens { collateral_tokens_to_unlock: schemars::Map<CollateralToken, u128> },
    UnlockCollateralToken { collateral_token_to_unlock: CollateralToken, collateral_amount_to_unlock: u128 },

    // All functions related to minting / returning rupees

    MintRupees { rupees_to_mint: u128 },
    ReturnRupees { rupees_to_return: u128 },

    // Liquidate someone's stablecoins if their stablecoin health goes below a certain health

    LiquidateStablecoins { liquidate_stablecoin_minter_address: String },

    // Function to set collateral prices from oracles

    SetCollateralPricesInRupees { collateral_prices_in_rupees: schemars::Map<CollateralToken, u128> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {

    QueryLockedCollateral { collateral_address_to_query: String },

    QueryStablecoinHealth { stablecoin_minter_address_to_query: String },

    QueryCollateralPrices { collateral_tokens: Option<Vec<CollateralToken>> },

}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralResponse {
    pub collateral_locked: schemars::Map<CollateralToken, u128>,
}