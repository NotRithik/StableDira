use std::borrow::{Borrow, BorrowMut};

use cosmwasm_std::{coins, to_json_binary, to_json_string, ContractResult, Decimal, Env, Response};
use cosmwasm_vm::{
    call_execute, call_instantiate, testing::{
        mock_env, mock_info, mock_instance_with_gas_limit, MockApi, MockQuerier, MockStorage,
    }, to_vec, Instance, Storage, VmError, VmResult
};
use cw_storage_plus::Map;
use stable_rupee::{
    msg::{ExecuteMsg, InstantiateMsg},
    state::{CollateralToken, LOCKED_COLLATERALS},
}; // Replace with your contract module

const WASM: &[u8] = include_bytes!("../artifacts/stable_rupee.wasm"); // Path to compiled Wasm

/// Sets up a mock instance and instantiates the contract.
/// Returns the initialized `Instance` and `Response`.
fn setup_instance() -> (Instance<MockApi, MockStorage, MockQuerier>, Response, Env) {
    let mut instance = mock_instance_with_gas_limit(WASM, 5_000_000_000_000_000);
    let env = mock_env();

    let response = {
        let info = mock_info("creator", &coins(1000, "token"));

        let msg = InstantiateMsg {
            liquidation_health: Decimal::from_ratio(3u32, 2u32),
            allowed_collaterals: vec![CollateralToken::NativeToken],
            native_token_denom: String::from("uom"),
        };

        dbg!(&msg);
        dbg!(to_json_string(&msg));

        let msg_binary = &*to_vec(&msg).unwrap();

        let res: VmResult<ContractResult<Response>> =
            call_instantiate(instance.borrow_mut(), &env, &info, msg_binary);

        res.unwrap().unwrap()
    };

    (instance, response, env)
}

// #[test]
// fn test_instantiate_contract() {
//     let (_instance, response) = setup_instance();

//     dbg!(&response);
//     assert_eq!(response.attributes[0].value, "instantiate");
// }

#[test]
fn test_lock_unlock_collateral() {
    let (mut instance, _response, env) = setup_instance();

    // let msg_to_execute = ExecuteMsg::LockCollateralTokens {
    //     collateral_tokens_to_lock: schemars::Map::from([
    //         (CollateralToken::CW20Token("usdc".into()), 200u128),
    //         (CollateralToken::NativeToken, 796u128)
    //     ]),
    // };

    let msg_to_execute = ExecuteMsg::LockCollateralToken {
        // collateral_token_to_lock: CollateralToken::CW20Token("cadf".into()),
        collateral_token_to_lock: CollateralToken::NativeToken,
        collateral_amount_to_lock: 102u128,
    };

    dbg!(&msg_to_execute);

    let msg_binary = &*to_json_binary(&msg_to_execute).unwrap();

    dbg!(to_json_string(&msg_to_execute));

    let info = mock_info("creator", &coins(102, "uom"));
    let res: ContractResult<Response> =
        call_execute(instance.borrow_mut(), &env, &info, msg_binary).unwrap();

    dbg!(&res);
    assert!(res.is_ok());

    let unlock_msg_to_execute = ExecuteMsg::UnlockCollateral {
        collateral_token_to_unlock: CollateralToken::NativeToken,
        collateral_amount_to_unlock: 102u128,
    };

    dbg!(&unlock_msg_to_execute);

    let msg_binary = &*to_json_binary(&unlock_msg_to_execute).unwrap();
    let res: ContractResult<Response> =
        call_execute(instance.borrow_mut(), &env, &info, msg_binary).unwrap();

    dbg!(&res);
    assert!(res.is_ok());
}
