use std::borrow::{Borrow, BorrowMut};

use cosmwasm_std::{coins, to_json_binary, to_json_string, ContractResult, Decimal, Env, Response};
use cosmwasm_vm::{
    call_execute, call_instantiate,
    testing::{
        mock_env, mock_info, mock_instance_with_gas_limit, MockApi, MockQuerier, MockStorage,
    },
    to_vec, Instance, Storage, VmError, VmResult,
};

use stable_dira::{
    msg::{ExecuteMsg, InstantiateMsg},
    state::LOCKED_COLLATERAL,
}; // Replace with your contract module

const WASM: &[u8] = include_bytes!("../artifacts/stable_dira.wasm"); // Path to compiled Wasm

/// Sets up a mock instance and instantiates the contract.
/// Returns the initialized `Instance` and `Response`.
fn setup_instance() -> (Instance<MockApi, MockStorage, MockQuerier>, Response, Env) {
    let mut instance = mock_instance_with_gas_limit(WASM, 5_000_000_000_000_000);
    let env = mock_env();

    let response = {
        let info = mock_info("creator", &[]);

        let msg = InstantiateMsg {
            liquidation_health: Decimal::from_ratio(3u32, 2u32),
            collateral_token_denom: String::from("uatom"),
        };

        let msg_binary = &*to_vec(&msg).unwrap();

        let res: VmResult<ContractResult<Response>> =
            call_instantiate(instance.borrow_mut(), &env, &info, msg_binary);

        assert!(res.is_ok());

        res.unwrap().unwrap()
    };

    (instance, response, env)
}

#[test]
fn test_setup_instance() {
    _ = setup_instance();
    dbg!("Successfully instantiated the contract!");
}

#[test]
fn test_lock_unlock_collateral() {
    let (mut instance, _response, env) = setup_instance();

    let msg_to_execute = ExecuteMsg::LockCollateral {
        collateral_amount_to_lock: Decimal::from_ratio(103u128, 1u128),
    };

    let msg_binary = &*to_json_binary(&msg_to_execute).unwrap();

    let info = mock_info("creator", &coins(103, "uatom"));
    let res: ContractResult<Response> =
        call_execute(instance.borrow_mut(), &env, &info, msg_binary).unwrap();

    assert!(res.is_ok());

    let unlock_msg_to_execute = ExecuteMsg::UnlockCollateral {
        collateral_amount_to_unlock: Decimal::from_ratio(103u128, 1u128),
    };

    let msg_binary = &*to_json_binary(&unlock_msg_to_execute).unwrap();
    let info = mock_info("creator", &[]); // No funds sent for unlock operation
    let res: ContractResult<Response> =
        call_execute(instance.borrow_mut(), &env, &info, msg_binary).unwrap();

    assert!(res.is_ok());
    dbg!("Successfully locked and unlocked collateral!");
}

//TODO: MODIFY TESTS TO WORK WITH UPDATED CONTRACT