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

    #[error("Insufficient collateral to unlock")]
    InsufficientCollateral {},

    #[error("Liquidation health or mintable health cannot be zero")]
    HealthCannotBeZero {},

    #[error("Unlock amount too high. Max unlockable tokens: {max_unlockable}")]
    UnlockAmountTooHigh { max_unlockable: Decimal },

    #[error("Collateral Token Denom has not been set")]
    MissingCollateralTokenDenom {},

    #[error("Mintable Health cannot be set to be lower than Liquidation Health")]
    MintableHealthLowerThanLiquidationHealth {},

    #[error("Collateral Price has not been set")]
    CollateralPriceNotSet {},
    
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
