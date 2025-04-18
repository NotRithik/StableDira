// quint/stable_dira_stubs.qnt
// -*- mode: Bluespec; -*-
module stable_dira {

  import basicSpells.* from "./lib/basicSpells"
  import cw_types.* from "./lib/cw_types"
  import cw_utils.* from "./lib/cw_utils"
  import messaging.* from "./lib/messaging"
  import bank from "./lib/bank"


  var contract_state: ContractState
  var result: Result
  var bank: bank::Bank
  var time: int


  pure val CONTRACT_ADDRESS = "contract0"


  pure val ADDRESSES = Set("sender1", "sender2", "sender3", "admin", CONTRACT_ADDRESS)
  // Assuming 'uatom' is the primary collateral denom based on tests/usage
  // DIRA itself is a CW20 token, not tracked directly in the native bank module here.
  pure val DENOMS = Set("uatom")
  // Using a larger max amount due to Decimal usage often involving large atomics
  pure val MAX_AMOUNT = 100000000000000 // Represents 100 ATOM if decimals=6

  // Types translated from src/msg.rs and src/state.rs
  // Note: Decimal is translated to int for Quint modelling simplicity.
  // Precision needs to be handled manually in the logic if required.
  type InstantiateMsg = {
    liquidation_health: int,
    mintable_health: int,
    collateral_token_denom: str,
    cw20_dira_contract_address: Option[Addr],
  }

  type ExecuteMsg =
    | ExecuteMsg_LockCollateral
    | ExecuteMsg_UnlockCollateral({ collateral_amount_to_unlock: int })
    | ExecuteMsg_MintDira({ dira_to_mint: int })
    | ExecuteMsg_BurnDira({ dira_to_burn: int })
    | ExecuteMsg_LiquidateStablecoins({ wallet_address_to_liquidate: Addr })
    | ExecuteMsg_SetCollateralPriceInDirham({ collateral_price_in_dirham: int })
    | ExecuteMsg_SetLiquidationHealth({ liquidation_health: int })
    | ExecuteMsg_SetMintableHealth({ mintable_health: int })
    | ExecuteMsg_SetCW20DiraContractAddress({ cw20_dira_contract_address: Addr })

  // --- Function Stubs ---

  pure def instantiate(state: ContractState, _env: Env, info: MessageInfo, msg: InstantiateMsg): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:instantiate logic
    //       - Check health > 0, mintable > liquidation
    //       - Check denom not empty
    //       - Validate CW20 address if Some
    //       - Save state variables: ADMIN_ADDRESSES=[info.sender], LIQUIDATION_HEALTH, MINTABLE_HEALTH, COLLATERAL_TOKEN_DENOM, CW20_DIRA_CONTRACT_ADDRESS
    val initial_state: ContractState = {
        ...init_contract_state, // Use defaults as base
        admin_addresses: [info.sender],
        liquidation_health: msg.liquidation_health,
        mintable_health: msg.mintable_health,
        collateral_token_denom: msg.collateral_token_denom,
        cw20_dira_contract_address: msg.cw20_dira_contract_address,
    }
    (Ok(Response_new.add_attribute("action", FromStr("instantiate"))
                    .add_attribute("admin", FromStr(info.sender))),
     initial_state) // Return the initialized state
  }

  pure def execute(state: ContractState, _env: Env, info: MessageInfo, msg: ExecuteMsg): (Result[Response, ContractError], ContractState) = match msg {
    | ExecuteMsg_LockCollateral => lock_collateral(state, info)
    | ExecuteMsg_UnlockCollateral(__r) => unlock_collateral(state, info, __r.collateral_amount_to_unlock)
    | ExecuteMsg_MintDira(__r) => mint_dira(state, info, __r.dira_to_mint)
    | ExecuteMsg_BurnDira(__r) => burn_dira(state, info, __r.dira_to_burn)
    | ExecuteMsg_LiquidateStablecoins(__r) => liquidate_stablecoins(state, info, __r.wallet_address_to_liquidate)
    | ExecuteMsg_SetCollateralPriceInDirham(__r) => set_collateral_price_in_dirham(state, info, __r.collateral_price_in_dirham)
    | ExecuteMsg_SetLiquidationHealth(__r) => set_liquidation_health(state, info, __r.liquidation_health)
    | ExecuteMsg_SetMintableHealth(__r) => set_mintable_health(state, info, __r.mintable_health)
    | ExecuteMsg_SetCW20DiraContractAddress(__r) => set_cw20_dira_contract_address(state, info, __r.cw20_dira_contract_address)
  }

  pure def lock_collateral(state: ContractState, info: MessageInfo): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_lock_collateral
    //       - Check funds sent match collateral_token_denom
    //       - Update LOCKED_COLLATERAL map
    (Ok(Response_new.add_attribute("action", FromStr("lock_collateral"))), state)
  }

  action lock_collateral_action = {
    // Expecting funds in the collateral token denom
    pure val max_funds = MAX_AMOUNT
    pure val message: ExecuteMsg = ExecuteMsg_LockCollateral
    // Override default denom for this action
    nondet sender = ADDRESSES.oneOf()
    nondet amount = 1.to(max_funds).oneOf() // Lock at least 1 unit
    val funds = [{ denom: contract_state.collateral_token_denom, amount: amount }]
    val info = { sender: sender, funds: funds }

    val r = execute(contract_state, env_val, info, message)
    all {
      // Bank check: User must have the funds they are sending
      bank.get(sender).get(contract_state.collateral_token_denom) >= amount,
      match r._1 {
        // Bank update: Funds move from sender to contract
        | Ok(_) => bank' = bank.setBy(sender, balances => balances.setBy(contract_state.collateral_token_denom, balance => balance - amount))
                               .setBy(CONTRACT_ADDRESS, balances => balances.setBy(contract_state.collateral_token_denom, balance => balance + amount))
        | Err(_) => bank' = bank // No bank change on error
      },
      result' = r._1,
      contract_state' = r._2,
    }
  }

  pure def unlock_collateral(state: ContractState, info: MessageInfo, collateral_amount_to_unlock: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_unlock_collateral
    //       - Check max_unlockable_collateral
    //       - Update LOCKED_COLLATERAL map
    //       - Add BankMsg::Send to response messages
    (Ok(Response_new.add_attribute("action", FromStr("unlock_collateral"))), state)
  }

  action unlock_collateral_action = {
    // Usually takes no funds
    pure val max_funds = 0
    nondet message_collateral_amount_to_unlock: int = 1.to(MAX_AMOUNT).oneOf() // Unlock at least 1
    pure val message: ExecuteMsg = ExecuteMsg_UnlockCollateral({ collateral_amount_to_unlock: message_collateral_amount_to_unlock })
    // This action sends funds FROM the contract, need refinement in execute_message or custom action logic
    execute_message(message, max_funds) // Basic execute_message might incorrectly add funds to contract
  }

  pure def mint_dira(state: ContractState, info: MessageInfo, dira_to_mint: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_mint_dira
    //       - Check max_mintable_dira
    //       - Update MINTED_DIRA map
    //       - Add WasmMsg::Execute Mint to response messages (needs CW20 address)
    (Ok(Response_new.add_attribute("action", FromStr("mint_dira"))), state)
  }

  action mint_dira_action = {
    // Usually takes no funds
    pure val max_funds = 0
    nondet message_dira_to_mint: int = 1.to(MAX_AMOUNT).oneOf() // Mint at least 1
    pure val message: ExecuteMsg = ExecuteMsg_MintDira({ dira_to_mint: message_dira_to_mint })
    execute_message(message, max_funds)
  }

  pure def burn_dira(state: ContractState, info: MessageInfo, dira_to_burn: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_burn_dira
    //       - Check dira_to_burn <= previously_minted_dira
    //       - Update MINTED_DIRA map
    //       - Add WasmMsg::Execute BurnFrom to response messages (needs CW20 address and approval)
    (Ok(Response_new.add_attribute("action", FromStr("burn_dira"))), state)
  }

  action burn_dira_action = {
    // This action implies a prior CW20 approval step, not modelled here. Assumes 0 funds needed directly.
    pure val max_funds = 0
    nondet message_dira_to_burn: int = 1.to(MAX_AMOUNT).oneOf() // Burn at least 1
    pure val message: ExecuteMsg = ExecuteMsg_BurnDira({ dira_to_burn: message_dira_to_burn })
    execute_message(message, max_funds)
  }

  pure def liquidate_stablecoins(state: ContractState, info: MessageInfo, wallet_address_to_liquidate: Addr): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_liquidate_stablecoin_minter
    //       - Calculate user_health
    //       - Check user_health < liquidation_health
    //       - Update LOCKED_COLLATERAL and MINTED_DIRA for liquidated user
    //       - Handle liquidator reward (currently 0 in Rust code)
    (Ok(Response_new.add_attribute("action", FromStr("liquidate_stablecoins"))), state)
  }

  action liquidate_stablecoins_action = {
    // Takes no funds
    pure val max_funds = 0
    nondet message_wallet_address_to_liquidate: Addr = ADDRESSES.oneOf() // Choose an address to liquidate
    pure val message: ExecuteMsg = ExecuteMsg_LiquidateStablecoins({ wallet_address_to_liquidate: message_wallet_address_to_liquidate })
    execute_message(message, max_funds)
  }

  // --- Admin Actions ---

  pure def set_collateral_price_in_dirham(state: ContractState, info: MessageInfo, collateral_price_in_dirham: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_set_collateral_price_in_dirham
    //       - Check info.sender is in admin_addresses
    //       - Update COLLATERAL_TOKEN_PRICE
    (Ok(Response_new.add_attribute("action", FromStr("set_collateral_price_in_dirham"))), state)
  }

  action set_collateral_price_in_dirham_action = {
    // Admin action, takes no funds
    pure val max_funds = 0
    nondet message_collateral_price_in_dirham: int = 1.to(MAX_AMOUNT * 100).oneOf() // Price > 0
    pure val message: ExecuteMsg = ExecuteMsg_SetCollateralPriceInDirham({ collateral_price_in_dirham: message_collateral_price_in_dirham })
    // Need to ensure sender is admin in execute_message or here
    execute_message(message, max_funds)
  }

  pure def set_liquidation_health(state: ContractState, info: MessageInfo, liquidation_health: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_set_liquidation_health
    //       - Check info.sender is in admin_addresses
    //       - Update LIQUIDATION_HEALTH
    (Ok(Response_new.add_attribute("action", FromStr("set_liquidation_health"))), state)
  }

  action set_liquidation_health_action = {
    // Admin action, takes no funds
    pure val max_funds = 0
    nondet message_liquidation_health: int = 1.to(MAX_AMOUNT).oneOf() // Health > 0
    pure val message: ExecuteMsg = ExecuteMsg_SetLiquidationHealth({ liquidation_health: message_liquidation_health })
    execute_message(message, max_funds)
  }

  pure def set_mintable_health(state: ContractState, info: MessageInfo, mintable_health: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_set_mintable_health
    //       - Check info.sender is in admin_addresses
    //       - Check mintable_health >= current liquidation_health
    //       - Update MINTABLE_HEALTH
    (Ok(Response_new.add_attribute("action", FromStr("set_mintable_health"))), state)
  }

  action set_mintable_health_action = {
    // Admin action, takes no funds
    pure val max_funds = 0
    nondet message_mintable_health: int = 1.to(MAX_AMOUNT).oneOf() // Health > 0
    pure val message: ExecuteMsg = ExecuteMsg_SetMintableHealth({ mintable_health: message_mintable_health })
    execute_message(message, max_funds)
  }

  pure def set_cw20_dira_contract_address(state: ContractState, info: MessageInfo, cw20_dira_contract_address: Addr): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_set_cw20_dira_contact_address
    //       - Check info.sender is in admin_addresses (Rust code missing this check!)
    //       - Validate address is CW20 (hard to model, assume valid or skip check)
    //       - Update CW20_DIRA_CONTRACT_ADDRESS
    (Ok(Response_new.add_attribute("action", FromStr("set_cw20_dira_contract_address"))), state)
  }

  action set_cw20_dira_contract_address_action = {
    // Admin action, takes no funds
    pure val max_funds = 0
    nondet message_cw20_dira_contract_address: Addr = ADDRESSES.oneOf() // Could be any address initially
    pure val message: ExecuteMsg = ExecuteMsg_SetCW20DiraContractAddress({ cw20_dira_contract_address: message_cw20_dira_contract_address })
    execute_message(message, max_funds)
  }

  // --- State Definition ---

  // Based on src/state.rs
  type ContractState = {
    collateral_token_denom: str,
    admin_addresses: List[Addr],
    liquidation_health: int,
    mintable_health: int,
    locked_collateral: Addr -> int, // Map<Addr, Decimal>
    minted_dira: Addr -> int,       // Map<Addr, Decimal>
    collateral_token_price: int,    // Item<Decimal>
    cw20_dira_contract_address: Option[Addr], // Item<Addr>, corrected to Option based on InstantiateMsg
  }

  // Initial state values based on logic (some set in instantiate)
  pure val init_contract_state: ContractState = {
    collateral_token_denom: "",
    admin_addresses: [],
    liquidation_health: 0,
    mintable_health: 0,
    locked_collateral: Map(),
    minted_dira: Map(),
    collateral_token_price: 0, // Assuming 0 means not set, requires admin action
    cw20_dira_contract_address: None, // Corrected initial state
  }

  // --- Execution Steps ---

  action execute_step = all {
    any {
      lock_collateral_action,
      unlock_collateral_action,
      mint_dira_action,
      burn_dira_action,
      liquidate_stablecoins_action,
      set_collateral_price_in_dirham_action,
      set_liquidation_health_action,
      set_mintable_health_action,
      set_cw20_dira_contract_address_action
    },
    advance_time,
  }

  // Default reply stub (Needed for boilerplate step action)
  pure def reply(state: ContractState, _env: Env, _reply: Reply): (Result[Response, ContractError], ContractState) = {
      (Ok(Response_new), state)
  }

  // --- Boilerplate Actions ---

  pure val init_bank_state = ADDRESSES.mapBy(_ => DENOMS.mapBy(_ => MAX_AMOUNT))

  val env_val = { block: { time: time, height: 1 } }

  action init = {
    pure val max_funds = 0 // Instantiate usually takes no funds

    nondet sender = Set("admin").oneOf() // Initial admin
    // Instantiate doesn't use funds field directly
    val info = { sender: sender, funds: [] }

    // Choose reasonable initial values or make them nondeterministic
    nondet init_liquidation_health: int = 110.to(150).oneOf() // Must be > 0
    nondet init_mintable_health: int = 130.to(200).oneOf()    // Must be > 0
    nondet init_collateral_token_denom: str = Set("uatom").oneOf() // Must not be empty
    nondet init_cw20_addr_opt: Option[Addr] = Set(Some("cw20_addr_placeholder"), None).oneOf()

    // Ensure crucial constraints from Rust instantiate
    assume(init_liquidation_health > 0)
    assume(init_mintable_health > 0)
    assume(init_mintable_health >= init_liquidation_health) // Rust check is <, Quint uses >=
    assume(init_collateral_token_denom != "")

    pure val message: InstantiateMsg = {
      liquidation_health: init_liquidation_health,
      mintable_health: init_mintable_health,
      collateral_token_denom: init_collateral_token_denom,
      cw20_dira_contract_address: init_cw20_addr_opt,
    }

    pure val r = instantiate(init_contract_state, { block: { time: 0, height: 1 } }, info, message)

    all {
      contract_state' = r._2, // State should be fully set by instantiate
      bank' = init_bank_state, // Bank state starts initialized
      result' = r._1,
      time' = 0,
    }
  }


  // Simplified bank logic - needs refinement based on actual message handling
  action execute_message(message, max_funds) = {
    nondet sender = ADDRESSES.oneOf()
    // Determine relevant denom based on message type if possible, else nondet
    nondet denom = DENOMS.oneOf()
    nondet amount = 0.to(max_funds).oneOf()
    val funds = [{ denom: denom, amount: amount }]
    val info = { sender: sender, funds: funds }

    // --- Pre-execution Checks (Optional but Recommended) ---
    // Example: Check if admin action is called by admin
    // val isAdmin = contract_state.admin_addresses.contains(sender)
    // val requiresAdmin = match message {
    // | ExecuteMsg_SetCollateralPriceInDirham(_) => true
    // | ExecuteMsg_SetLiquidationHealth(_) => true
    // | ExecuteMsg_SetMintableHealth(_) => true
    // | ExecuteMsg_SetCW20DiraContractAddress(_) => true // Rust code missing check!
    // | _ => false
    // }
    // assume(not(requiresAdmin) or isAdmin) // Abort trace if non-admin calls admin function


    val r = execute(contract_state, env_val, info, message)
    all {
      // Basic bank check (user must have funds they send)
      bank.get(sender).get(denom) >= amount,
      match r._1 {
        | Ok(resp) => {
            // Bank update logic: Needs to check resp.messages for BankMsg::Send
            // For simplicity, assume LockCollateral receives funds, others don't change native bank state directly.
            // UnlockCollateral sends funds *from* contract. Burn/Mint affect CW20 (not modelled in bank).
            // Liquidate might send rewards (not implemented yet).
             if (message == ExecuteMsg_LockCollateral) {
                bank' = bank.setBy(sender, balances => balances.setBy(denom, balance => balance - amount))
                               .setBy(CONTRACT_ADDRESS, balances => balances.setBy(denom, balance => balance + amount))
             } else {
                // More complex: Need to parse resp.messages for BankMsg_Send
                // For now, assume no change or handle UnlockCollateral case specifically
                bank' = bank
             }
          }
        | Err(_) => bank' = bank // No bank change on error
      },
      result' = r._1,
      contract_state' = r._2,
    }
  }

  action advance_time = time' = time + 1 // Basic time advancement

  action step = {
    val message_getting = get_message(result)
    val new_result = message_getting._1
    val opt_message = message_getting._2
    match opt_message {
      | Some(submsg) => {
          val current_state = { bank: bank, result: new_result, contract_state: contract_state }
          // process_message needs the reply function defined for the contract
          val new_state = process_message(current_state, env_val, CONTRACT_ADDRESS, submsg, reply)
          all {
            bank' = new_state.bank,
            result' = new_state.result,
            contract_state' = new_state.contract_state,
            advance_time, // Assume time advances even when processing submessages
          }
      }
      | None => execute_step // If no submessages, execute a top-level step
    }
  }

} // End of module stable_dira