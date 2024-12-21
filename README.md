# Dira - The ATOM-backed stablecoin for the AED

Dira is a decentralized, overcollateralized stablecoin solution for AED (United Arab Emirates Dirham), inspired by MakerDAO for USD. Built on the Cosmos ecosystem, Dira allows users to mint AED stablecoins by locking ATOM as collateral, providing a stable on-chain value store and enabling seamless integration with the growing blockchain ecosystem in Dubai and beyond.

## Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
- [Smart Contract](#smart-contract)
- [Frontend](#frontend)
- [How It Works](#how-it-works)
- [Schema Generation](#schema-generation)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

Dira addresses the need for localized stablecoins in the Cosmos ecosystem. With the increasing adoption of tokenized real-world assets (RWAs) in Dubai, Dira provides a reliable and decentralized AED-pegged stablecoin, backed by ATOM as collateral.

Dira will leverage IBC to integrate seamlessly with Cosmos chains, enhancing liquidity and enabling AED-based liquidity pools for cross-border transactions. By leveraging staking and liquidity pool fees, Dira also drives utility and demand for ATOM.

---

## Features

- **Decentralized & Overcollateralized**: Dira stablecoins are backed by ATOM, ensuring stability and security.
- **Cross-Chain Integration**: Dira will leverage IBC to integrate with Neutron, Mantra, and other Cosmos chains.
- **Liquidity Pools**: Enables AED/USD stablecoin liquidity pools for efficient on-chain currency conversion.
- **Transparent Governance**: Admin functionalities are accessible only to approved wallets.

---

## Getting Started

Follow these steps to get started with Dira:

1. **Clone the Repository**
   ```bash
   git clone https://github.com/NotRithik/StableDira.git
   cd StableDira
   ```

2. **Compile the Smart Contract**
   Ensure you have [Rust](https://www.rust-lang.org/) and `wasm32-unknown-unknown` target installed.
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo build --target wasm32-unknown-unknown --release
   ```

   Alternatively, you can use the Cosmos optimizer for a production-ready `.wasm` file:
   ```bash
   docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/optimizer:0.16.0
   ```

3. **Run Tests**
   Use `cw-multi-test` to test the smart contract.
   ```bash
   cargo test -- --show-output
   ```

4. **Deploy the Contract**
   Deploy the compiled `.wasm` file to a Cosmos chain.

---

## Smart Contract

The Dira smart contract is written in Rust using CosmWasm.

- **State Management**:
  - Collateral amounts, minted stablecoins, and admin addresses are stored securely on-chain.
  - Supports liquidations when collateral health drops below the threshold.

- **Query Functions**:
  - Public query endpoints provide transparent access to all contract states, including locked collateral, minted Dira, collateral prices, and admin addresses.

The smart contract repository: [Dira Smart Contract](https://github.com/NotRithik/StableDira).

---

## Frontend

A web interface is under development for user interaction with the Dira ecosystem.

- **Lock Collateral**: Users can lock ATOM to mint Dira stablecoins.
- **Mint/Burn Stablecoins**: Intuitive interface to manage minted stablecoins.
- **Liquidation**: View and liquidate unhealthy stablecoins.
- **Collateral Auction**: Participate in Collateral Auctions to buy liquidated assets.

Live Preview: [Dira Frontend](https://xatrknaz5h7ejdov.vercel.app/)
Frontend Source Code: [Dira Frontend Repository](https://github.com/NotRithik/dira-frontend)

---

## How It Works

1. **Lock Collateral**:
   Users lock ATOM as collateral in the smart contract.

2. **Mint Stablecoins**:
   The system calculates the user's collateral health, allowing them to mint stablecoins proportionally.

3. **Burn Stablecoins**:
   Users burn Dira to unlock collateral.

4. **Liquidations**:
   If a user's collateral health drops below the threshold, their collateral is liquidated by other users for rewards.

---

## Schema Generation

The project provides a schema generation script to enforce message type correctness when integrating with frontends (e.g., TypeScript).

Run the `generate_message_ts.sh` script to generate schema files for all messages and query responses:

```bash
./scripts/generate_message_ts.sh
```

This ensures consistent JSON message passing from the frontend to the contract, improving type safety and reliability.

---

## Roadmap

- **Phase 1**:
  - Launch Dira on testnet.
  - Develop user-friendly web interface.
  - Conduct security audits.

- **Phase 2**:
  - Integrate with Neutron and Mantra via IBC.
  - Establish liquidity pools for AED/USD stablecoins.

- **Phase 3**:
  - Expand to other regional stablecoins (e.g., SGD).
  - Develop a multi-currency stablecoin DEX.

---

## Contributing

Contributions are welcome!

1. Fork the repository.
2. Create a feature branch.
3. Submit a pull request with a clear description of the changes.

For major changes, please open an issue first to discuss your ideas.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

**Dira - The ATOM-backed stablecoin for the Emirati Dirham. Secure, stable, and decentralized.**
