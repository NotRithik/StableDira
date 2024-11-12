use std::borrow::BorrowMut;

use cosmwasm_std::{coins, ContractResult, Decimal, Response};
use cosmwasm_vm::{
    to_vec, call_instantiate, testing::{mock_env, mock_info, mock_instance_with_gas_limit}
};
use stable_rupee::{ msg::InstantiateMsg, state::CollateralToken}; // Replace with your contract module

const WASM: &[u8] = include_bytes!("../artifacts/stable_rupee.wasm"); // Path to compiled Wasm

#[test]
fn test_instantiate_contract() {
    // Create an instance with a gas limit (500k here is arbitrary for testing)
    let mut instance = mock_instance_with_gas_limit(WASM, 5_000_000_000_000_000);

    let msg = InstantiateMsg {
        liquidation_health: Decimal::from_ratio(3u32, 2u32),
        allowed_collaterals: vec![CollateralToken::NativeToken],
    };

    let info = mock_info("creator", &coins(1000, "token"));
    let env = mock_env();
    // let res = instantiate(instance.borrow_mut(), mock_env(), info, msg); // Use `deps_mut`
    let res = call_instantiate(instance.borrow_mut(), &env, &info, &to_vec(&msg).unwrap());

    dbg!(&res);
    assert!(res.is_ok());
    let response: ContractResult<Response<>> = res.unwrap();
    dbg!(&response);
    // assert_eq!(response.attributes[0].value, "instantiate");
}
