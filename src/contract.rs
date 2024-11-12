#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, WasmMsg, WasmQuery,
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
    msg: InstantiateMsg,
    // liquidation_health: f32,
    // allowed_collaterals: Vec<CollateralToken>,
) -> Result<Response, ContractError> {
    deps.api.debug("Instantiating contract...");
    deps.api.debug(&format!("Received message: {:?}", msg));

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ADMIN_ADDRESS.save(deps.storage, &info.sender)?;
    LIQUIDATION_HEALTH.save(deps.storage, &msg.liquidation_health)?;
    ALLOWED_COLLATERALS.save(deps.storage, &msg.allowed_collaterals)?;

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
            deps,
            info,
            env,
            schemars::Map::from([(collateral_token_to_lock, collateral_amount_to_lock)]),
        ),
        ExecuteMsg::LockCollateralTokens {
            collateral_tokens_to_lock,
        } => execute_lock_collateral(deps, info, env, collateral_tokens_to_lock),

        ExecuteMsg::UnlockCollateralToken {
            collateral_token_to_unlock,
            collateral_amount_to_unlock,
        } => execute_unlock_collateral(
            deps,
            info,
            env,
            schemars::Map::from([(collateral_token_to_unlock, collateral_amount_to_unlock)]),
        ),
        ExecuteMsg::UnlockCollateralTokens {
            collateral_tokens_to_unlock,
        } => execute_unlock_collateral(deps, info, env, collateral_tokens_to_unlock),

        ExecuteMsg::MintRupees { rupees_to_mint } => {
            execute_mint_rupees(deps, info.sender.into_string(), rupees_to_mint)
        }
        ExecuteMsg::ReturnRupees { rupees_to_return } => {
            execute_return_rupees(deps, info.sender.into_string(), rupees_to_return)
        }

        ExecuteMsg::LiquidateStablecoins {
            liquidate_stablecoin_minter_address,
        } => execute_liquidate_stablecoin_minter(
            deps,
            info.sender.into_string(),
            liquidate_stablecoin_minter_address,
        ),

        ExecuteMsg::SetCollateralPricesInRupees {
            collateral_prices_in_rupees,
        } => execute_set_collateral_prices_in_rupees(
            deps,
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
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collateral_tokens: schemars::Map<CollateralToken, u128>,
) -> Result<Response, ContractError> {
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
                }
                funds_deposited -= collateral_amount;

                LOCKED_COLLATERALS.update(deps.storage, Addr::unchecked(sender_address.clone()),
            |previously_locked_collaterals|
                    -> StdResult<schemars::Map<CollateralToken, u128>> {
                        let mut collaterals = previously_locked_collaterals.unwrap_or_default();
                        *collaterals.entry(collateral_token).or_insert(0) += collateral_amount;
                        Ok(collaterals)
                })?;
                // TODO: Implement error handling for different types of errors that could be returned instead of
                // propagating the error up with stack with ?
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
                    )?;
                // TODO: Implement error handling for different types of errors that could be returned instead of
                // propagating the error up with stack with ?

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
                panic!("UNIMPLEMENTED. Can't have NFTs or CW721 tokens as collateral just yet.");
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
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collateral_tokens: schemars::Map<CollateralToken, u128>,
) -> Result<Response, ContractError> {
    let mut unlock_collateral_messages: Vec<CosmosMsg> = Vec::new();
    let sender_address = info.sender.to_string().clone();

    for (collateral_token, collateral_amount) in collateral_tokens {
        match collateral_token.clone() {
            CollateralToken::NativeToken => {
                LOCKED_COLLATERALS.update(deps.storage, Addr::unchecked(sender_address.clone()),
            |previously_locked_collaterals|
                    -> StdResult<schemars::Map<CollateralToken, u128>> {
                        let mut collaterals = previously_locked_collaterals.unwrap_or_default();
                        collaterals.entry(collateral_token).and_modify(|locked_amount| {
                            if *locked_amount >= collateral_amount {
                                *locked_amount -= collateral_amount;
                            } else {
                                //TODO: Error handling
                                panic!("Not enough collateral locked!");
                            }
                        }).or_insert_with(|| {
                            //TODO: Error handling
                            panic!("Not enough collateral locked!");
                        });
                        Ok(collaterals)
                })?;

                // Create unlock collateral message
                let send_msg = CosmosMsg::Bank(BankMsg::Send {
                    to_address: sender_address.clone(),
                    amount: vec![Coin {
                        denom: NATIVE_TOKEN_DENOM
                            .load(deps.storage)
                            .unwrap_or(String::from("uatom")),
                        amount: collateral_amount.into(),
                    }],
                });

                // Push the message onto the unlock_collateral_messages vector
                unlock_collateral_messages.push(send_msg);
            }

            CollateralToken::CW20Token(cw20_collateral_token_addr) => {
                LOCKED_COLLATERALS.update(deps.storage, Addr::unchecked(sender_address.clone()),
            |previously_locked_collaterals|
                    -> StdResult<schemars::Map<CollateralToken, u128>> {
                        let mut collaterals = previously_locked_collaterals.unwrap_or_default();
                        collaterals.entry(collateral_token).and_modify(|locked_amount| {
                            if *locked_amount >= collateral_amount {
                                *locked_amount -= collateral_amount;
                            } else {
                                //TODO: Error handling
                                panic!("Not enough collateral locked!");
                            }
                        }).or_insert_with(|| {
                            //TODO: Error handling
                            panic!("Not enough collateral locked!");
                        });
                        Ok(collaterals)
                })?;

                // Create unlock collateral message
                let transfer_message = Cw20ExecuteMsg::Transfer {
                    recipient: (sender_address.clone()),
                    amount: (collateral_amount.into()),
                };

                let cw20collateral_transfer_message = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: cw20_collateral_token_addr.to_string(),
                    msg: to_json_binary(&transfer_message)?,
                    funds: vec![],
                });

                // Push the message onto the unlock_collateral_messages vector
                unlock_collateral_messages.push(cw20collateral_transfer_message);
            }

            CollateralToken::CW721Token(cw721_collateral_token_addr) => {
                panic!("UNIMPLEMENTED. Can't unlock NFTs or CW721 tokens as collateral just yet.");
            }
        }
    }

    // panic!("TODO: Implement this function!");
    Ok(Response::new()
        .add_messages(unlock_collateral_messages)
        .add_attribute("action", "lock_collateral")
        .add_attribute("sender", sender_address))
}

// Function to mint rupees
fn execute_mint_rupees(
    deps: DepsMut,
    sender: String,
    rupees_to_mint: u128,
) -> Result<Response, ContractError> {
    panic!("TODO: Implement this function!");
}

// Function to return rupees
fn execute_return_rupees(
    deps: DepsMut,
    sender: String,
    rupees_to_return: u128,
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
fn execute_set_collateral_prices_in_rupees(
    deps: DepsMut,
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

//  #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
//     use cosmwasm_std::{coins, from_binary};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies_with_balance(&coins(1000, "umantra"));

//         let msg = InstantiateMsg {
//             liquidation_health: 1.5,
//             allowed_collaterals: vec![
//                 CollateralToken::NativeToken,
//                 CollateralToken::CW20Token(Addr::unchecked("cw20_token")),
//             ],
//         };
//         let info = mock_info("admin", &[]);

//         let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         let stored_admin = ADMIN_ADDRESS.load(&deps.storage).unwrap();
//         assert_eq!(stored_admin, info.sender);

//         let allowed_collaterals = ALLOWED_COLLATERALS.load(&deps.storage).unwrap();
//         assert_eq!(
//             allowed_collaterals,
//             vec![
//                 CollateralToken::NativeToken,
//                 CollateralToken::CW20Token(Addr::unchecked("cw20_token")),
//             ]
//         );
//     }

//     #[test]
//     fn lock_native_token_collateral() {
//         let mut deps = mock_dependencies_with_balance(&coins(1000, "umantra"));

//         let msg = InstantiateMsg {
//             liquidation_health: 1.5,
//             allowed_collaterals: vec![CollateralToken::NativeToken],
//         };
//         let info = mock_info("admin", &[]);
//         instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // Lock 500 umantra as collateral
//         let lock_msg = ExecuteMsg::LockCollateralToken {
//             collateral_token_to_lock: CollateralToken::NativeToken,
//             collateral_amount_to_lock: 500,
//         };
//         let info = mock_info("user1", &coins(500, "umantra"));

//         let res = execute(deps.as_mut(), mock_env(), info.clone(), lock_msg).unwrap();
//         assert_eq!(1, res.messages.len());

//         // Query locked collateral
//         let query_msg = QueryMsg::QueryLockedCollateral {
//             collateral_address_to_query: "user1".to_string(),
//         };
//         let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
//         let locked_collateral: schemars::Map<CollateralToken, u128> = from_binary(&res).unwrap();
//         assert_eq!(locked_collateral[&CollateralToken::NativeToken], 500);
//     }

//     #[test]
//     fn unlock_native_token_collateral() {
//         let mut deps = mock_dependencies_with_balance(&coins(1000, "umantra"));

//         let msg = InstantiateMsg {
//             liquidation_health: 1.5,
//             allowed_collaterals: vec![CollateralToken::NativeToken],
//         };
//         let info = mock_info("admin", &[]);
//         instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // Lock 500 umantra as collateral
//         let lock_msg = ExecuteMsg::LockCollateralToken {
//             collateral_token_to_lock: CollateralToken::NativeToken,
//             collateral_amount_to_lock: 500,
//         };
//         let info = mock_info("user1", &coins(500, "umantra"));
//         execute(deps.as_mut(), mock_env(), info.clone(), lock_msg).unwrap();

//         // Unlock 300 umantra
//         let unlock_msg = ExecuteMsg::UnlockCollateralToken {
//             collateral_token_to_unlock: CollateralToken::NativeToken,
//             collateral_amount_to_unlock: 300,
//         };
//         let res = execute(deps.as_mut(), mock_env(), info.clone(), unlock_msg).unwrap();
//         assert_eq!(1, res.messages.len());

//         // Query locked collateral
//         let query_msg = QueryMsg::QueryLockedCollateral {
//             collateral_address_to_query: "user1".to_string(),
//         };
//         let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
//         let locked_collateral: schemars::Map<CollateralToken, u128> = from_binary(&res).unwrap();
//         assert_eq!(locked_collateral[&CollateralToken::NativeToken], 200);
//     }

//     #[test]
//     fn lock_cw20_token_collateral() {
//         let mut deps = mock_dependencies_with_balance(&coins(1000, "umantra"));

//         let msg = InstantiateMsg {
//             liquidation_health: 1.5,
//             allowed_collaterals: vec![CollateralToken::CW20Token(Addr::unchecked("cw20_token"))],
//         };
//         let info = mock_info("admin", &[]);
//         instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // Lock 500 CW20 tokens as collateral
//         let lock_msg = ExecuteMsg::LockCollateralToken {
//             collateral_token_to_lock: CollateralToken::CW20Token(Addr::unchecked("cw20_token")),
//             collateral_amount_to_lock: 500,
//         };
//         let info = mock_info("user1", &[]);

//         let res = execute(deps.as_mut(), mock_env(), info.clone(), lock_msg).unwrap();
//         assert_eq!(1, res.messages.len());

//         // Query locked collateral
//         let query_msg = QueryMsg::QueryLockedCollateral {
//             collateral_address_to_query: "user1".to_string(),
//         };
//         let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
//         let locked_collateral: schemars::Map<CollateralToken, u128> = from_binary(&res).unwrap();
//         assert_eq!(
//             locked_collateral[&CollateralToken::CW20Token(Addr::unchecked("cw20_token"))],
//             500
//         );
//     }
// }
 