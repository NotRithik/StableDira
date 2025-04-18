// stable_dira_stubs.qnt
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

  
  pure val ADDRESSES = Set("sender1", "sender2", "sender3", CONTRACT_ADDRESS)
  // Assuming 'uatom' is the primary collateral denom based on tests/usage
  pure val DENOMS = Set("uatom", "dira_cw20_placeholder") // Added placeholder for CW20 Dira
  pure val MAX_AMOUNT = 1000000 // Increased based on typical token amounts

  // Types translated from src/msg.rs and src/state.rs
  type InstantiateMsg = {
    liquidation_health: int, // Decimal -> int
    mintable_health: int,    // Decimal -> int
    collateral_token_denom: str,
    cw20_dira_contract_address: Option[Addr],
  }

  type ExecuteMsg =
    | ExecuteMsg_LockCollateral
    | ExecuteMsg_UnlockCollateral({ collateral_amount_to_unlock: int }) // Decimal -> int
    | ExecuteMsg_MintDira({ dira_to_mint: int }) // Decimal -> int
    | ExecuteMsg_BurnDira({ dira_to_burn: int })   // Decimal -> int
    | ExecuteMsg_LiquidateStablecoins({ wallet_address_to_liquidate: Addr })
    | ExecuteMsg_SetCollateralPriceInDirham({ collateral_price_in_dirham: int }) // Decimal -> int
    | ExecuteMsg_SetLiquidationHealth({ liquidation_health: int }) // Decimal -> int
    | ExecuteMsg_SetMintableHealth({ mintable_health: int })       // Decimal -> int
    | ExecuteMsg_SetCW20DiraContractAddress({ cw20_dira_contract_address: Addr })

  // --- Function Stubs ---

  pure def instantiate(state: ContractState, _env: Env, info: MessageInfo, msg: InstantiateMsg): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:instantiate
    (Ok(Response_new.add_attribute("action", FromStr("instantiate"))), state)
  }

  pure def execute(state: ContractState, _env: Env, info: MessageInfo, msg: ExecuteMsg): (Result[Response, ContractError], ContractState) = match msg {
    | ExecuteMsg_LockCollateral(__r) => lock_collateral(state, info)
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
    (Ok(Response_new), state)
  }
                            
  action lock_collateral_action = {
    // TODO: Change next line according to fund expectations (expecting collateral token denom)
    pure val max_funds = MAX_AMOUNT 
    
    pure val message: ExecuteMsg = ExecuteMsg_LockCollateral
    execute_message(message, max_funds)
  }

  pure def unlock_collateral(state: ContractState, info: MessageInfo, collateral_amount_to_unlock: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_unlock_collateral
    (Ok(Response_new), state)
  }
                            
  action unlock_collateral_action = {
    // TODO: Change next line according to fund expectations (usually 0 for unlock)
    pure val max_funds = 0 
    nondet message_collateral_amount_to_unlock: int = 0.to(MAX_AMOUNT).oneOf()
    pure val message: ExecuteMsg = ExecuteMsg_UnlockCollateral({ collateral_amount_to_unlock: message_collateral_amount_to_unlock })
    execute_message(message, max_funds)
  }

  pure def mint_dira(state: ContractState, info: MessageInfo, dira_to_mint: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_mint_dira
    (Ok(Response_new), state)
  }
                            
  action mint_dira_action = {
    // TODO: Change next line according to fund expectations (usually 0 for mint)
    pure val max_funds = 0
    nondet message_dira_to_mint: int = 0.to(MAX_AMOUNT).oneOf()
    pure val message: ExecuteMsg = ExecuteMsg_MintDira({ dira_to_mint: message_dira_to_mint })
    execute_message(message, max_funds)
  }

  pure def burn_dira(state: ContractState, info: MessageInfo, dira_to_burn: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_burn_dira
    (Ok(Response_new), state)
  }
                            
  action burn_dira_action = {
    // TODO: Change next line according to fund expectations (expecting CW20 transfer/burn approval)
    pure val max_funds = 0 
    nondet message_dira_to_burn: int = 0.to(MAX_AMOUNT).oneOf()
    pure val message: ExecuteMsg = ExecuteMsg_BurnDira({ dira_to_burn: message_dira_to_burn })
    execute_message(message, max_funds)
  }

  pure def liquidate_stablecoins(state: ContractState, info: MessageInfo, wallet_address_to_liquidate: Addr): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_liquidate_stablecoin_minter
    (Ok(Response_new), state)
  }
                            
  action liquidate_stablecoins_action = {
    // TODO: Change next line according to fund expectations (usually 0)
    pure val max_funds = 0
    nondet message_wallet_address_to_liquidate: Addr = ADDRESSES.oneOf() // Choose an address to liquidate
    pure val message: ExecuteMsg = ExecuteMsg_LiquidateStablecoins({ wallet_address_to_liquidate: message_wallet_address_to_liquidate })
    execute_message(message, max_funds)
  }

  pure def set_collateral_price_in_dirham(state: ContractState, info: MessageInfo, collateral_price_in_dirham: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_set_collateral_price_in_dirham
    (Ok(Response_new), state)
  }
                            
  action set_collateral_price_in_dirham_action = {
    // TODO: Change next line according to fund expectations (0 for admin actions)
    pure val max_funds = 0
    nondet message_collateral_price_in_dirham: int = 1.to(MAX_AMOUNT * 100).oneOf() // Price > 0
    pure val message: ExecuteMsg = ExecuteMsg_SetCollateralPriceInDirham({ collateral_price_in_dirham: message_collateral_price_in_dirham })
    execute_message(message, max_funds)
  }

  pure def set_liquidation_health(state: ContractState, info: MessageInfo, liquidation_health: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_set_liquidation_health
    (Ok(Response_new), state)
  }
                            
  action set_liquidation_health_action = {
    // TODO: Change next line according to fund expectations (0 for admin actions)
    pure val max_funds = 0
    nondet message_liquidation_health: int = 1.to(MAX_AMOUNT).oneOf() // Health > 0
    pure val message: ExecuteMsg = ExecuteMsg_SetLiquidationHealth({ liquidation_health: message_liquidation_health })
    execute_message(message, max_funds)
  }

  pure def set_mintable_health(state: ContractState, info: MessageInfo, mintable_health: int): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_set_mintable_health
    (Ok(Response_new), state)
  }
                            
  action set_mintable_health_action = {
    // TODO: Change next line according to fund expectations (0 for admin actions)
    pure val max_funds = 0
    nondet message_mintable_health: int = 1.to(MAX_AMOUNT).oneOf() // Health > 0
    pure val message: ExecuteMsg = ExecuteMsg_SetMintableHealth({ mintable_health: message_mintable_health })
    execute_message(message, max_funds)
  }

  pure def set_cw20_dira_contract_address(state: ContractState, info: MessageInfo, cw20_dira_contract_address: Addr): (Result[Response, ContractError], ContractState) = {
    // TODO: Update body based on src/contract.rs:execute_set_cw20_dira_contact_address
    (Ok(Response_new), state)
  }
                            
  action set_cw20_dira_contract_address_action = {
    // TODO: Change next line according to fund expectations (0 for admin actions)
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
    liquidation_health: int,    // Decimal -> int
    mintable_health: int,       // Decimal -> int
    locked_collateral: Addr -> int, // Decimal -> int
    minted_dira: Addr -> int,       // Decimal -> int
    collateral_token_price: int, // Decimal -> int
    cw20_dira_contract_address: Addr, // Assuming it gets set, might need Option[Addr]
  }

  // Initial state values
  pure val init_contract_state: ContractState = {
    collateral_token_denom: "", // Set during instantiate
    admin_addresses: [],        // Set during instantiate
    liquidation_health: 0,      // Set during instantiate
    mintable_health: 0,         // Set during instantiate
    locked_collateral: Map(),
    minted_dira: Map(),
    collateral_token_price: 0, // Needs to be set by admin
    cw20_dira_contract_address: "", // Needs to be set by admin or instantiate
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

  val env_val = { block: { time: time, height: 1 } } // TODO: Add a height var if you need it

  action init = {
    // TODO: Fill in InstantiateMsg fields nondeterministically or with defaults
    pure val max_funds = 0 // Instantiate usually takes no funds

    nondet sender = Set("admin").oneOf() // Initial admin
    nondet denom = DENOMS.oneOf()        // Not relevant for instantiate usually
    nondet amount = 0.to(max_funds).oneOf()
    val funds = [{ denom: denom, amount: amount }]
    val info = { sender: sender, funds: funds }

    // Choose reasonable initial values or make them nondeterministic
    nondet init_liquidation_health: int = 110.to(150).oneOf()
    nondet init_mintable_health: int = 130.to(200).oneOf() // Ensure > liquidation
    nondet init_collateral_token_denom: str = Set("uatom").oneOf()
    nondet init_cw20_addr_opt: Option[Addr] = Set(Some("cw20_addr_placeholder"), None).oneOf() // Placeholder

    // Ensure mintable > liquidation if both are nondet
    assume(init_mintable_health > init_liquidation_health)

    pure val message: InstantiateMsg = {
      liquidation_health: init_liquidation_health,
      mintable_health: init_mintable_health,
      collateral_token_denom: init_collateral_token_denom,
      cw20_dira_contract_address: init_cw20_addr_opt,
    }
    // Use init_contract_state as the base, instantiate modifies it
    pure val r = instantiate(init_contract_state, { block: { time: 0, height: 1 } }, info, message)

    all {
      contract_state' = r._2,
      bank' = init_bank_state,
      result' = r._1,
      time' = 0,
    }
  }


  action execute_message(message, max_funds) = {
    nondet sender = ADDRESSES.oneOf()
    nondet denom = DENOMS.oneOf()
    nondet amount = 0.to(max_funds).oneOf()
    val funds = [{ denom: denom, amount: amount }]
    val info = { sender: sender, funds: funds }

    val r = execute(contract_state, env_val, info, message)
    all {
      // Basic bank check, might need refinement based on action
      bank.get(sender).get(denom) >= amount,
      match r._1 {
        // Basic bank update, needs refinement for unlocks/burns etc.
        | Ok(resp) => {
             // Simple model: assume funds go to contract on success unless it's an unlock/burn
             // TODO: Refine based on actual message effects (BankMsg_Send etc in response)
             bank' = bank.setBy(sender, balances => balances.setBy(denom, balance => balance - amount))
                               .setBy(CONTRACT_ADDRESS, balances => balances.setBy(denom, balance => balance + amount))
        }
        | Err(_) => bank' = bank // No bank change on error
      },
      result' = r._1,
      contract_state' = r._2,
    }
  }

  action advance_time = time' = time + 1

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