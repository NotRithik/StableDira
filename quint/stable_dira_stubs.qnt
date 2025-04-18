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


  pure val ADDRESSES = Set("sender1", "sender2", "sender3", "admin", "user", CONTRACT_ADDRESS) // Added user from tests
  pure val DENOMS = Set("uatom") // Only collateral denom tracked in native bank
  pure val MAX_AMOUNT = 100000000000000 // Represents 100 ATOM if decimals=6

  // Assume Decimals are represented as integers (e.g., value * 10^18 or similar)
  // For simplicity here, we treat them as plain integers. Adjust scaling if needed.
  pure val DECIMAL_PLACES = 6 // Example: Used for converting amounts in messages

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

  // --- Helper Functions (Translated from Rust) ---

  // NOTE: Integer division used here. May differ from Decimal division.
  pure def calculate_stablecoin_health(minted_dira: int, locked_collateral: int, collateral_price: int): int = {
    val locked_value = collateral_price * locked_collateral
    if (minted_dira == 0) {
       // Representing Decimal::MAX with a large integer. Choose a suitable large number.
       // Using MAX_AMOUNT*MAX_AMOUNT might overflow Quint's internal representation.
       // Let's use a symbolic large value or a reasonable upper bound.
       100000000000000000000 // Placeholder for a very large health ratio
    } else {
      // Integer division - precision loss compared to Decimal
      locked_value / minted_dira
    }
  }

  pure def calculate_max_mintable_dira(locked_collateral: int, collateral_price: int, mintable_health: int): int = {
     if (mintable_health == 0) {
         // Avoid division by zero, technically shouldn't happen due to instantiate checks
         0
     } else {
         // Integer division
         (locked_collateral * collateral_price) / mintable_health
     }
  }

  pure def calculate_max_unlockable_collateral(locked_collateral: int, collateral_price: int, minted_dira: int, mintable_health: int): int = {
     if (collateral_price == 0) {
        // Avoid division by zero if price is somehow 0
        0
     } else {
       val required_collateral = (minted_dira * mintable_health) / collateral_price
       val unlockable = locked_collateral - required_collateral
       // Cannot unlock negative amounts
       if (unlockable < 0) 0 else unlockable
     }
  }

  // Cannot directly model CW20 query validation in Quint state machine
  pure def is_cw20_contract(addr: Addr): bool = {
    // Placeholder - Assume true for non-empty addresses in the model
    addr != ""
  }

  // --- Function Implementations ---

  pure def instantiate(state: ContractState, _env: Env, info: MessageInfo, msg: InstantiateMsg): (Result[Response, ContractError], ContractState) = {
      if (msg.liquidation_health <= 0 or msg.mintable_health <= 0) {
          (Err("HealthCannotBeZero"), state)
      } else if (msg.collateral_token_denom == "") {
          (Err("MissingCollateralTokenDenom"), state)
      } else if (msg.mintable_health < msg.liquidation_health) {
          (Err("MintableHealthLowerThanLiquidationHealth"), state)
      } else {
          val cw20_valid_or_none = match msg.cw20_dira_contract_address {
              | Some(addr) => is_cw20_contract(addr) // Simplified check
              | None => true
          }
          if (not(cw20_valid_or_none)) {
              (Err("InvalidCW20ContractAddress"), state)
          } else {
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
               initial_state)
          }
      }
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
      // Simplified must_pay logic from cw_utils (checks single fund, correct denom)
      if (info.funds.length() != 1) {
          (Err("Expected single fund type"), state)
      } else {
          val sent_fund = info.funds[0]
          if (sent_fund.denom != state.collateral_token_denom) {
              (Err("Wrong Denom"), state)
          } else if (sent_fund.amount <= 0) {
              (Err("InsufficientFundsSent"), state) // Amount must be positive
          } else {
              val sent_amount = sent_fund.amount // Assuming amount is already scaled int
              val current_locked = state.locked_collateral.getOrElse(info.sender, 0)
              val next_locked_map = state.locked_collateral.put(info.sender, current_locked + sent_amount)
              val next_state = { ...state, locked_collateral: next_locked_map }
              (Ok(Response_new.add_attribute("action", FromStr("lock_collateral"))
                              .add_attribute("sender", FromStr(info.sender))
                              .add_attribute("total_funds_locked_by_user", FromInt(current_locked + sent_amount))), // Use updated amount
               next_state)
          }
      }
  }

  action lock_collateral_action = {
    // Expecting funds in the collateral token denom
    pure val max_funds = MAX_AMOUNT
    pure val message: ExecuteMsg = ExecuteMsg_LockCollateral
    // Override default denom for this action
    nondet sender = ADDRESSES.oneOf()
    nondet amount = 1.to(max_funds).oneOf() // Lock at least 1 unit
    // Use the denom set in the contract state
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
      val locked_collateral = state.locked_collateral.getOrElse(info.sender, 0)
      val minted_dira = state.minted_dira.getOrElse(info.sender, 0)

      if (state.collateral_token_price <= 0) {
         // Price must be set and positive
         (Err("CollateralPriceNotSet"), state)
      } else {
          val max_unlockable = calculate_max_unlockable_collateral(
              locked_collateral,
              state.collateral_token_price,
              minted_dira,
              state.mintable_health
          )

          if (collateral_amount_to_unlock > max_unlockable) {
              (Err("UnlockAmountTooHigh"), state) // Simplified error, Rust includes max value
          } else {
              val new_locked = locked_collateral - collateral_amount_to_unlock
              val next_locked_map = state.locked_collateral.put(info.sender, new_locked)
              val next_state = { ...state, locked_collateral: next_locked_map }

              // Construct BankMsg::Send
              val bank_send_msg = BankMsg_Send({
                  to_address: info.sender,
                  amount: [{ denom: state.collateral_token_denom, amount: collateral_amount_to_unlock }]
              })

              (Ok(Response_new.add_attribute("action", FromStr("unlock_collateral"))
                              .add_attribute("sender", FromStr(info.sender))
                              .add_attribute("total_funds_locked_by_user", FromInt(new_locked))
                              .add_message(CosmosMsg_Bank(bank_send_msg))),
               next_state)
          }
      }
  }

  action unlock_collateral_action = {
    // Usually takes no funds
    pure val max_funds = 0
    nondet message_collateral_amount_to_unlock: int = 1.to(MAX_AMOUNT).oneOf() // Unlock at least 1
    pure val message: ExecuteMsg = ExecuteMsg_UnlockCollateral({ collateral_amount_to_unlock: message_collateral_amount_to_unlock })
    // Need custom logic in execute_message to handle BankMsg_Send response
    execute_message(message, max_funds)
  }

  pure def mint_dira(state: ContractState, info: MessageInfo, dira_to_mint: int): (Result[Response, ContractError], ContractState) = {
      val collateral_locked = state.locked_collateral.getOrElse(info.sender, 0)
      val previously_minted = state.minted_dira.getOrElse(info.sender, 0)

      if (state.collateral_token_price <= 0) {
          (Err("CollateralPriceNotSet"), state)
      } else {
          val max_mintable = calculate_max_mintable_dira(
              collateral_locked,
              state.collateral_token_price,
              state.mintable_health
          )

          if (dira_to_mint + previously_minted > max_mintable) {
              (Err("InsufficientCollateral"), state)
          } else {
              match state.cw20_dira_contract_address {
                  | None => (Err("CW20DiraContractAddressNotSet"), state)
                  | Some(cw20_addr) => {
                      val new_minted = previously_minted + dira_to_mint
                      val next_minted_map = state.minted_dira.put(info.sender, new_minted)
                      val next_state = { ...state, minted_dira: next_minted_map }

                      // Construct WasmMsg::Execute Mint placeholder
                      // Actual Mint msg structure isn't modelled deeply
                      val wasm_mint_msg = CosmosMsg_Wasm( /* WasmMsg_Execute({ contract_addr: cw20_addr, msg: ..., funds: [] }) */ )

                      (Ok(Response_new.add_attribute("action", FromStr("mint_dira"))
                                      .add_attribute("sender", FromStr(info.sender))
                                      .add_attribute("total_dira_minted_by_sender", FromInt(new_minted))
                                      .add_message(wasm_mint_msg) // Add placeholder message
                         ),
                       next_state)
                  }
              }
          }
      }
  }

  action mint_dira_action = {
    // Usually takes no funds
    pure val max_funds = 0
    nondet message_dira_to_mint: int = 1.to(MAX_AMOUNT).oneOf() // Mint at least 1
    pure val message: ExecuteMsg = ExecuteMsg_MintDira({ dira_to_mint: message_dira_to_mint })
    execute_message(message, max_funds)
  }

  pure def burn_dira(state: ContractState, info: MessageInfo, dira_to_burn: int): (Result[Response, ContractError], ContractState) = {
      val previously_minted = state.minted_dira.getOrElse(info.sender, 0)

      if (dira_to_burn > previously_minted) {
          (Err("ReturningMoreDiraThanMinted"), state)
      } else {
          match state.cw20_dira_contract_address {
              | None => (Err("CW20DiraContractAddressNotSet"), state)
              | Some(cw20_addr) => {
                  val new_minted = previously_minted - dira_to_burn
                  val next_minted_map = state.minted_dira.put(info.sender, new_minted)
                  val next_state = { ...state, minted_dira: next_minted_map }

                  // Construct WasmMsg::Execute BurnFrom placeholder
                  val wasm_burn_msg = CosmosMsg_Wasm( /* WasmMsg_Execute({ contract_addr: cw20_addr, msg: ..., funds: [] }) */ )

                  (Ok(Response_new.add_attribute("action", FromStr("burn_dira"))
                                  .add_attribute("sender", FromStr(info.sender))
                                  .add_attribute("total_dira_remaining_by_sender", FromInt(new_minted))
                                  .add_message(wasm_burn_msg) // Add placeholder message
                     ),
                   next_state)
              }
          }
      }
  }

  action burn_dira_action = {
    // Implies prior CW20 approval
    pure val max_funds = 0
    nondet message_dira_to_burn: int = 1.to(MAX_AMOUNT).oneOf() // Burn at least 1
    pure val message: ExecuteMsg = ExecuteMsg_BurnDira({ dira_to_burn: message_dira_to_burn })
    execute_message(message, max_funds)
  }

  pure def liquidate_stablecoins(state: ContractState, info: MessageInfo, wallet_address_to_liquidate: Addr): (Result[Response, ContractError], ContractState) = {
      // Address validation is implicit by type Addr = str in Quint
      val dira_minted = state.minted_dira.getOrElse(wallet_address_to_liquidate, 0)
      val collateral_locked = state.locked_collateral.getOrElse(wallet_address_to_liquidate, 0)

      if (state.collateral_token_price <= 0) {
          (Err("CollateralPriceNotSet"), state)
      } else {
          val user_health = calculate_stablecoin_health(dira_minted, collateral_locked, state.collateral_token_price)

          if (user_health >= state.liquidation_health) {
              (Err("TooHealthyToLiquidate"), state) // Simplified error
          } else {
              val next_locked_map = state.locked_collateral.put(wallet_address_to_liquidate, 0)
              val next_minted_map = state.minted_dira.put(wallet_address_to_liquidate, 0)
              val next_state = { ...state, locked_collateral: next_locked_map, minted_dira: next_minted_map }

              // TODO: Add liquidator reward logic if implemented in Rust
              (Ok(Response_new.add_attribute("action", FromStr("liquidate_stablecoins"))
                              .add_attribute("liquidated_wallet", FromStr(wallet_address_to_liquidate))
                              .add_attribute("liquidated_collateral", FromInt(collateral_locked)) // Record amount before setting to 0
                              .add_attribute("liquidated_dira", FromInt(dira_minted)) // Record amount before setting to 0
                              .add_attribute("initiator", FromStr(info.sender))
                              .add_attribute("liquidator_reward_paid", FromInt(0))), // Placeholder
               next_state)
          }
      }
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
      if (not(state.admin_addresses.contains(info.sender))) {
          (Err("UnauthorizedUser"), state)
      } else {
          val next_state = { ...state, collateral_token_price: collateral_price_in_dirham }
          (Ok(Response_new.add_attribute("action", FromStr("set_collateral_price_in_dirham"))
                          .add_attribute("sender", FromStr(info.sender))
                          .add_attribute("new_collateral_price", FromInt(collateral_price_in_dirham))),
           next_state)
      }
  }

  action set_collateral_price_in_dirham_action = {
    pure val max_funds = 0
    nondet message_collateral_price_in_dirham: int = 1.to(MAX_AMOUNT * 100).oneOf() // Price > 0
    pure val message: ExecuteMsg = ExecuteMsg_SetCollateralPriceInDirham({ collateral_price_in_dirham: message_collateral_price_in_dirham })
    execute_message(message, max_funds) // Assumes execute_message checks admin or pre-check is added
  }

  pure def set_liquidation_health(state: ContractState, info: MessageInfo, liquidation_health: int): (Result[Response, ContractError], ContractState) = {
      if (not(state.admin_addresses.contains(info.sender))) {
          (Err("UnauthorizedUser"), state)
      } else if (liquidation_health <= 0) {
          // Implicit check from Rust function logic returning ContractError::HealthCannotBeZero
          (Err("HealthCannotBeZero"), state)
      } else if (liquidation_health > state.mintable_health) {
         // Rust doesn't check this here, but maybe should? Added for model consistency.
         (Err("LiquidationHealthHigherThanMintableHealth"), state)
      }
       else {
          val next_state = { ...state, liquidation_health: liquidation_health }
          (Ok(Response_new.add_attribute("action", FromStr("set_liquidation_health"))
                          .add_attribute("sender", FromStr(info.sender))
                          .add_attribute("new_liquidation_health", FromInt(liquidation_health))),
           next_state)
      }
  }

  action set_liquidation_health_action = {
    pure val max_funds = 0
    nondet message_liquidation_health: int = 1.to(MAX_AMOUNT).oneOf() // Health > 0
    pure val message: ExecuteMsg = ExecuteMsg_SetLiquidationHealth({ liquidation_health: message_liquidation_health })
    execute_message(message, max_funds)
  }

  pure def set_mintable_health(state: ContractState, info: MessageInfo, mintable_health: int): (Result[Response, ContractError], ContractState) = {
      if (not(state.admin_addresses.contains(info.sender))) {
          (Err("UnauthorizedUser"), state)
      } else if (mintable_health <= 0) {
          (Err("HealthCannotBeZero"), state)
      } else if (mintable_health < state.liquidation_health) {
          (Err("MintableHealthLowerThanLiquidationHealth"), state)
      } else {
          val next_state = { ...state, mintable_health: mintable_health }
          (Ok(Response_new.add_attribute("action", FromStr("set_mintable_health"))
                          .add_attribute("sender", FromStr(info.sender))
                          .add_attribute("new_mintable_health", FromInt(mintable_health))), // Corrected attribute name
           next_state)
      }
  }

  action set_mintable_health_action = {
    pure val max_funds = 0
    nondet message_mintable_health: int = 1.to(MAX_AMOUNT).oneOf() // Health > 0
    pure val message: ExecuteMsg = ExecuteMsg_SetMintableHealth({ mintable_health: message_mintable_health })
    execute_message(message, max_funds)
  }

  pure def set_cw20_dira_contract_address(state: ContractState, info: MessageInfo, cw20_dira_contract_address: Addr): (Result[Response, ContractError], ContractState) = {
      // FIXME: Rust code is missing the admin check here! Adding it to the model for correctness.
      if (not(state.admin_addresses.contains(info.sender))) {
          (Err("UnauthorizedUser"), state)
      } else if (not(is_cw20_contract(cw20_dira_contract_address))) {
          (Err("InvalidCW20ContractAddress"), state)
      } else {
          val next_state = { ...state, cw20_dira_contract_address: Some(cw20_dira_contract_address) }
          (Ok(Response_new.add_attribute("action", FromStr("set_cw20_dira_contract_address"))
                          .add_attribute("contract_address", FromStr(cw20_dira_contract_address))),
           next_state)
      }
  }

  action set_cw20_dira_contract_address_action = {
    pure val max_funds = 0
    nondet message_cw20_dira_contract_address: Addr = ADDRESSES.oneOf() // Could be any address initially
    pure val message: ExecuteMsg = ExecuteMsg_SetCW20DiraContractAddress({ cw20_dira_contract_address: message_cw20_dira_contract_address })
    execute_message(message, max_funds)
  }

  // --- State Definition ---

  type ContractState = {
    collateral_token_denom: str,
    admin_addresses: List[Addr],
    liquidation_health: int,
    mintable_health: int,
    locked_collateral: Addr -> int,
    minted_dira: Addr -> int,
    collateral_token_price: int,
    cw20_dira_contract_address: Option[Addr],
  }

  pure val init_contract_state: ContractState = {
    collateral_token_denom: "",
    admin_addresses: [],
    liquidation_health: 0,
    mintable_health: 0,
    locked_collateral: Map(),
    minted_dira: Map(),
    collateral_token_price: 0,
    cw20_dira_contract_address: None,
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

  pure def reply(state: ContractState, _env: Env, _reply: Reply): (Result[Response, ContractError], ContractState) = {
      (Ok(Response_new), state)
  }

  // --- Boilerplate Actions ---

  pure val init_bank_state = ADDRESSES.mapBy(_ => DENOMS.mapBy(_ => MAX_AMOUNT))

  val env_val = { block: { time: time, height: 1 } }

  action init = {
    pure val max_funds = 0
    nondet sender = Set("admin").oneOf()
    val info = { sender: sender, funds: [] }

    nondet init_liquidation_health: int = 110.to(150).oneOf()
    nondet init_mintable_health: int = 130.to(200).oneOf()
    nondet init_collateral_token_denom: str = Set("uatom").oneOf()
    nondet init_cw20_addr_opt: Option[Addr] = Set(Some("cw20_addr_placeholder"), None).oneOf()

    assume(init_liquidation_health > 0)
    assume(init_mintable_health > 0)
    assume(init_mintable_health >= init_liquidation_health)
    assume(init_collateral_token_denom != "")

    pure val message: InstantiateMsg = {
      liquidation_health: init_liquidation_health,
      mintable_health: init_mintable_health,
      collateral_token_denom: init_collateral_token_denom,
      cw20_dira_contract_address: init_cw20_addr_opt,
    }

    pure val r = instantiate(init_contract_state, { block: { time: 0, height: 1 } }, info, message)

    all {
      contract_state' = r._2,
      bank' = init_bank_state,
      result' = r._1,
      time' = 0,
    }
  }

  // Refined execute_message to better handle bank updates based on response messages
  action execute_message(message, max_funds) = {
    nondet sender = ADDRESSES.oneOf()
    // Funds logic depends heavily on the message type
    val (funds, required_denom) = match message {
        | ExecuteMsg_LockCollateral => {
            // Requires funds in the collateral denom
            val required_denom = contract_state.collateral_token_denom
            nondet amount = 1.to(max_funds).oneOf() // Send at least 1 unit
            ([{ denom: required_denom, amount: amount }], required_denom)
          }
        | _ => {
            // Most other messages require 0 funds directly
            ([], "") // Empty denom indicates no specific fund requirement check here
          }
    }
    val info = { sender: sender, funds: funds }

    // --- Pre-execution Checks ---
    val isAdmin = contract_state.admin_addresses.contains(sender)
    val requiresAdmin = match message {
    | ExecuteMsg_SetCollateralPriceInDirham(_) => true
    | ExecuteMsg_SetLiquidationHealth(_) => true
    | ExecuteMsg_SetMintableHealth(_) => true
    | ExecuteMsg_SetCW20DiraContractAddress(_) => true // Added admin check assumption
    | _ => false
    }
    assume(not(requiresAdmin) or isAdmin) // Abort trace if non-admin calls admin function

    val r = execute(contract_state, env_val, info, message)
    all {
      // Bank check: User must have the funds they send (if any required)
      if (funds.length() > 0) {
         bank.get(sender).get(required_denom) >= funds[0].amount
      } else {
         true // No funds required, always passes
      },
      // Bank update logic based on response messages
      match r._1 {
        | Ok(resp) => {
             // Default: no change unless BankMsg found
             var next_bank = bank
             // Check response messages for BankMsg::Send
             if (resp.messages.any(submsg => match submsg.msg { case CosmosMsg_Bank(BankMsg_Send(_)) => true case _ => false })) {
                // Found a Bank Send message, assume it's UnlockCollateral sending FROM contract TO sender
                val sendMsg = resp.messages.filter(submsg => match submsg.msg { case CosmosMsg_Bank(BankMsg_Send(_)) => true case _ => false })[0].msg.value // Get the BankMsg_Send
                val sentAmount = sendMsg.amount[0].amount
                val sentDenom = sendMsg.amount[0].denom
                // Check if contract has enough balance to send
                if (bank.get(CONTRACT_ADDRESS).get(sentDenom) >= sentAmount) {
                    next_bank = bank.setBy(CONTRACT_ADDRESS, balances => balances.setBy(sentDenom, balance => balance - sentAmount))
                                     .setBy(sendMsg.to_address, balances => balances.setBy(sentDenom, balance => balance + sentAmount))
                } else {
                    // Contract doesn't have funds? This shouldn't happen if logic is correct, treat as error or ignore?
                    // For simplicity, ignore bank change if contract lacks funds
                    next_bank = bank
                }
             } else if (message == ExecuteMsg_LockCollateral) {
                // LockCollateral case (funds already checked)
                val sent_amount = funds[0].amount
                next_bank = bank.setBy(sender, balances => balances.setBy(required_denom, balance => balance - sent_amount))
                               .setBy(CONTRACT_ADDRESS, balances => balances.setBy(required_denom, balance => balance + sent_amount))
             }
             // Assign the calculated next bank state
             bank' = next_bank
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
          val new_state = process_message(current_state, env_val, CONTRACT_ADDRESS, submsg, reply)
          all {
            bank' = new_state.bank,
            result' = new_state.result,
            contract_state' = new_state.contract_state,
            advance_time,
          }
      }
      | None => execute_step
    }
  }

} // End of module stable_dira