use cosmwasm_std::{coins, Decimal, Addr};
use cw_multi_test::{App, AppBuilder, BankSudo, Contract, ContractWrapper, Executor};
use stable_dira::msg::{ExecuteMsg, InstantiateMsg};

// Mock implementation for contract initialization
fn dira_contract() -> Box<dyn Contract<cosmwasm_std::Empty>> {
    let contract = ContractWrapper::new(
        stable_dira::contract::execute,
        stable_dira::contract::instantiate,
        stable_dira::contract::query,
    );
    Box::new(contract)
}

// Helper to initialize the app and contract
fn setup_app() -> (App, Addr, Addr, Addr) {
    let mut app = AppBuilder::new().build(|router, _, storage| {
        // Initialize app state with some balances
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked("cosmos1creatorxxxxxxxxxxxxxxxxxxxxxx"),
                coins(1_000_000_000, "uatom"),
            )
            .unwrap();

        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked("cosmos1nonadminxxxxxxxxxxxxxxxxxxx"),
                coins(1_000_000_000, "uatom"),
            )
            .unwrap();
    });

    // Store the contract code
    let code_id = app.store_code(dira_contract());

    // Instantiate the contract
    let contract_addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("cosmos1creatorxxxxxxxxxxxxxxxxxxxxxx"), // Admin address
            &InstantiateMsg {
                liquidation_health: Decimal::percent(110),
                mintable_health: Decimal::percent(130),
                collateral_token_denom: "uatom".to_string(),
            },
            &[],
            "Dira Stablecoin",
            None,
        )
        .unwrap();

    // Return the app instance, contract address, and user addresses
    (
        app,
        contract_addr,
        Addr::unchecked("cosmos1creatorxxxxxxxxxxxxxxxxxxxxxx"), // Admin
        Addr::unchecked("cosmos1nonadminxxxxxxxxxxxxxxxxxxx"),  // Non-admin
    )
}

#[test]
fn test_setup_instance() {
    let (_app, contract_addr, _admin, _non_admin) = setup_app();
    assert!(!contract_addr.to_string().is_empty());
    dbg!("Successfully instantiated the contract!");
}

#[test]
fn test_admin_functions() {
    let (mut app, contract_addr, admin, non_admin) = setup_app();

    // Test setting collateral price
    let msg = ExecuteMsg::SetCollateralPriceInDirham {
        collateral_price_in_dirham: Decimal::from_ratio(3309u128, 100u128),
    };
    let res = app.execute_contract(admin.clone(), contract_addr.clone(), &msg, &[]);
    assert!(res.is_ok());

    let res = app.execute_contract(non_admin.clone(), contract_addr.clone(), &msg, &[]);
    assert!(res.is_err());

    // Test setting mintable health
    let msg = ExecuteMsg::SetMintableHealth {
        mintable_health: Decimal::percent(195),
    };
    let res = app.execute_contract(admin.clone(), contract_addr.clone(), &msg, &[]);
    assert!(res.is_ok());

    let res = app.execute_contract(non_admin.clone(), contract_addr.clone(), &msg, &[]);
    assert!(res.is_err());

    // Test setting liquidation health
    let msg = ExecuteMsg::SetLiquidationHealth {
        liquidation_health: Decimal::percent(85),
    };
    let res = app.execute_contract(admin.clone(), contract_addr.clone(), &msg, &[]);
    assert!(res.is_ok());

    let res = app.execute_contract(non_admin.clone(), contract_addr.clone(), &msg, &[]);
    assert!(res.is_err());
}

#[test]
fn test_lock_unlock_collateral() {
    let (mut app, contract_addr, admin, _non_admin) = setup_app();

    // Set collateral price
    let msg = ExecuteMsg::SetCollateralPriceInDirham {
        collateral_price_in_dirham: Decimal::from_ratio(3309u128, 100u128),
    };
    let res = app.execute_contract(admin.clone(), contract_addr.clone(), &msg, &[]);
    assert!(res.is_ok());

    // Lock collateral
    let msg = ExecuteMsg::LockCollateral {};
    let res = app.execute_contract(
        admin.clone(),
        contract_addr.clone(),
        &msg,
        &coins(1204, "uatom"),
    );
    assert!(res.is_ok());

    // Unlock collateral
    let msg = ExecuteMsg::UnlockCollateral {
        collateral_amount_to_unlock: Decimal::from_atomics(1204u128, 6).unwrap(),
    };
    let res = app.execute_contract(admin.clone(), contract_addr.clone(), &msg, &[]);
    assert!(res.is_ok());

    // Attempt to unlock too much collateral (should fail)
    let msg = ExecuteMsg::UnlockCollateral {
        collateral_amount_to_unlock: Decimal::from_atomics(1500u128, 6).unwrap(),
    };
    let res = app.execute_contract(admin.clone(), contract_addr.clone(), &msg, &[]);
    assert!(res.is_err());

    dbg!("Successfully locked and unlocked collateral!");
}
