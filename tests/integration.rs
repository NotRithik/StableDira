use bech32::{encode, Hrp};
use cosmwasm_std::{coins, Addr, Decimal};
use cw20::MinterResponse;
use cw20_base::msg::{ExecuteMsg as Cw20ExecuteMsg, InstantiateMsg as Cw20InstantiateMsg};
use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};
use stable_dira::msg::{ExecuteMsg as DiraExecuteMsg, InstantiateMsg as DiraInstantiateMsg};

// Mock implementation for Dira stablecoin contract
fn dira_contract() -> Box<dyn Contract<cosmwasm_std::Empty>> {
    let contract = ContractWrapper::new(
        stable_dira::contract::execute,
        stable_dira::contract::instantiate,
        stable_dira::contract::query,
    );
    Box::new(contract)
}

// Mock implementation for CW20 base contract
fn cw20_contract() -> Box<dyn Contract<cosmwasm_std::Empty>> {
    let contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
}

// Generate Bech32 Address:
// dbg!(encode::<bech32::Bech32>(
//     Hrp::parse("cosmwasm").unwrap(),
//     &[
//         0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14,
//         0x15, 0x16, 0x17, 0x18, 0x12, 0x20
//     ]
// )
// .unwrap());

// Helper to initialize the app and deploy both contracts
fn setup_app() -> (App, Addr, Addr, Addr, Addr) {
    let mut app = AppBuilder::new().build(|router, _, storage| {
        // Initialize app state with some balances
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked("cosmwasm1qypqxpq9qcrsszgszyfpx9q4zct3sxfqx5vwjh"),
                coins(1_000_000_000, "uatom"),
            )
            .unwrap();

        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked("cosmwasm1qypqxpq9qcrsszgszyfpx9q4zct3sy3q8mmchv"),
                coins(1_000_000_000, "uatom"),
            )
            .unwrap();
    });

    // Store the Dira stablecoin contract code
    let dira_code_id = app.store_code(dira_contract());

    // Instantiate the Dira stablecoin contract
    let dira_contract_addr = app
        .instantiate_contract(
            dira_code_id,
            Addr::unchecked("cosmwasm1qypqxpq9qcrsszgszyfpx9q4zct3sxfqx5vwjh"), // Admin address
            &DiraInstantiateMsg {
                liquidation_health: Decimal::percent(110),
                mintable_health: Decimal::percent(130),
                collateral_token_denom: "uatom".to_string(),
            },
            &[],
            "Dira Stablecoin",
            None,
        )
        .unwrap();

    // Store the CW20 base contract code
    let cw20_code_id = app.store_code(cw20_contract());

    // Instantiate the CW20 token contract
    let cw20_contract_addr = app
        .instantiate_contract(
            cw20_code_id,
            Addr::unchecked("cosmwasm1qypqxpq9qcrsszgszyfpx9q4zct3sxfqx5vwjh"), // Admin address
            &Cw20InstantiateMsg {
                name: "Dira".to_string(),
                symbol: "DIRA".to_string(),
                decimals: 6,
                initial_balances: vec![],
                mint: Some(MinterResponse {
                    minter: "cosmwasm1qypqxpq9qcrsszgszyfpx9q4zct3sxfqx5vwjh".to_string(),
                    cap: None,
                }),
                marketing: None,
            },
            &[],
            "CW20 Dira Token",
            None,
        )
        .unwrap();

    // Return the app instance, both contract addresses, and user addresses
    (
        app,
        dira_contract_addr,
        cw20_contract_addr,
        Addr::unchecked("cosmwasm1qypqxpq9qcrsszgszyfpx9q4zct3sxfqx5vwjh"), // Admin
        Addr::unchecked("cosmwasm1qypqxpq9qcrsszgszyfpx9q4zct3sy3q8mmchv"), // Non-admin
    )
}

#[test]
fn test_setup_instance() {
    let (_app, dira_contract_addr, cw20_contract_addr, _admin, _non_admin) = setup_app();
    assert!(!dira_contract_addr.to_string().is_empty());
    assert!(!cw20_contract_addr.to_string().is_empty());
    dbg!("Successfully instantiated both contracts!");
}

#[test]
fn test_admin_functions() {
    let (mut app, dira_contract_addr, _cw20_contract_addr, admin, non_admin) = setup_app();

    // Test setting collateral price
    let msg = DiraExecuteMsg::SetCollateralPriceInDirham {
        collateral_price_in_dirham: Decimal::from_ratio(3309u128, 100u128),
    };
    let res = app.execute_contract(admin.clone(), dira_contract_addr.clone(), &msg, &[]);
    assert!(res.is_ok());

    let res = app.execute_contract(non_admin.clone(), dira_contract_addr.clone(), &msg, &[]);
    assert!(res.is_err());

    // Test setting mintable health
    let msg = DiraExecuteMsg::SetMintableHealth {
        mintable_health: Decimal::percent(195),
    };
    let res = app.execute_contract(admin.clone(), dira_contract_addr.clone(), &msg, &[]);
    assert!(res.is_ok());

    let res = app.execute_contract(non_admin.clone(), dira_contract_addr.clone(), &msg, &[]);
    assert!(res.is_err());

    // Test setting liquidation health
    let msg = DiraExecuteMsg::SetLiquidationHealth {
        liquidation_health: Decimal::percent(85),
    };
    let res = app.execute_contract(admin.clone(), dira_contract_addr.clone(), &msg, &[]);
    assert!(res.is_ok());

    let res = app.execute_contract(non_admin.clone(), dira_contract_addr.clone(), &msg, &[]);
    assert!(res.is_err());
}

#[test]
fn test_lock_unlock_collateral() {
    let (mut app, dira_contract_addr, _cw20_contract_addr, admin, _non_admin) = setup_app();

    // Set collateral price
    let msg = DiraExecuteMsg::SetCollateralPriceInDirham {
        collateral_price_in_dirham: Decimal::from_ratio(3309u128, 100u128),
    };
    let res = app.execute_contract(admin.clone(), dira_contract_addr.clone(), &msg, &[]);
    assert!(res.is_ok());

    // Lock collateral
    let msg = DiraExecuteMsg::LockCollateral {};
    let res = app.execute_contract(
        admin.clone(),
        dira_contract_addr.clone(),
        &msg,
        &coins(1204, "uatom"),
    );
    assert!(res.is_ok());

    // Unlock collateral
    let msg = DiraExecuteMsg::UnlockCollateral {
        collateral_amount_to_unlock: Decimal::from_atomics(1204u128, 6).unwrap(),
    };
    let res = app.execute_contract(admin.clone(), dira_contract_addr.clone(), &msg, &[]);
    assert!(res.is_ok());

    // Attempt to unlock too much collateral (should fail)
    let msg = DiraExecuteMsg::UnlockCollateral {
        collateral_amount_to_unlock: Decimal::from_atomics(1500u128, 6).unwrap(),
    };
    let res = app.execute_contract(admin.clone(), dira_contract_addr.clone(), &msg, &[]);
    assert!(res.is_err());

    dbg!("Successfully locked and unlocked collateral!");
}

#[test]
fn test_mint_redeem_dira() {
    let (mut app, dira_contract_addr, cw20_contract_addr, admin, non_admin) = setup_app();

    // Update the CW20 token's minter to the Dira contract
    let update_minter_msg = Cw20ExecuteMsg::UpdateMinter {
        new_minter: Some(dira_contract_addr.to_string()),
    };
    let res = app.execute_contract(
        admin.clone(),
        cw20_contract_addr.clone(),
        &update_minter_msg,
        &[],
    );
    dbg!(&res);
    assert!(res.is_ok());
    dbg!("Updated CW20 token minter to Dira contract");

    // Lock collateral from the admin user
    let set_collateral_price_msg = DiraExecuteMsg::SetCollateralPriceInDirham {
        collateral_price_in_dirham: Decimal::from_ratio(3309u128, 100u128),
    };
    let res = app.execute_contract(
        admin.clone(),
        dira_contract_addr.clone(),
        &set_collateral_price_msg,
        &[],
    );
    assert!(res.is_ok());
    dbg!("Set collateral price");

    let lock_collateral_msg = DiraExecuteMsg::LockCollateral {};
    let res = app.execute_contract(
        admin.clone(),
        dira_contract_addr.clone(),
        &lock_collateral_msg,
        &coins(10_000, "uatom"),
    );
    assert!(res.is_ok());
    dbg!("Locked collateral from admin");

    // Lock collateral from the non-admin user
    let res = app.execute_contract(
        non_admin.clone(),
        dira_contract_addr.clone(),
        &lock_collateral_msg,
        &coins(5_000, "uatom"),
    );
    assert!(res.is_ok());
    dbg!("Locked collateral from non-admin");

    dbg!("Successfully set up mint/redeem scenario with collateral locked");
}
