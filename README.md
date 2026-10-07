# Callora Contracts
Soroban smart contracts for the Callora API marketplace: prepaid vault (USDC) and balance deduction for pay-per-call settlement.

[![CI](https://github.com/CalloraOrg/Callora-Contracts/actions/workflows/ci.yml/badge.svg)](https://github.com/CalloraOrg/Callora-Contracts/actions/workflows/ci.yml)
[![Coverage](https://github.com/CalloraOrg/Callora-Contracts/actions/workflows/coverage.yml/badge.svg)](https://github.com/CalloraOrg/Callora-Contracts/actions/workflows/coverage.yml)

## Tech stack

- **Rust** with **Soroban SDK** (Stellar)
- Contract compiles to WebAssembly and deploys to Stellar/Soroban
- Minimal WASM size (~17.5 KB for vault)

## Contract Quickstart

A minimal set of commands to build, test, and produce release WASM for the Soroban contracts in this workspace. Run them from the repository root.

**Prerequisites:** [Rust](https://rustup.rs/) (stable) with the `wasm32-unknown-unknown` target (`rustup target add wasm32-unknown-unknown`).

```bash
# 1. Format & lint (fails on any warning)
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# 2. Build and run the full test suite
cargo build
cargo test

# 3. Release WASM for a specific contract
cargo build --target wasm32-unknown-unknown --release -p callora-vault
cargo build --target wasm32-unknown-unknown --release -p callora-revenue-pool
cargo build --target wasm32-unknown-unknown --release -p callora-settlement

# 4. Or build all contracts and verify WASM size limits in one step
./scripts/check-wasm-size.sh

# 5. Line-coverage check (must stay ≥ 95%)
./scripts/coverage.sh
