# Arcana Soroban Contracts

This directory contains the Soroban smart contracts that power the Arcana platform on the Stellar network.

## Overview

The Arcana Soroban contracts implement the core financial identity and reputation system for the Arcana platform, including:

- **Arcana Profile**: A living 3D/AR holographic financial identity
- **Arcana Score**: Dynamic on-chain reputation and credit scoring system
- **Realm Send**: Instant payments using profile names or AR scanning
- **Financial Fate**: AI-powered financial insights (on-chain data)

## Key Features

- **Secure Profile Management**: Create, update, and manage Arcana Profiles
- **Reputation Scoring**: Track and calculate Arcana Scores based on financial activity
- **Cross-Chain Ready**: Designed for future cross-chain integration
- **3D/AR Integration**: Structured to support AR-enhanced financial interactions

## Project Structure

```
soroban/
├── src/                # Soroban smart contract source code (Rust)
│   └── lib.rs          # Core Arcana Profile and Score contract
├── scripts/            # Deployment and testing scripts
├── tests/              # Contract tests
├── Cargo.toml          # Soroban project configuration
└── README.md           # This file
```

## Getting Started

### Prerequisites

- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/install)
- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

### Installation

```bash
# Install Soroban CLI
curl --proto '=https' --tlsv1.2 -sSf https://sh.soroban.stellar.org/install.sh | sh

# Build contracts
cargo build --release

# Run tests
cargo test
```

## Integration

The Arcana Profile contract integrates with:

- **Frontend (`apps/web`)**: For 3D/AR interface and user experience
- **Backend (`backend`)**: For API layer and off-chain logic
- **Stellar Network**: For on-chain execution and security

---

**Made with passion for the future of finance on Stellar.**