use std::borrow::BorrowMut;

use cosmwasm_std::{coins, ContractResult, Response, Decimal};
use cosmwasm_vm::{
    VmResult, VmError, Instance, to_vec, call_instantiate, call_execute,
    testing::{MockApi, MockStorage, MockQuerier, mock_env, mock_info, mock_instance_with_gas_limit}
};
use cw_storage_plus::Map;
use stable_rupee::{msg::{ExecuteMsg, InstantiateMsg}, state::CollateralToken}; // Replace with your contract module

// use serde_json_wasm::to_vec;

const WASM: &[u8] = include_bytes!("../artifacts/stable_rupee.wasm"); // Path to compiled Wasm

/// Sets up a mock instance and instantiates the contract.
/// Returns the initialized `Instance` and `Response`.
fn setup_instance() -> (Instance<MockApi, MockStorage, MockQuerier>, Response) {
    let mut instance = mock_instance_with_gas_limit(WASM, 5_000_000_000_000_000);

    let response = {
        let info = mock_info("creator", &coins(1000, "token"));
        let env = mock_env();

        let msg = InstantiateMsg {
            liquidation_health: Decimal::from_ratio(3u32, 2u32),
            allowed_collaterals: vec![CollateralToken::NativeToken],
        };
    
        let msg_binary = &*to_vec(&msg).unwrap();
    
        let res: VmResult<ContractResult<Response>> =
            call_instantiate(instance.borrow_mut(), &env, &info, msg_binary);
    
        res.unwrap().unwrap()
    };

    (instance, response)
}

#[test]
fn test_instantiate_contract() {
    let (_instance, response) = setup_instance();

    dbg!(&response);
    assert_eq!(response.attributes[0].value, "instantiate");
}

#[test]
fn test_lock_collateral() {
    let (mut instance, _response) = setup_instance();

    // let msg_to_execute = ExecuteMsg::LockCollateralTokens {
    //     collateral_tokens_to_lock: schemars::Map::from([
    //         (CollateralToken::CW20Token("usdc".into()), 200u128),
    //         (CollateralToken::NativeToken, 796u128)
    //     ]),
    // };

    let msg_to_execute = ExecuteMsg::LockCollateralToken {
        collateral_token_to_lock: CollateralToken::NativeToken,
        collateral_amount_to_lock: 102u128,
    };

    let msg_binary = &*to_vec(&msg_to_execute).unwrap();

    let info = mock_info("creator", &coins(10000, "token"));
    let env = mock_env();
    let res: VmResult<ContractResult<Response>> = call_execute(instance.borrow_mut(), &env, &info, msg_binary);

    dbg!(&res);
    assert!(res.is_ok());

    let unlock_msg_to_execute = ExecuteMsg::UnlockCollateralToken {
        collateral_token_to_unlock: CollateralToken::NativeToken,
        collateral_amount_to_unlock: 1040u128,
    };

    dbg!(&unlock_msg_to_execute);

    let msg_binary = &*to_vec(&unlock_msg_to_execute).unwrap();

    // let info = mock_info("creator", &coins(100, "token"));
    // let env = mock_env();
    let res: VmResult<ContractResult<Response>> = call_execute(instance.borrow_mut(), &env, &info, msg_binary);

    dbg!(&res);
    assert!(res.is_ok());
}