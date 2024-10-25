#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    CollateralToken, CollateralTokenAmount,
    ADMIN_ADDRESS,
    ALLOWED_COLLATERALS, COLLATERAL_TOKEN_PRICES, LIQUIDATION_HEALTH,
    LOCKED_COLLATERALS, MINTED_RUPEES,
};

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
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::LockCollateralToken { collateral_token_to_lock } => 
            execute_lock_collateral(&deps, info.sender.into_string(), vec![collateral_token_to_lock]),
        ExecuteMsg::LockCollateralTokens { collateral_tokens_to_lock } =>
            execute_lock_collateral(&deps, info.sender.into_string(), collateral_tokens_to_lock),

        ExecuteMsg::UnlockCollateralToken { collateral_token_to_unlock } =>
            execute_unlock_collateral(&deps, info.sender.into_string(), vec![collateral_token_to_unlock]),
        ExecuteMsg::UnlockCollateralTokens { collateral_tokens_to_unlock } =>
            execute_unlock_collateral(&deps, info.sender.into_string(), collateral_tokens_to_unlock),

        ExecuteMsg::MintRupees { rupees_to_mint } =>
            execute_mint_rupees(&deps, info.sender.into_string(), rupees_to_mint),
        ExecuteMsg::ReturnRupees { rupees_to_return } =>
            execute_return_rupees(&deps, info.sender.into_string(), rupees_to_return),

        ExecuteMsg::LiquidateStablecoins { liquidate_stablecoin_minter_address } =>
            execute_liquidate_stablecoin_minter(&deps, info.sender.into_string(), liquidate_stablecoin_minter_address), 

        ExecuteMsg::SetCollateralPricesInRupees { collateral_prices_in_rupees } =>
            execute_set_collateral_prices_in_rupees(&deps, info.sender.into_string(), collateral_prices_in_rupees),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryCollateralPrices { collateral_tokens } =>
            query_collateral_prices(&deps, collateral_tokens),
        QueryMsg::QueryLockedCollateral { collateral_address_to_query } =>
            query_locked_collateral(&deps, collateral_address_to_query),
        QueryMsg::QueryStablecoinHealth { stablecoin_minter_address_to_query } =>
            query_stablecoin_health(&deps, stablecoin_minter_address_to_query),
    }
}


/****
 * THIS IS THE SECTION FOR ACTUAL IMPLEMENTATIONS OF ALL THE FUNCTIONS USED ABOVE!
 ****/

// Function to lock a single collateral token
fn execute_lock_collateral(
    deps: &DepsMut,
    sender: String,
    collateral_tokens: Vec<CollateralTokenAmount>,
) -> Result<Response, ContractError> {
    panic!("TODO: Implement this function!");
}

// Function to unlock a single collateral token
fn execute_unlock_collateral(
    deps: &DepsMut,
    sender: String,
    collateral_tokens: Vec<CollateralTokenAmount>,
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
fn query_locked_collateral(
    deps: &Deps,
    collateral_address_to_query: String,
) -> StdResult<Binary> {
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
