# Dira: The Decentralized AED Stablecoin - Smart Contract Repository

This repository houses the **smart contract** for **Dira**, a decentralized, overcollateralized stablecoin pegged to the Emirati Dirham (AED). Built using CosmWasm in Rust and designed for the Cosmos ecosystem, Dira provides a robust and transparent smart contract to facilitate the minting of a digital AED currency, backed by the OM token.

Dira's smart contract is the core of the Dira protocol, ensuring the stability, security, and decentralized nature of the AED stablecoin.  A user-friendly frontend application is also available (see below) to interact with this smart contract once deployed.

---

## Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Getting Started - Smart Contract Development](#getting-started---smart-contract-development)
- [Smart Contract Architecture](#smart-contract-architecture)
- [Frontend Application](#frontend-application)
- [How Dira Works - Smart Contract Logic](#how-dira-works---smart-contract-logic)
- [Schema Generation](#schema-generation)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

Dira addresses the growing demand for localized stablecoins within the Cosmos ecosystem, particularly in regions like Dubai where tokenized real-world assets (RWAs) are gaining traction. This smart contract implements the core logic for Dira, providing a reliable and decentralized AED-pegged stablecoin solution, with OM tokens serving as collateral.

Dira is designed to leverage Inter-Blockchain Communication (IBC) for seamless integration with other Cosmos chains, enhancing liquidity for the AED stablecoin.  Liquidity pools for AED and other stablecoins are envisioned to facilitate efficient on-chain currency exchange.  Furthermore, Dira aims to drive utility and demand for the OM token through its protocol mechanisms.

---

## Features

The Dira smart contract incorporates the following key features:

*   **Decentralized and Overcollateralized:** Dira stablecoins are fully backed by OM collateral, algorithmically ensuring stability and security through smart contract logic.
*   **Cross-Chain Compatibility:**  Designed for future integration with Neutron, Mantra, and other Cosmos chains via IBC, enabling broader accessibility and utility.
*   **Enables Liquidity Pools:**  Provides the foundation for the creation of AED/USD and other stablecoin liquidity pools, facilitating efficient on-chain foreign exchange.
*   **Transparent Governance (Administered):**  Admin functionalities within the smart contract are designed to be executed by approved wallet addresses, ensuring transparent and controlled administrative actions.
*   **Robust State Management:**  Secure on-chain storage of critical state variables, including collateral amounts, minted stablecoins, and authorized admin addresses.
*   **Liquidation Mechanism:**  Implements automated liquidation processes to maintain collateral health and protocol solvency when collateral ratios fall below predefined thresholds.
*   **Public Query Endpoints:** Offers comprehensive public query endpoints for transparent access to all contract states, including collateral levels, minted Dira supply, and collateral price information.

---

## Getting Started - Smart Contract Development

To begin development or contribute to the Dira smart contract, follow these steps:

1.  **Clone the Smart Contract Repository**
    ```bash
    git clone https://github.com/NotRithik/StableDira.git
    cd StableDira
    ```

2.  **Compile the Smart Contract**
    Ensure you have [Rust](https://www.rust-lang.org/) installed and the `wasm32-unknown-unknown` target added to your Rust toolchain.
    ```bash
    rustup target add wasm32-unknown-unknown
    cargo build --target wasm32-unknown-unknown --release
    ```
    For optimized production builds, you can utilize the Cosmos optimizer Docker image:
    ```bash
    docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/optimizer:0.16.0
    ```

3.  **Run Unit Tests**
    Utilize `cw-multi-test` for comprehensive unit testing of the smart contract logic.
    ```bash
    cargo test -- --show-output
    ```

4.  **Deploy the Smart Contract**
    The compiled `.wasm` file (located in the `target/wasm32-unknown-unknown/release/` directory) can be deployed to a compatible Cosmos chain, such as Mantra Chain's DuKong testnet.  Deployment requires appropriate chain tooling and configuration.

---

## Smart Contract Architecture

The Dira smart contract, written in Rust using CosmWasm, is structured to ensure security, transparency, and efficiency.

-   **State Management:** The contract meticulously manages its state on-chain, ensuring secure storage and retrieval of:
    *   Collateral amounts locked by users.
    *   The total supply of minted Dira stablecoins.
    *   A list of authorized admin addresses with privileged functionalities.
    *   Key protocol parameters such as liquidation and minting health thresholds.

-   **Query Functions:**  Transparency is paramount. The contract exposes a suite of public query functions, enabling anyone to retrieve critical contract state information, including:
    *   User-specific locked collateral amounts.
    *   User-specific minted Dira balances.
    *   Current collateral price as determined by the price oracle.
    *   Protocol-wide liquidation and minting health parameters.
    *   The list of authorized admin addresses.
    *   The denomination of the collateral token.
    *   The contract address of the CW20 Dira token.

The source code for the Dira Smart Contract is available in this repository: [Dira Smart Contract Repository](https://github.com/NotRithik/StableDira).

---

## Frontend Application

A user-friendly web interface, the **Dira Frontend**, has been developed to facilitate interaction with the deployed Dira smart contract on the Mantra DuKong testnet.

The frontend currently implements the following core functionalities:

*   **Lock Collateral:**  Allows users to lock OM tokens within the smart contract to mint Dira stablecoins.
*   **Mint/Return Dira:** Provides an intuitive interface for users to mint Dira against their collateral and return Dira to unlock their OM.
*   **Dashboard:**  Offers users a comprehensive dashboard to monitor their wallet connection status, account information, and relevant market data.

Future frontend enhancements are planned, including features such as collateral auctions, oracle price feed visualization, and governance participation interfaces.

Live Preview of the Dira Frontend: [Dira Frontend](https://dira-alpha.vercel.app/).
Frontend Source Code Repository: [Dira Frontend Repository](https://github.com/NotRithik/dira-frontend)

---

## How Dira Works - Smart Contract Logic

The Dira smart contract operates through the following core mechanisms:

1.  **Collateral Locking:**
    Users initiate the process by locking OM tokens within the smart contract as collateral. This collateral acts as backing for the Dira stablecoins they intend to mint.

2.  **Stablecoin Minting:**
    Upon locking collateral, the smart contract calculates the user's current collateral health ratio based on the prevailing collateral price.  Users are then permitted to mint Dira stablecoins proportionally to their locked collateral, ensuring overcollateralization.

3.  **Stablecoin Burning (Returning):**
    To unlock their OM collateral, users must return (burn) an equivalent amount of Dira stablecoins to the smart contract. This burn mechanism maintains the peg and overall supply of Dira.

4.  **Liquidation Protocol:**
    The smart contract incorporates a robust liquidation protocol to safeguard the system's solvency. If a user's collateral health ratio declines below a predefined liquidation threshold (due to fluctuations in collateral price), their collateral becomes eligible for liquidation. Other users can then liquidate undercollateralized positions, receiving a portion of the liquidated collateral as a reward, while ensuring the system remains solvent.

---

## Schema Generation

To ensure type consistency and message integrity between the smart contract and external interfaces (such as the frontend), this project includes a schema generation script.

Execute the `generate_message_ts.sh` script to automatically generate TypeScript schema files for all smart contract messages and query responses:

```bash
./scripts/generate_message_ts.sh
```

---

## Roadmap

The Dira project follows a phased roadmap for development and expansion:

*   **Phase 1: Testnet Deployment and Core Functionality**
    *   Successful deployment of the Dira smart contract on a testnet environment (Mantra DuKong).
    *   Development and deployment of a user-friendly web interface (Dira Frontend).
    *   Comprehensive security audits of the smart contract codebase.

*   **Phase 2: Cross-Chain Integration and Liquidity Enhancement**
    *   Implementation of Inter-Blockchain Communication (IBC) to enable Dira's integration with Neutron, Mantra, and potentially other Cosmos-based chains.
    *   Establishment of liquidity pools for AED/USD stablecoins to facilitate on-chain currency exchange and increase Dira's utility.

*   **Phase 3: Regional Expansion and Advanced Features**
    *   Expansion of Dira to support additional regional stablecoins beyond AED, such as the Singapore Dollar (SGD).
    *   Development of UI components for a multi-currency stablecoin Decentralized Exchange (DEX) interface.
    *   Implementation of on-chain governance mechanisms for the Dira protocol, potentially including governance token integration.

---

## Contributing

We welcome contributions to the Dira smart contract project.

1.  Fork this repository: [Dira Smart Contract Repository](https://github.com/NotRithik/StableDira).
2.  Create a dedicated feature branch for your proposed changes.
3.  Submit a pull request with a clear and detailed description of your contributions.

For substantial changes or feature additions, it is recommended to open an issue first to discuss your ideas and approach with the project maintainers.

---

## License

The Dira smart contract project is licensed under the [MIT License](LICENSE).  See the [LICENSE](LICENSE) file for full license details.

---

**Dira Smart Contract - The foundation for a decentralized and stable Emirati Dirham currency within the Cosmos ecosystem.**
