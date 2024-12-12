#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult, Uint128, WasmMsg, WasmQuery,
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    ADMIN_ADDRESS, COLLATERAL_TOKEN_DENOM, COLLATERAL_TOKEN_PRICE, LIQUIDATION_HEALTH,
    LOCKED_COLLATERAL, MINTED_DIRA,
};

use cw20::{Cw20ExecuteMsg, Cw20QueryMsg};
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmwasm-stable-rupee";
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
    // liquidation_health: f32,
    // allowed_collaterals: Vec<CollateralToken>,
) -> Result<Response, ContractError> {
    deps.api.debug("Instantiating contract...");
    deps.api.debug(&format!("Received message: {:?}", msg));

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION);

    ADMIN_ADDRESS.save(deps.storage, &info.sender);
    LIQUIDATION_HEALTH.save(deps.storage, &msg.liquidation_health);
    COLLATERAL_TOKEN_DENOM.save(deps.storage, &msg.collateral_token_denom);

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    deps.api.debug("Executing function...");
    deps.api.debug(&format!("Received message: {:?}", &msg));

    match msg {
        ExecuteMsg::LockCollateral {
            collateral_amount_to_lock,
        } => execute_lock_collateral(deps, info, env, collateral_amount_to_lock),

        ExecuteMsg::UnlockCollateral {
            collateral_amount_to_unlock,
        } => execute_unlock_collateral(deps, info, env, collateral_amount_to_unlock),

        ExecuteMsg::MintDira { dira_to_mint } => {
            execute_mint_dira(deps, info.sender.into_string(), dira_to_mint)
        }
        ExecuteMsg::RedeemDira { dira_to_redeem } => {
            execute_return_dira(deps, info.sender.into_string(), dira_to_redeem)
        }

        ExecuteMsg::LiquidateStablecoins {
            liquidate_stablecoin_minter_address,
        } => execute_liquidate_stablecoin_minter(
            deps,
            info.sender.into_string(),
            liquidate_stablecoin_minter_address,
        ),

        ExecuteMsg::SetCollateralPricesInDirham {
            collateral_price_in_aed,
        } => execute_set_collateral_prices_in_dirham(
            deps,
            info.sender.into_string(),
            collateral_price_in_aed,
        ),
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

// Function to lock collateral
fn execute_lock_collateral(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collateral_amount: Decimal,
) -> Result<Response, ContractError> {
    let collateral_token_denom = COLLATERAL_TOKEN_DENOM
        .load(deps.storage)
        .unwrap_or(String::from("uatom"));
    let message_sender = info.sender;

    // Check if the user has sent enough funds along with the transaction
    let sent_funds = info
        .funds
        .iter()
        .find(|coin| coin.denom == collateral_token_denom)
        .ok_or(ContractError::InsufficientFundsSent {})
        .unwrap();

    let sent_amount = Decimal::from_ratio(sent_funds.amount, Uint128::new(1));

    if sent_amount < collateral_amount {
        return Err(ContractError::InsufficientFundsSent {});
    }

    LOCKED_COLLATERAL.update(
        deps.storage,
        message_sender.clone(),
        |balance: Option<Decimal>| -> Result<_, ContractError> {
            Ok(balance.unwrap_or_default() + collateral_amount)
        },
    )?;

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
    env: Env,
    collateral_amount: Decimal,
) -> Result<Response, ContractError> {
    let collateral_token_denom = COLLATERAL_TOKEN_DENOM.load(deps.storage).unwrap_or(String::from("uatom"));
    let message_sender = info.sender;

    LOCKED_COLLATERAL.update(
        deps.storage,
        message_sender.clone(),
        |balance: Option<Decimal>| -> Result<_, ContractError> {
            match balance {
                Some(bal) => {
                    if bal < collateral_amount {
                        return Err(ContractError::InsufficientCollateral {});
                    }
                    Ok(bal - collateral_amount)
                }
                None => Err(ContractError::InsufficientCollateral {}),
            }
        },
    )?;

    let unlock_collateral_message = BankMsg::Send {
        to_address: message_sender.to_string(),
        amount: vec![Coin {
            denom: collateral_token_denom,
            amount: collateral_amount * Uint128::new(1), // Assuming decimal representation
        }],
    };

    Ok(Response::new()
        .add_message(CosmosMsg::Bank(unlock_collateral_message))
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

// Function to mint rupees
fn execute_mint_dira(
    deps: DepsMut,
    sender: String,
    dira_to_mint: Decimal,
) -> Result<Response, ContractError> {
    panic!("TODO: Implement this function!");
}

// Function to return rupees
fn execute_return_dira(
    deps: DepsMut,
    sender: String,
    dira_to_return: Decimal,
) -> Result<Response, ContractError> {
    panic!("TODO: Implement this function!");
}

// Function to liquidate stablecoins
fn execute_liquidate_stablecoin_minter(
    deps: DepsMut,
    sender: String,
    liquidate_stablecoin_minter_address: String,
) -> Result<Response, ContractError> {
    panic!("TODO: Implement this function!");
}

// Function to set collateral prices in rupees
fn execute_set_collateral_prices_in_dirham(
    deps: DepsMut,
    sender: String,
    collateral_prices_in_rupees: Decimal,
) -> Result<Response, ContractError> {
    panic!("TODO: Implement this function!");
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
