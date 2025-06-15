use cosmwasm_std::{Decimal, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("The user is not an admin authorized to perform this action")]
    UnauthorizedUser {},

    #[error("Insufficient funds sent, or funds sent with the incorrect token")]
    InsufficientFundsSent {},

    #[error("Not enough collateral locked")]
    InsufficientCollateral {},

    #[error("Liquidation health or mintable health cannot be zero")]
    HealthCannotBeZero {},

    #[error("Unlock amount too high. Max unlockable tokens: {max_unlockable}")]
    UnlockAmountTooHigh { max_unlockable: Decimal },

    #[error("Collateral Token Denom has not been set")]
    MissingCollateralTokenDenom {},

    #[error("Mintable Health cannot be set to be lower than Liquidation Health")]
    MintableHealthLowerThanLiquidationHealth {},

    #[error("Collateral price has not been set")]
    CollateralPriceNotSet {},

    #[error("Cannot return more Dira than minted")]
    ReturningMoreDiraThanMinted {},

    #[error("Invalid Wallet Address")]
    InvalidWalletAddress {},

    #[error("Invalid CW20 Contract Address")]
    InvalidCW20ContractAddress {},

    #[error("CW20 Dira Contract Address not set")]
    CW20DiraContractAddressNotSet {},

    #[error("{wallet_address}'s Dira are too healthy to liquidate")]
    TooHealthyToLiquidate{ wallet_address: cosmwasm_std::Addr },

    #[error("Fee Switch is currently Disabled")]
    FeeSwitchDisabled  {} ,

    #[error("No admin addresses are set in the contract.")]
    NoAdminAddressesSet {},

    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
