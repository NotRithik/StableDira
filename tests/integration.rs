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
};

const WASM: &[u8] = include_bytes!("../artifacts/stable_dira.wasm");

/// Sets up a mock instance and instantiates the contract.
/// Returns the initialized `Instance` and `Response`.
fn setup_instance() -> (Instance<MockApi, MockStorage, MockQuerier>, Response, Env) {
    let mut instance = mock_instance_with_gas_limit(WASM, 5_000_000_000_000_000);
    let env = mock_env();

    let response = {
        let info = mock_info("creator", &[]);

        let msg = InstantiateMsg {
            liquidation_health: Decimal::from_ratio(11u32, 10u32),
            mintable_health: Decimal::from_ratio(13u32, 10u32), // Added mintable_health
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
fn test_admin_functions() {
    let (mut instance, _response, env) = setup_instance();

    let admin_info = mock_info("creator", &[]);
    let non_admin_info = mock_info("non_admin", &[]);

    // Test Set Collateral Price
    let set_collateral_price_msg = ExecuteMsg::SetCollateralPriceInDirham {
        collateral_price_in_aed: Decimal::from_ratio(3309u128, 100u128),
    };
    let res: ContractResult<Response> = call_execute(
        instance.borrow_mut(),
        &env,
        &admin_info,
        &to_vec(&set_collateral_price_msg).unwrap(),
    ).unwrap();

    assert!(res.is_ok());

    let res: ContractResult<Response> = call_execute(
        instance.borrow_mut(),
        &env,
        &non_admin_info,
        &to_vec(&set_collateral_price_msg).unwrap(),
    ).unwrap();

    assert!(res.is_err());

    // Test Set Mintable Health
    let set_mintable_health_msg = ExecuteMsg::SetMintableHealth {
        mintable_health: Decimal::percent(195),
    };
    let res: ContractResult<Response> = call_execute(
        instance.borrow_mut(),
        &env,
        &admin_info,
        &to_vec(&set_mintable_health_msg).unwrap(),
    ).unwrap();
    assert!(res.is_ok());

    let res: ContractResult<Response> = call_execute(
        instance.borrow_mut(),
        &env,
        &non_admin_info,
        &to_vec(&set_mintable_health_msg).unwrap(),
    ).unwrap();

    assert!(res.is_err());

    // Test Set Liquidation Health
    let set_liquidation_health_msg = ExecuteMsg::SetLiquidationHealth {
        liquidation_health: Decimal::percent(85),
    };
    let res: ContractResult<Response> = call_execute(
        instance.borrow_mut(),
        &env,
        &admin_info,
        &to_vec(&set_liquidation_health_msg).unwrap(),
    ).unwrap();

    assert!(res.is_ok());

    let res: ContractResult<Response> = call_execute(
        instance.borrow_mut(),
        &env,
        &non_admin_info,
        &to_vec(&set_liquidation_health_msg).unwrap(),
    ).unwrap();

    assert!(res.is_err());
}

#[test]
fn test_lock_unlock_collateral() {
    let (mut instance, _response, env) = setup_instance();

    let set_collateral_price_msg = ExecuteMsg::SetCollateralPriceInDirham {
        collateral_price_in_aed: Decimal::from_ratio(3309u128, 100u128),
    };
    let res: ContractResult<Response> = call_execute(
        instance.borrow_mut(),
        &env,
        &mock_info("creator", &[]),
        &to_vec(&set_collateral_price_msg).unwrap(),
    ).unwrap();

    assert!(res.is_ok());

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

    dbg!(&res);
    assert!(res.is_ok());
    dbg!("Successfully locked and unlocked collateral!");
}
