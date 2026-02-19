
# ⭐ AfiaPass Contracts — The Soroban Truth Engine 🦀

**AfiaPass Contracts** serve as the on-chain "Digital Law" for transit and logistics permits in Nigeria. Built on the **Stellar Soroban** smart contract platform, these Rust-based contracts enforce absolute mathematical transparency for automated revenue collection and distribution.

When a payment is made (via the AfiaPass SDK), this contract ensures that NGNC (Naira stablecoins) are instantly and irreversibly split between Local Government Areas (LGAs), transport unions, and logistics operators (like **Drive-Thru Afia**) before any permit is cryptographically validated.

### 🔑 Quick Summary

| Property | Value |
| :--- | :--- |
| **Blockchain** | **Stellar Network** |
| **Execution Environment**| **Soroban (WASM)** |
| **Language** | **Rust** |
| **Primary Asset** | **NGNC / NGNT Stablecoins** |
| **Core Function** | Trustless Levy Splitting & Permit Minting |

---

### ⚖️ The "Digital Law" Features

#### 🧮 1. Precision Splitter Logic
* **Safe Math**: Utilizes absolute `i128` integer precision to handle Naira stablecoin fractional splits, completely eliminating floating-point errors or rounding attacks.
* **Atomic Routing**: Instantly routes 5% to the specific LGA treasury wallet, 5% to the Transport Union, and 90% to the Vendor. If one transfer fails, the entire transaction reverts.

#### 🎫 2. Immutable Permit Minting
* **Time-Bound Metadata**: Once the tax split succeeds, the contract logs a unique, timestamped `AFIAPASS` permit record directly to the ledger.
* **Double-Spend Prevention**: Enforces strictly monotonic sequence checks to ensure a rider cannot reuse a payment payload.

#### 🏛️ 3. On-Chain Auth & Registry
* **Wallet Whitelisting**: Maintains a verifiable, on-chain registry of authorized Government and Union wallet addresses to prevent funds from being routed to bad actors.
* **Multi-Sig Governance**: Administrative functions (like updating the tax percentage) require cryptographic signatures from multiple platform administrators.

---

### 🔄 Architecture Flow



1. **Invoke**: Rider pays via the AfiaPass App/SDK.
2. **Execute**: Soroban contract `issue_permit_and_split` function is called.
3. **Split**: Contract calculates percentages and moves NGNC to configured sub-wallets.
4. **Emit**: Contract emits a `PermitIssued` event.
5. **Sign**: The Java SDK detects the success and signs the offline SEP-10 JWT.

---

### 🛠️ Development Setup

**Prerequisites**
You must have the Rust toolchain and the Stellar CLI installed.

1. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh

    Add the WASM target (Soroban runs strictly on WebAssembly):
    Bash

    rustup target add wasm32-unknown-unknown

    Install the Stellar CLI:
    Bash

    cargo install --locked stellar-cli --features opt

🧪 Building and Testing

1. Build the Contract
Compile the Rust code into a highly optimized .wasm binary:
Bash

cargo build --target wasm32-unknown-unknown --release

(The compiled binary will be located at target/wasm32-unknown-unknown/release/afiapass_splitter.wasm)

2. Run the Test Suite
Execute the isolated Rust test suite to verify math and auth logic:
Bash

cargo test

🚀 Deployment Guide

We utilize the stellar-cli to deploy the compiled WASM to the network.

Deploy to Testnet
Bash

stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/afiapass_splitter.wasm \
    --source-account my-admin-account \
    --network testnet

Note: Save the Contract ID outputted by this command. You will need to inject this into the AFIAPASS_CONTRACT_ID environment variable of your Java SDK.
🛡️ Security & Auditing

    No std Library: This contract uses #![no_std] to ensure a deterministic, lightweight WASM footprint.

    Strict Authz: Utilizes Soroban's native env.auth() capabilities to ensure only the authorized AfiaPass platform key can trigger the split functions.

🗺️ Roadmap

    Phase 1: Core i128 stablecoin splitter and basic event emission.

    Phase 2: Dynamic, upgradable percentage logic via admin multi-sig.

    Phase 3: Multi-route registry for complex interstate haulage taxes.
