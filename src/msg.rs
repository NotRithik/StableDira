use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Decimal};

/// InstantiateMsg is used for initializing the contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub liquidation_health: Decimal,
    pub mintable_health: Decimal,
    pub collateral_token_denom: String,
    pub cw20_dira_contract_address: Option<Addr>,
}

/// ExecuteMsg contains all the executable contract endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Lock and unlock collateral
    LockCollateral {},
    UnlockCollateral {
        collateral_amount_to_unlock: Decimal,
    },

    // Mint and burn DIRA stablecoin
    MintDira {
        dira_to_mint: Decimal,
    },
    BurnDira {
        dira_to_burn: Decimal,
    },

    // Liquidation
    LiquidateStablecoins {
        wallet_address_to_liquidate: Addr,
    },

    // Admin functionalities
    SetCollateralPriceInDirham {
        collateral_price_in_dirham: Decimal,
    },
    SetLiquidationHealth {
        liquidation_health: Decimal,
    },
    SetMintableHealth {
        mintable_health: Decimal,
    },
    SetCW20DiraContractAddress {
        cw20_dira_contract_address: Addr,
    },
}

/// QueryMsg contains all queryable contract endpoints.
/// These endpoints allow public access to the contract's state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Query the total collateral locked by a specific wallet address.
    QueryLockedCollateral {
        wallet_address_to_query: Addr,
    },

    /// Query the total DIRA stablecoins minted by a specific wallet address.
    QueryMintedDira {
        wallet_address_to_query: Addr,
    },

    /// Query the current health of a stablecoin minter.
    QueryStablecoinHealth {
        stablecoin_minter_address_to_query: Addr,
    },

    /// Query the price of the collateral in dirham.
    QueryCollateralPrice {},

    /// Query the liquidation health threshold.
    QueryLiquidationHealth {},

    /// Query the mintable health threshold.
    QueryMintableHealth {},

    /// Query the list of admin addresses.
    QueryAdminAddresses {},

    /// Query the collateral token denom allowed.
    QueryCollateralTokenDenom {},

    /// Query the CW20 DIRA token contract address.
    QueryCW20DiraContractAddress {},
}

/// Responses for each query

/// Response for querying locked collateral.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralResponse {
    pub collateral_locked: Decimal,
}

/// Response for querying minted DIRA.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintedDiraResponse {
    pub dira_minted: Decimal,
}

/// Response for querying stablecoin health.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StablecoinHealthResponse {
    pub health: Decimal,
}

/// Response for querying the collateral price.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralPriceResponse {
    pub collateral_price: Decimal,
}

/// Response for querying the liquidation health threshold.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LiquidationHealthResponse {
    pub liquidation_health: Decimal,
}

/// Response for querying the mintable health threshold.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintableHealthResponse {
    pub mintable_health: Decimal,
}

/// Response for querying the list of admin addresses.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AdminAddressesResponse {
    pub admin_addresses: Vec<Addr>,
}

/// Response for querying the collateral token denom.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralTokenDenomResponse {
    pub collateral_token_denom: String,
}

/// Response for querying the CW20 DIRA contract address.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CW20DiraContractAddressResponse {
    pub cw20_dira_contract_address: Option<Addr>,
}
