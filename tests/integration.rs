use cosmwasm_std::{coins, Addr, Decimal, Uint128};
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
// dbg!(bech32::encode::<bech32::Bech32>(
//     bech32::Hrp::parse("cosmwasm").unwrap(),
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

    // Store the Dira stablecoin contract code
    let dira_code_id = app.store_code(dira_contract());

    // Instantiate the Dira stablecoin contract
    let dira_contract_addr = app
        .instantiate_contract(
            dira_code_id,
            Addr::unchecked("cosmwasm1qypqxpq9qcrsszgszyfpx9q4zct3sxfqx5vwjh"), // Admin address
            &DiraInstantiateMsg {
                liquidation_health: Decimal::from_ratio(110u128, 100u128),
                mintable_health: Decimal::from_ratio(130u128, 100u128),
                collateral_token_denom: "uatom".to_string(),
                cw20_dira_contract_address: Some(cw20_contract_addr.clone()),
            },
            &[],
            "Dira Stablecoin",
            None,
        )
        .unwrap();

    // Update the CW20 token's minter to the Dira contract
    let update_minter_msg = Cw20ExecuteMsg::UpdateMinter {
        new_minter: Some(dira_contract_addr.to_string()),
    };
    let res = app.execute_contract(
        Addr::unchecked("cosmwasm1qypqxpq9qcrsszgszyfpx9q4zct3sxfqx5vwjh"),
        cw20_contract_addr.clone(),
        &update_minter_msg,
        &[],
    );
    assert!(res.is_ok());
    dbg!("Updated CW20 token minter to Dira contract");

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
fn test_mint_burn_dira() {
    let (mut app, dira_contract_addr, cw20_contract_addr, admin, non_admin) = setup_app();

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

    dbg!("Successfully set up mint/burn scenario with collateral locked");

    // Mint DIRA for admin
    let mint_dira_msg = DiraExecuteMsg::MintDira {
        dira_to_mint: Decimal::from_atomics(1_000u128, 6).unwrap(),
    };
    let res = app.execute_contract(
        admin.clone(),
        dira_contract_addr.clone(),
        &mint_dira_msg,
        &[],
    );
    assert!(res.is_ok());
    dbg!("Minted DIRA for admin");

    // Query admin's balance of CW20 DIRA
    let balance_query = cw20::Cw20QueryMsg::Balance {
        address: admin.to_string(),
    };
    let balance: cw20::BalanceResponse = app
        .wrap()
        .query_wasm_smart(cw20_contract_addr.clone(), &balance_query)
        .unwrap();
    assert_eq!(balance.balance, Uint128::new(1_000));
    dbg!("Admin's balance of DIRA:", balance.balance);

    let increase_allowance_msg = Cw20ExecuteMsg::IncreaseAllowance {
        spender: dira_contract_addr.to_string(),
        amount: Uint128::from(500u128), // Approve 500 DIRA tokens
        expires: None,                  // No expiration
    };

    let res = app.execute_contract(
        admin.clone(),              // The user granting the approval
        cw20_contract_addr.clone(), // The CW20 token contract
        &increase_allowance_msg,    // The IncreaseAllowance message
        &[],                        // No funds required
    );
    assert!(res.is_ok());
    dbg!("Successfully granted allowance to Dira contract!");

    // Burn DIRA from admin
    let burn_dira_msg = DiraExecuteMsg::BurnDira {
        dira_to_burn: Decimal::from_atomics(500u128, 6).unwrap(),
    };
    let res = app.execute_contract(
        admin.clone(),
        dira_contract_addr.clone(),
        &burn_dira_msg,
        &[],
    );
    dbg!(&res);
    assert!(res.is_ok());
    dbg!("Burnt 500 DIRA from admin");

    // Query admin's balance of CW20 DIRA after burning
    let balance: cw20::BalanceResponse = app
        .wrap()
        .query_wasm_smart(cw20_contract_addr.clone(), &balance_query)
        .unwrap();
    assert_eq!(balance.balance, Uint128::new(500));
    dbg!("Admin's balance of DIRA after burning:", balance.balance);

    // Mint DIRA for non-admin
    let mint_dira_msg = DiraExecuteMsg::MintDira {
        dira_to_mint: Decimal::from_atomics(500u128, 6).unwrap(),
    };
    let res = app.execute_contract(
        non_admin.clone(),
        dira_contract_addr.clone(),
        &mint_dira_msg,
        &[],
    );
    assert!(res.is_ok());
    dbg!("Minted DIRA for non-admin");

    // Query non-admin's balance of CW20 DIRA
    let non_admin_balance_query = cw20::Cw20QueryMsg::Balance {
        address: non_admin.to_string(),
    };
    let balance: cw20::BalanceResponse = app
        .wrap()
        .query_wasm_smart(cw20_contract_addr.clone(), &non_admin_balance_query)
        .unwrap();
    assert_eq!(balance.balance, Uint128::new(500));
    dbg!("Non-admin's balance of DIRA:", balance.balance);

    let increase_allowance_msg = Cw20ExecuteMsg::IncreaseAllowance {
        spender: dira_contract_addr.to_string(),
        amount: Uint128::from(250u128), // Approve 500 DIRA tokens
        expires: None,                  // No expiration
    };

    let res = app.execute_contract(
        non_admin.clone(),          // The user granting the approval
        cw20_contract_addr.clone(), // The CW20 token contract
        &increase_allowance_msg,    // The IncreaseAllowance message
        &[],                        // No funds required
    );
    assert!(res.is_ok());
    dbg!("Successfully granted allowance to Dira contract!");

    // Burn DIRA from non-admin
    let burn_dira_msg = DiraExecuteMsg::BurnDira {
        dira_to_burn: Decimal::from_atomics(250u128, 6).unwrap(),
    };
    let res = app.execute_contract(
        non_admin.clone(),
        dira_contract_addr.clone(),
        &burn_dira_msg,
        &[],
    );
    assert!(res.is_ok());
    dbg!("Burned 250 DIRA from non-admin");

    // Query non-admin's balance of CW20 DIRA after burning
    let balance: cw20::BalanceResponse = app
        .wrap()
        .query_wasm_smart(cw20_contract_addr.clone(), &non_admin_balance_query)
        .unwrap();
    assert_eq!(balance.balance, Uint128::new(250));
    dbg!(
        "Non-admin's balance of DIRA after burning:",
        balance.balance
    );
}

#[test]
fn test_liquidate_collateral() {
    let (mut app, dira_contract_addr, _cw20_contract_addr, admin, user) = setup_app();

    // 1. Setup the environment
    // Step 1.1: Set collateral price
    let set_collateral_price_msg = DiraExecuteMsg::SetCollateralPriceInDirham {
        collateral_price_in_dirham: Decimal::from_ratio(3309u128, 100u128), // 33.09
    };
    let res = app.execute_contract(
        admin.clone(),
        dira_contract_addr.clone(),
        &set_collateral_price_msg,
        &[],
    );
    assert!(res.is_ok());
    dbg!("Set collateral price to 33.09");

    // Step 1.2: Lock collateral from both admin and user
    let lock_collateral_msg = DiraExecuteMsg::LockCollateral {};

    let res = app.execute_contract(
        admin.clone(),
        dira_contract_addr.clone(),
        &lock_collateral_msg,
        &coins(1_000_000, "uatom"), // Admin locks 1 atom
    );
    assert!(res.is_ok());
    dbg!("Locked 1 atom collateral from admin");

    let res = app.execute_contract(
        user.clone(),
        dira_contract_addr.clone(),
        &lock_collateral_msg,
        &coins(1_000_000, "uatom"), // User locks 1 atom
    );
    assert!(res.is_ok());
    dbg!("Locked 1 atom collateral from user");

    // Step 1.3: Mint DIRA for both users
    let mint_dira_msg = DiraExecuteMsg::MintDira {
        dira_to_mint: Decimal::from_ratio(1000000u128, 100000u128),
    };

    let res = app.execute_contract(
        admin.clone(),
        dira_contract_addr.clone(),
        &mint_dira_msg,
        &[],
    );
    dbg!(&res);
    assert!(res.is_ok());
    dbg!("Admin minted 1 DIRA");

    let res = app.execute_contract(
        user.clone(),
        dira_contract_addr.clone(),
        &mint_dira_msg,
        &[],
    );
    assert!(res.is_ok());
    dbg!("User minted 1 DIRA");

    // Step 1.4: Verify state before price drop
    dbg!("State before price drop:");
    dbg!(app.wrap().query_balance(&admin, "uatom").unwrap());
    dbg!(app.wrap().query_balance(&user, "uatom").unwrap());

    // 2. Test liquidation due to price drop
    // Step 2.1: Drop collateral price
    let set_low_collateral_price_msg = DiraExecuteMsg::SetCollateralPriceInDirham {
        collateral_price_in_dirham: Decimal::from_ratio(1000u128, 100u128), // Price drops to 10.00
    };
    let res = app.execute_contract(
        admin.clone(),
        dira_contract_addr.clone(),
        &set_low_collateral_price_msg,
        &[],
    );
    assert!(res.is_ok());
    dbg!("Collateral price dropped to 10.00");

    // Step 2.2: Attempt to liquidate admin from user account
    let liquidate_admin_msg = DiraExecuteMsg::LiquidateStablecoins {
        wallet_address_to_liquidate: admin.clone(),
    };
    let res = app.execute_contract(
        user.clone(),
        dira_contract_addr.clone(),
        &liquidate_admin_msg,
        &[],
    );
    dbg!(&res);
    assert!(res.is_ok());
    dbg!("Admin successfully liquidated by user");

    // Step 2.3: Verify state after admin liquidation
    dbg!("State after admin liquidation:");
    dbg!(app.wrap().query_balance(&admin, "uatom").unwrap());
    dbg!(app.wrap().query_balance(&user, "uatom").unwrap());

    // 3. Test liquidation of user from admin account
    // Step 3.1: Drop collateral price further
    let set_lower_collateral_price_msg = DiraExecuteMsg::SetCollateralPriceInDirham {
        collateral_price_in_dirham: Decimal::from_ratio(500u128, 100u128), // Price drops to 5.00
    };
    let res = app.execute_contract(
        admin.clone(),
        dira_contract_addr.clone(),
        &set_lower_collateral_price_msg,
        &[],
    );
    assert!(res.is_ok());
    dbg!("Collateral price dropped to 5.00");

    // Step 3.2: Attempt to liquidate user from admin account
    let liquidate_user_msg = DiraExecuteMsg::LiquidateStablecoins {
        wallet_address_to_liquidate: user.clone(),
    };
    let res = app.execute_contract(
        admin.clone(),
        dira_contract_addr.clone(),
        &liquidate_user_msg,
        &[],
    );
    assert!(res.is_ok());
    dbg!("User successfully liquidated by admin");

    // Step 3.3: Verify state after user liquidation
    dbg!("State after user liquidation:");
    dbg!(app.wrap().query_balance(&admin, "uatom").unwrap());
    dbg!(app.wrap().query_balance(&user, "uatom").unwrap());

    // 4. Edge Case: Attempt liquidation when health is above threshold
    let invalid_liquidation_msg = DiraExecuteMsg::LiquidateStablecoins {
        wallet_address_to_liquidate: admin.clone(),
    };
    let res = app.execute_contract(
        user.clone(),
        dira_contract_addr.clone(),
        &invalid_liquidation_msg,
        &[],
    );
    dbg!(&res);
    assert!(res.is_err());
    dbg!("Liquidation failed as admin's health is above threshold");

    // 5. Edge Case: Liquidation of a wallet with no minted DIRA
    let invalid_liquidation_msg = DiraExecuteMsg::LiquidateStablecoins {
        wallet_address_to_liquidate: Addr::unchecked("cosmos1no_minterxxxxxxxxxxxxxx"),
    };
    let res = app.execute_contract(
        admin.clone(),
        dira_contract_addr.clone(),
        &invalid_liquidation_msg,
        &[],
    );
    assert!(res.is_err());
    dbg!("Liquidation failed for wallet with no minted DIRA");

    // 6. Edge Case: Liquidation attempt on a non-existing user
    let non_existing_user_msg = DiraExecuteMsg::LiquidateStablecoins {
        wallet_address_to_liquidate: Addr::unchecked("cosmos1nonexistentxxxxxxxxxxx"),
    };
    let res = app.execute_contract(
        admin.clone(),
        dira_contract_addr.clone(),
        &non_existing_user_msg,
        &[],
    );
    assert!(res.is_err());
    dbg!("Liquidation failed for non-existing user");
}
