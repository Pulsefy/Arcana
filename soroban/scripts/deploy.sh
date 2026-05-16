#!/bin/bash

# Arcana Soroban Contract Deployment Script

set -e

echo "🚀 Deploying Arcana Profile Contract to Stellar Testnet..."

# Check if Soroban CLI is installed
if ! command -v soroban &> /dev/null; then
    echo "❌ Soroban CLI not found. Please install it first: https://soroban.stellar.org/docs/getting-started/install"
    exit 1
fi

# Build the contract
echo "🔧 Building contract..."
cargo build --release

# Get the contract ID from the built Wasm file
CONTRACT_WASM="target/wasm32-unknown-unknown/release/arcana_contract.wasm"
if [ ! -f "$CONTRACT_WASM" ]; then
    echo "❌ Contract WASM file not found. Make sure to run 'cargo build --release' first."
    exit 1
fi

# Deploy to testnet
echo "🌐 Deploying to Stellar Testnet..."
soroban contract deploy \
    --wasm "$CONTRACT_WASM" \
    --network testnet \
    --source "<YOUR_TESTNET_ACCOUNT>"

if [ $? -eq 0 ]; then
    echo "✅ Arcana Profile Contract deployed successfully!"
    echo "💡 Next steps:"
    echo "   - Initialize the contract with 'soroban contract invoke --id <CONTRACT_ID> --fn initialize'"
    echo "   - Create your first profile with 'soroban contract invoke --id <CONTRACT_ID> --fn create_profile --args ...'"
else
    echo "❌ Deployment failed. Check your Soroban configuration and network connection."
    exit 1
fi
