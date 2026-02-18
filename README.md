---
### Repo 2: `afiapass-contracts`
**The "Truth" — Soroban Smart Contracts**

```markdown
# AfiaPass Soroban Contracts 🦀 🌟

## The Logic Layer for Automated Revenue Collection

## 🏗️ Architecture
These contracts act as the "Digital Law" for transit permits in Nigeria. When a payment is made, the contract ensures the funds are mathematically locked and split before a permit is validated.

## 🎯 Smart Contract Features
- **Splitter Logic**: Precision `i128` math to handle Naira stablecoin splits (e.g., 5% to LGA wallet, 90% to Vendor).
- **Minting**: Issues a unique, time-bound `AFIAPASS` token metadata on the ledger.
- **Registry**: Maintains a verifiable list of authorized Government/Union wallet addresses.

## ⚙️ Development
### Prerequisites
- Rust & Cargo
- Stellar CLI

### Deployment
```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Deploy to Testnet
stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/afiapass_splitter.wasm \
    --source-account my-account \
    --network testnet
