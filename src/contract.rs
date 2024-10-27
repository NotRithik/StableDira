use std::intrinsics::mir::PtrMetadata;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    WasmMsg, WasmQuery,
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    CollateralToken, ADMIN_ADDRESS, ALLOWED_COLLATERALS, COLLATERAL_TOKEN_PRICES,
    LIQUIDATION_HEALTH, LOCKED_COLLATERALS, MINTED_RUPEES, NATIVE_TOKEN_DENOM,
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
    _msg: InstantiateMsg,
    liquidation_health: f32,
    allowed_collaterals: Vec<CollateralToken>,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin_address_string: String = String::from(info.sender.as_str());

    ADMIN_ADDRESS.save(deps.storage, &admin_address_string);
    LIQUIDATION_HEALTH.save(deps.storage, &liquidation_health);
    ALLOWED_COLLATERALS.save(deps.storage, &allowed_collaterals);

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
    match msg {
        ExecuteMsg::LockCollateralToken {
            collateral_token_to_lock,
            collateral_amount_to_lock,
        } => execute_lock_collateral(
            &mut deps,
            info,
            env,
            schemars::Map::from([(collateral_token_to_lock, collateral_amount_to_lock)]),
        ),
        ExecuteMsg::LockCollateralTokens {
            collateral_tokens_to_lock,
        } => execute_lock_collateral(&mut deps, info, env, collateral_tokens_to_lock),

        ExecuteMsg::UnlockCollateralToken {
            collateral_token_to_unlock,
            collateral_amount_to_unlock,
        } => execute_unlock_collateral(
            &mut deps,
            info,
            env,
            schemars::Map::from([(collateral_token_to_unlock, collateral_amount_to_unlock)]),
        ),
        ExecuteMsg::UnlockCollateralTokens {
            collateral_tokens_to_unlock,
        } => execute_unlock_collateral(&mut deps, info, env, collateral_tokens_to_unlock),

        ExecuteMsg::MintRupees { rupees_to_mint } => {
            execute_mint_rupees(&deps, info.sender.into_string(), rupees_to_mint)
        }
        ExecuteMsg::ReturnRupees { rupees_to_return } => {
            execute_return_rupees(&deps, info.sender.into_string(), rupees_to_return)
        }

        ExecuteMsg::LiquidateStablecoins {
            liquidate_stablecoin_minter_address,
        } => execute_liquidate_stablecoin_minter(
            &deps,
            info.sender.into_string(),
            liquidate_stablecoin_minter_address,
        ),

        ExecuteMsg::SetCollateralPricesInRupees {
            collateral_prices_in_rupees,
        } => execute_set_collateral_prices_in_rupees(
            &deps,
            info.sender.into_string(),
            collateral_prices_in_rupees,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryCollateralPrices { collateral_tokens } => {
            query_collateral_prices(&deps, collateral_tokens)
        }
        QueryMsg::QueryLockedCollateral {
            collateral_address_to_query,
        } => query_locked_collateral(&deps, collateral_address_to_query),
        QueryMsg::QueryStablecoinHealth {
            stablecoin_minter_address_to_query,
        } => query_stablecoin_health(&deps, stablecoin_minter_address_to_query),
    }
}

/****
 * THIS IS THE SECTION FOR ACTUAL IMPLEMENTATIONS OF ALL THE FUNCTIONS USED ABOVE!
 ****/

// Function to lock a single collateral token
fn execute_lock_collateral(
    deps: &mut DepsMut,
    info: MessageInfo,
    env: Env,
    collateral_tokens: schemars::Map<CollateralToken, u128>,
) -> Result<Response, ContractError> {
    // First, check if the address trying to lock collateral has enabled spending
    // the amount of collateral they're trying to lock

    let mut lock_collateral_messages: Vec<CosmosMsg> = Vec::new();
    let sender_address = info.sender.to_string().clone();

    let mut funds_deposited = info
        .funds
        .iter()
        .find(|coin| {
            coin.denom
                == NATIVE_TOKEN_DENOM
                    .load(deps.storage)
                    .unwrap_or(String::from("uatom"))
        })
        .map(|coin| coin.amount.u128())
        .unwrap_or(0);

    for (collateral_token, collateral_amount) in collateral_tokens {
        match collateral_token.clone() {
            CollateralToken::NativeToken => {
                if funds_deposited < collateral_amount {
                    panic!("Not enough funds deposited");
                    // return Err(ContractError::Std ::generic_err("Incorrect deposit amount sent."));
                }
                funds_deposited -= collateral_amount;

                LOCKED_COLLATERALS.update(deps.storage, Addr::unchecked(sender_address.clone()),
            |previously_locked_collaterals|
                    -> StdResult<schemars::Map<CollateralToken, u128>> {
                        let mut collaterals = previously_locked_collaterals.unwrap_or_default();
                        *collaterals.entry(collateral_token).or_insert(0) += collateral_amount;
                        Ok(collaterals)
                });
            }

            CollateralToken::CW20Token(cw20_collateral_token_addr) => {
                let transfer_msg = Cw20ExecuteMsg::TransferFrom {
                    owner: sender_address.clone(), // The user (sender) must have approved the contract to spend their tokens
                    recipient: env.contract.address.to_string(), // The contract itself as the recipient
                    amount: collateral_amount.into(),            // The amount to transfer
                };

                // Create a WasmMsg to call the CW20 contract
                let transfer_from_msg = WasmMsg::Execute {
                    contract_addr: cw20_collateral_token_addr.to_string(), // Address of the CW20 token contract
                    msg: to_json_binary(&transfer_msg)?, // Convert the TransferFrom message to binary format
                    funds: vec![], // No native tokens are sent along with this message
                };

                lock_collateral_messages.push(transfer_from_msg.into());

                LOCKED_COLLATERALS.update(deps.storage, Addr::unchecked(sender_address.clone()),
                     |previously_locked_collaterals|
                     -> StdResult<schemars::Map<CollateralToken, u128>> {
                        let mut collaterals = previously_locked_collaterals.unwrap_or_default();
                        *collaterals.entry(collateral_token).or_insert(0) += collateral_amount;
                        Ok(collaterals)
                     }
                    );

                // // Add the transfer message to the response
                // return Ok(Response::new()
                //     .add_message(transfer_from_msg)
                //     .add_attribute("action", "transfer_cw20_collateral")
                //     .add_attribute("cw20_contract", cw20_collateral_token_addr)
                //     .add_attribute(
                //         "amount",
                //         collateral_token_amount.collateral_amount.to_string(),
                //     ));
            }

            CollateralToken::CW721Token(cw721_collateral_token_addr) => {
                panic!("UNIMPLEMENTED. Can't have NFTs as collateral just yet.");
            }
        }
    }

    // Send the lock collateral messages and return the Ok response
    Ok(Response::new()
        .add_messages(lock_collateral_messages)
        .add_attribute("action", "lock_collateral")
        .add_attribute("sender", sender_address))
}

// Function to unlock a single collateral token
fn execute_unlock_collateral(
    deps: &mut DepsMut,
    info: MessageInfo,
    env: Env,
    collateral_tokens: schemars::Map<CollateralToken, u128>,
) -> Result<Response, ContractError> {
    panic!("TODO: Implement this function!");
}

// Function to mint rupees
fn execute_mint_rupees(
    deps: &DepsMut,
    sender: String,
    rupees_to_mint: u128,
) -> Result<Response, ContractError> {
    panic!("TODO: Implement this function!");
}

// Function to return rupees
fn execute_return_rupees(
    deps: &DepsMut,
    sender: String,
    rupees_to_return: u128,
) -> Result<Response, ContractError> {
    panic!("TODO: Implement this function!");
}

// Function to liquidate stablecoins
fn execute_liquidate_stablecoin_minter(
    deps: &DepsMut,
    sender: String,
    liquidate_stablecoin_minter_address: String,
) -> Result<Response, ContractError> {
    panic!("TODO: Implement this function!");
}

// Function to set collateral prices in rupees
fn execute_set_collateral_prices_in_rupees(
    deps: &DepsMut,
    sender: String,
    collateral_prices_in_rupees: schemars::Map<CollateralToken, u128>,
) -> Result<Response, ContractError> {
    panic!("TODO: Implement this function!");
}

// Query function to get collateral prices
fn query_collateral_prices(
    deps: &Deps,
    collateral_tokens: Option<Vec<CollateralToken>>,
) -> StdResult<Binary> {
    panic!("TODO: Implement this function!");
}

// Query function to get locked collateral
fn query_locked_collateral(deps: &Deps, collateral_address_to_query: String) -> StdResult<Binary> {
    panic!("TODO: Implement this function!");
}

// Query function to get stablecoin health
fn query_stablecoin_health(
    deps: &Deps,
    stablecoin_minter_address_to_query: String,
) -> StdResult<Binary> {
    panic!("TODO: Implement this function!");
}

/****
 * THIS IS THE SECTION FOR ALL TESTS
 ****/

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
