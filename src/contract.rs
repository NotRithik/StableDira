use core::panic;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Binary, Coin, Decimal, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Response, StdError, StdResult, Uint128, WasmQuery,
};

use cw2::set_contract_version;
use cw20::TokenInfoResponse;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    ADMIN_ADDRESSES, COLLATERAL_TOKEN_DENOM, COLLATERAL_TOKEN_PRICE, CW20_DIRA_CONTRACT_ADDRESS,
    LIQUIDATION_HEALTH, LOCKED_COLLATERAL, MINTABLE_HEALTH, MINTED_DIRA,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmwasm-stable-dira";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/****
 * THIS IS THE SECTION FOR MATCHING EXECUTE AND QUERY MESSAGES
 * FROM msg.rs IN HERE. THE ACTUAL FUNCTION IMPLEMENTATIONS ARE DONE IN THE SECTION
 * WAY BELOW THIS ONE
 ****/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    deps.api.debug("Instantiating contract...");
    deps.api.debug(&format!("Received message: {:?}", msg));

    if msg.liquidation_health.is_zero() || msg.mintable_health.is_zero() {
        return Err(ContractError::HealthCannotBeZero {});
    }

    if msg.collateral_token_denom.is_empty() {
        return Err(ContractError::MissingCollateralTokenDenom {});
    }

    if msg.mintable_health < msg.liquidation_health {
        return Err(ContractError::MintableHealthLowerThanLiquidationHealth {});
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ADMIN_ADDRESSES.save(deps.storage, &vec![info.sender.clone()])?;
    LIQUIDATION_HEALTH.save(deps.storage, &msg.liquidation_health)?;
    MINTABLE_HEALTH.save(deps.storage, &msg.mintable_health)?;
    COLLATERAL_TOKEN_DENOM.save(deps.storage, &msg.collateral_token_denom)?;

    match msg.cw20_dira_contract_address {
        Some(contract_address) => {
            if helper_is_cw20_contract(deps.as_ref(), &contract_address) {
                CW20_DIRA_CONTRACT_ADDRESS.save(deps.storage, &contract_address)?;
            } else {
                return Err(ContractError::InvalidCW20ContractAddress {});
            }
        }
        _ => {}
    }

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    deps.api.debug("Executing function...");
    deps.api.debug(&format!("Received message: {:?}", &msg));

    match msg {
        ExecuteMsg::LockCollateral {} => execute_lock_collateral(deps, info),

        ExecuteMsg::UnlockCollateral {
            collateral_amount_to_unlock,
        } => execute_unlock_collateral(deps, info, collateral_amount_to_unlock),

        ExecuteMsg::MintDira { dira_to_mint } => execute_mint_dira(deps, info, dira_to_mint),
        ExecuteMsg::BurnDira { dira_to_burn } => execute_burn_dira(deps, info, dira_to_burn),

        ExecuteMsg::LiquidateStablecoins {
            wallet_address_to_liquidate,
        } => execute_liquidate_stablecoin_minter(deps, info, wallet_address_to_liquidate),

        ExecuteMsg::SetCollateralPriceInDirham {
            collateral_price_in_dirham,
        } => execute_set_collateral_price_in_dirham(deps, info, collateral_price_in_dirham),

        ExecuteMsg::SetLiquidationHealth { liquidation_health } => {
            execute_set_liquidation_health(deps, info, liquidation_health)
        }

        ExecuteMsg::SetMintableHealth { mintable_health } => {
            execute_set_mintable_health(deps, info, mintable_health)
        }

        ExecuteMsg::SetCW20DiraContractAddress {
            cw20_dira_contract_address,
        } => execute_set_cw20_dira_contact_address(deps, cw20_dira_contract_address),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryCollateralPrice {} => query_collateral_price(&deps),

        QueryMsg::QueryLockedCollateral {
            wallet_address_to_query,
        } => query_locked_collateral(&deps, wallet_address_to_query),

        QueryMsg::QueryStablecoinHealth {
            stablecoin_minter_address_to_query,
        } => query_stablecoin_health(&deps, stablecoin_minter_address_to_query),
    }
}

/****
 * THIS IS THE SECTION FOR ACTUAL IMPLEMENTATIONS OF ALL THE FUNCTIONS USED ABOVE!
 ****/

// Function to calculate stablecoin health of a particular user
// based on how much stablecoin they've minted and how much
// collateral they have locked
fn helper_calculate_stablecoin_health(
    minted_dira: Decimal,
    locked_collateral: Decimal,
    collateral_price_in_dirham: Decimal,
) -> Decimal {
    let locked_collateral_value_in_dirham = collateral_price_in_dirham * locked_collateral;

    if minted_dira.is_zero() {
        // if !locked_collateral_value_in_dirham.is_zero() {
        //     return Decimal::zero();
        // } else {
        return Decimal::MAX;
        // }
    }

    return locked_collateral_value_in_dirham / minted_dira;
}

// Function to calculate how much Dira the user can mint
// based on how much collateral is locked, what the
// value of the collateral is and what the
// mintable health is
fn helper_calculate_max_mintable_dira(
    locked_collateral: Decimal,
    collateral_price_in_dirham: Decimal,
    mintable_health: Decimal,
) -> Decimal {
    let max_mintable_dira = (locked_collateral * collateral_price_in_dirham) / mintable_health;

    max_mintable_dira
}

// Function to calculate how much collateral can be unlocked
// based on how much Dira the user has minted, what the value
// of the collateral is, and what the liquidation health is
fn helper_calculate_max_unlockable_collateral(
    locked_collateral: Decimal,
    collateral_price_in_dirham: Decimal,
    minted_dira: Decimal,
    mintable_health: Decimal,
) -> Decimal {
    let required_collateral_for_minted_dira =
        (minted_dira * mintable_health) / collateral_price_in_dirham;
    let unlockable_collateral = locked_collateral - required_collateral_for_minted_dira;

    unlockable_collateral
}

fn helper_is_cw20_contract(deps: Deps, contract_addr: &Addr) -> bool {
    let query_msg = to_json_binary(&cw20::Cw20QueryMsg::TokenInfo {}).unwrap();
    let query = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: contract_addr.to_string(),
        msg: query_msg,
    });

    match deps.querier.query::<TokenInfoResponse>(&query) {
        Ok(_response) => true, // The contract supports CW20 TokenInfo query
        Err(_) => false,       // Not a CW20 contract
    }
}

// Function to lock collateral
fn execute_lock_collateral(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let collateral_token_denom = COLLATERAL_TOKEN_DENOM
        .load(deps.storage)
        .map_err(|_| ContractError::MissingCollateralTokenDenom {})?;

    let message_sender = info.sender;

    // Check if the user has sent enough funds along with the transaction
    let sent_funds = info
        .funds
        .iter()
        .find(|coin| coin.denom == collateral_token_denom)
        .ok_or(ContractError::InsufficientFundsSent {})
        .unwrap();

    let sent_amount = Decimal::from_atomics(sent_funds.amount, 6).unwrap();

    match LOCKED_COLLATERAL.update(
        deps.storage,
        message_sender.clone(),
        |balance: Option<Decimal>| -> Result<Decimal, ContractError> {
            Ok(balance.unwrap_or_default() + sent_amount)
        },
    ) {
        Ok(_result) => {}
        Err(error) => {
            dbg!("Error in updating LOCKED_COLLATERAL storage item");
            return Err(error);
        }
    };

    // Send the lock collateral messages and return the Ok response
    Ok(Response::new()
        .add_attribute("action", "lock_collateral")
        .add_attribute("sender", message_sender.clone())
        .add_attribute(
            "total_funds_locked_by_user",
            LOCKED_COLLATERAL
                .load(deps.storage, message_sender)
                .unwrap_or_default()
                .to_string(),
        ))
}

// Function to unlock collateral
fn execute_unlock_collateral(
    deps: DepsMut,
    info: MessageInfo,
    collateral_amount: Decimal,
) -> Result<Response, ContractError> {
    let collateral_token_denom = COLLATERAL_TOKEN_DENOM
        .load(deps.storage)
        .map_err(|_| ContractError::MissingCollateralTokenDenom {})?;

    let message_sender = info.sender;

    let locked_collateral = LOCKED_COLLATERAL
        .load(deps.storage, message_sender.clone())
        .unwrap_or_default();

    let minted_dira = MINTED_DIRA
        .load(deps.storage, message_sender.clone())
        .unwrap_or_default();

    let mintable_health = MINTABLE_HEALTH.load(deps.storage)?;

    let collateral_price_in_dirham = COLLATERAL_TOKEN_PRICE
        .may_load(deps.storage)?
        .ok_or(ContractError::CollateralPriceNotSet {})
        .unwrap();

    let max_unlockable_collateral = helper_calculate_max_unlockable_collateral(
        locked_collateral,
        collateral_price_in_dirham,
        minted_dira,
        mintable_health,
    );

    if collateral_amount > max_unlockable_collateral {
        return Err(ContractError::UnlockAmountTooHigh {
            max_unlockable: max_unlockable_collateral,
        });
    }

    LOCKED_COLLATERAL.save(
        deps.storage,
        message_sender.clone(),
        &(locked_collateral - collateral_amount),
    )?;

    let return_collateral_to_user_message = BankMsg::Send {
        to_address: message_sender.to_string(),
        amount: vec![Coin {
            denom: collateral_token_denom,
            amount: collateral_amount.atomics() / Uint128::from(u128::pow(10, 12)),
        }],
    };

    Ok(Response::new()
        .add_message(return_collateral_to_user_message)
        .add_attribute("action", "unlock_collateral")
        .add_attribute("sender", message_sender.clone())
        .add_attribute(
            "total_funds_locked_by_user",
            LOCKED_COLLATERAL
                .load(deps.storage, message_sender)
                .unwrap_or_default()
                .to_string(),
        ))
}

// Function to mint dira
fn execute_mint_dira(
    deps: DepsMut,
    info: MessageInfo,
    dira_to_mint: Decimal,
) -> Result<Response, ContractError> {
    // First calculate how much dira this user can mint based on current collateral price
    // and how much collateral they have locked

    // To do this, first load all the variables from the blockchain
    let collateral_locked_by_user =
        match LOCKED_COLLATERAL.may_load(deps.storage, info.sender.clone()) {
            Ok(Some(locked_collateral)) => locked_collateral,
            _ => return Err(ContractError::InsufficientCollateral {}),
        };

    let previously_minted_dira = match MINTED_DIRA.may_load(deps.storage, info.sender.clone()) {
        Ok(Some(minted_dira)) => minted_dira,
        _ => Decimal::zero(),
    };

    let collateral_price_in_dirham = match COLLATERAL_TOKEN_PRICE.may_load(deps.storage) {
        Ok(Some(collateral_price)) => collateral_price,
        _ => return Err(ContractError::CollateralPriceNotSet {}),
    };

    let mintable_health = MINTABLE_HEALTH.load(deps.storage).unwrap();

    // Finally use the helper function to calculate max mintable dira by this user
    let max_mintable_dira = helper_calculate_max_mintable_dira(
        collateral_locked_by_user,
        collateral_price_in_dirham,
        mintable_health,
    );

    if dira_to_mint + previously_minted_dira > max_mintable_dira {
        return Err(ContractError::InsufficientCollateral {});
    }

    // Else, mint dira and transfer it to user, add that message to the response
    MINTED_DIRA.save(
        deps.storage,
        info.sender.clone(),
        &(dira_to_mint + previously_minted_dira),
    )?;

    // Get the CW20 contract address
    let cw20_dira_contract_address = match CW20_DIRA_CONTRACT_ADDRESS.may_load(deps.storage) {
        Ok(Some(contract_address)) => contract_address,
        _ => return Err(ContractError::CW20DiraContractAddressNotSet {}),
    };

    // Mint CW20 tokens
    let mint_msg = cw20::Cw20ExecuteMsg::Mint {
        recipient: info.sender.to_string(),
        amount: dira_to_mint.atomics() / Uint128::from(u128::pow(10, 12)),
    };

    let mint_cw20_message = cosmwasm_std::WasmMsg::Execute {
        contract_addr: cw20_dira_contract_address.to_string(),
        msg: to_json_binary(&mint_msg)?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(mint_cw20_message)
        .add_attribute("action", "mint_dira")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute(
            "total_dira_minted_by_sender",
            (dira_to_mint + previously_minted_dira).to_string(),
        ))
}

// Function to burn dira for the original collateral
fn execute_burn_dira(
    deps: DepsMut,
    info: MessageInfo,
    dira_to_return: Decimal,
) -> Result<Response, ContractError> {
    let previously_minted_dira = match MINTED_DIRA.may_load(deps.storage, info.sender.clone()) {
        Ok(Some(minted_dira)) => minted_dira,
        _ => Decimal::zero(),
    };

    if dira_to_return > previously_minted_dira {
        return Err(ContractError::ReturningMoreDiraThanMinted {});
    }

    MINTED_DIRA.save(
        deps.storage,
        info.sender.clone(),
        &(previously_minted_dira - dira_to_return),
    )?;

    // Get the CW20 contract address
    let cw20_dira_contract_address = match CW20_DIRA_CONTRACT_ADDRESS.may_load(deps.storage) {
        Ok(Some(contract_address)) => contract_address,
        _ => return Err(ContractError::CW20DiraContractAddressNotSet {}),
    };

    // Burn CW20 tokens
    let burn_msg = cw20::Cw20ExecuteMsg::BurnFrom {
        owner: info.sender.to_string(),
        amount: dira_to_return.atomics() / Uint128::from(u128::pow(10, 12)),
    };

    let burn_cw20_message = cosmwasm_std::WasmMsg::Execute {
        contract_addr: cw20_dira_contract_address.to_string(),
        msg: to_json_binary(&burn_msg)?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(burn_cw20_message)
        .add_attribute("action", "burn_dira")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute(
            "total_dira_remaining_by_sender",
            (previously_minted_dira - dira_to_return).to_string(),
        ))
}

// Function to liquidate stablecoins
fn execute_liquidate_stablecoin_minter(
    deps: DepsMut,
    info: MessageInfo,
    wallet_address_to_liquidate: Addr,
) -> Result<Response, ContractError> {
    // Validate the wallet address
    deps.api
        .addr_validate(wallet_address_to_liquidate.as_str())
        .map_err(|_| ContractError::InvalidWalletAddress {})?;

    // Load relevant data for liquidation
    let dira_minted_by_wallet_to_liquidate = MINTED_DIRA
        .load(deps.storage, wallet_address_to_liquidate.clone())
        .unwrap_or_default();

    let collateral_price_in_dirham = COLLATERAL_TOKEN_PRICE
        .load(deps.storage)
        .map_err(|_| ContractError::CollateralPriceNotSet {})?;

    let collateral_locked_by_user_to_liquidate = LOCKED_COLLATERAL
        .load(deps.storage, wallet_address_to_liquidate.clone())
        .unwrap_or_default();

    let liquidation_health = LIQUIDATION_HEALTH.load(deps.storage)?;

    // Calculate health
    let user_health = helper_calculate_stablecoin_health(
        dira_minted_by_wallet_to_liquidate,
        collateral_locked_by_user_to_liquidate,
        collateral_price_in_dirham,
    );
    
    // Check if the user is liquidatable
    if user_health >= liquidation_health {
        return Err(ContractError::TooHealthyToLiquidate {
            wallet_address: wallet_address_to_liquidate,
        });
    }

    // Liquidate: Reset the collateral to zero
    LOCKED_COLLATERAL.save(
        deps.storage,
        wallet_address_to_liquidate.clone(),
        &Decimal::zero(),
    )?;

    let mut liquidated_dira = Decimal::zero();
    MINTED_DIRA.update(
        deps.storage,
        wallet_address_to_liquidate.clone(),
        |minted_dira| {
            liquidated_dira = minted_dira.unwrap_or_default();
            Ok::<Decimal, StdError>(Decimal::zero())
        },
    )?;

    // Return a successful response
    Ok(Response::new()
        .add_attribute("action", "liquidate_stablecoins")
        .add_attribute("liquidated_wallet", wallet_address_to_liquidate.to_string())
        .add_attribute(
            "liquidated_collateral",
            collateral_locked_by_user_to_liquidate.to_string(),
        )
        .add_attribute("liquidated_dira", liquidated_dira.to_string())
        .add_attribute("initiator", info.sender.to_string())
        .add_attribute("liquidator_reward_paid", "0"))
    // TODO: Update liquidator reward logic here
}

// Function to set collateral prices in dirham
fn execute_set_collateral_price_in_dirham(
    deps: DepsMut,
    info: MessageInfo,
    collateral_price_in_dirham: Decimal,
) -> Result<Response, ContractError> {
    let admins = ADMIN_ADDRESSES.load(deps.storage)?;

    if !admins.contains(&info.sender) {
        return Err(ContractError::UnauthorizedUser {});
    }

    match COLLATERAL_TOKEN_PRICE.save(deps.storage, &collateral_price_in_dirham) {
        Ok(_result) => {}
        Err(error) => {
            dbg!(&error);
            panic!("Error in updating COLLATERAL_TOKEN_PRICE storage item");
        }
    }

    Ok(Response::new()
        .add_attribute("action", "set_collateral_price_in_dirham")
        .add_attribute("sender", info.sender)
        .add_attribute(
            "new_collateral_price",
            collateral_price_in_dirham.to_string(),
        ))
}

// Function to set liquidation health
fn execute_set_liquidation_health(
    deps: DepsMut,
    info: MessageInfo,
    liquidation_health: Decimal,
) -> Result<Response, ContractError> {
    let admins = ADMIN_ADDRESSES.load(deps.storage)?;

    if !admins.contains(&info.sender) {
        return Err(ContractError::UnauthorizedUser {});
    }

    LIQUIDATION_HEALTH.save(deps.storage, &liquidation_health)?;

    Ok(Response::new()
        .add_attribute("action", "set_liquidation_health")
        .add_attribute("sender", info.sender)
        .add_attribute("new_liquidation_health", liquidation_health.to_string()))
}

// Function to set mintable health
fn execute_set_mintable_health(
    deps: DepsMut,
    info: MessageInfo,
    mintable_health: Decimal,
) -> Result<Response, ContractError> {
    let admins = ADMIN_ADDRESSES.load(deps.storage)?;

    if !admins.contains(&info.sender) {
        return Err(ContractError::UnauthorizedUser {});
    }

    let current_liquidation_health = LIQUIDATION_HEALTH.load(deps.storage)?;

    MINTABLE_HEALTH.update(
        deps.storage,
        |_current_mintable_health| -> Result<Decimal, ContractError> {
            if mintable_health < current_liquidation_health {
                return Err(ContractError::MintableHealthLowerThanLiquidationHealth {});
            } else {
                return Ok(mintable_health);
            }
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "set_mintable_health")
        .add_attribute("sender", info.sender)
        .add_attribute("new_liquidation_health", mintable_health.to_string()))
}

fn execute_set_cw20_dira_contact_address(
    deps: DepsMut,
    cw20_dira_contract_address: Addr,
) -> Result<Response, ContractError> {
    if helper_is_cw20_contract(deps.as_ref(), &cw20_dira_contract_address) {
        CW20_DIRA_CONTRACT_ADDRESS.save(deps.storage, &cw20_dira_contract_address)?;
        return Ok(Response::new()
            .add_attribute("action", "set_cw20_dira_contract_address")
            .add_attribute("contract_address", cw20_dira_contract_address.into_string()));
    } else {
        return Err(ContractError::InvalidCW20ContractAddress {});
    }
}

// Query function to get collateral prices
fn query_collateral_price(deps: &Deps) -> StdResult<Binary> {
    panic!("TODO: Implement this function!");
}

// Query function to get locked collateral
fn query_locked_collateral(deps: &Deps, collateral_address_to_query: Addr) -> StdResult<Binary> {
    panic!("TODO: Implement this function!");
}

// Query function to get stablecoin health
fn query_stablecoin_health(
    deps: &Deps,
    stablecoin_minter_address_to_query: Addr,
) -> StdResult<Binary> {
    panic!("TODO: Implement this function!");
}
