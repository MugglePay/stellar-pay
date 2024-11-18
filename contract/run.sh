#!/bin/bash

# Generate accounts
stellar keys generate --global admin --network testnet
stellar keys generate --global sender --network testnet
stellar keys generate --global receiver --network testnet
stellar keys generate --global fee --network testnet

# Get addresses
ADMIN_ADDRESS=$(stellar keys address admin)
USER1_ADDRESS=$(stellar keys address sender)
USER2_ADDRESS=$(stellar keys address receiver)
FEE_WALLET_ADDRESS=$(stellar keys address fee)

# Build contracts
stellar contract build
cargo build --target wasm32-unknown-unknown --release
#cargo install --locked stellar-cli --features opt

# Optimized Build
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/contract.wasm

# Deploy token contracts
SEND_TOKEN_ADDRESS=$(stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/contract.wasm \
    --network testnet \
    --source admin)

RECV_TOKEN_ADDRESS=$(stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/contract.wasm \
    --network testnet \
    --source admin)

# Deploy swap contract
SWAP_CONTRACT_ADDRESS=$(stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/contract.wasm \
    --network testnet \
    --source admin)

# Initialize swap contract
stellar contract invoke \
    --id $SWAP_CONTRACT_ADDRESS \
    --source admin \
    --network testnet \
    -- \
    initialization \
    --admin $ADMIN_ADDRESS

# Mint tokens
stellar contract invoke \
    --id $SEND_TOKEN_ADDRESS \
    --source admin \
    --network testnet \
    -- \
    mint \
    --to $USER1_ADDRESS \
    --amount 10000

stellar contract invoke \
    --id $RECV_TOKEN_ADDRESS \
    --source admin \
    --network testnet \
    -- \
    mint \
    --to $USER2_ADDRESS \
    --amount 10000

stellar contract invoke \
    --id $SEND_TOKEN_ADDRESS \
    --source admin \
    --network testnet \
    -- \
    approve \
    --token $SEND_TOKEN_ADDRESS \
    --from $USER1_ADDRESS \
    --spender $SWAP_CONTRACT_ADDRESS \
    --amount 1000000000

stellar contract invoke \
    --id $RECV_TOKEN_ADDRESS \
    --source admin \
    --network testnet \
    -- \
    approve \
    --token $RECV_TOKEN_ADDRESS \
    --from $USER2_ADDRESS \
    --spender $SWAP_CONTRACT_ADDRESS \
    --amount 1000000000


# Set Fee
stellar contract invoke \
    --id $SWAP_CONTRACT_ADDRESS \
    --source admin \
    --network testnet \
    -- \
    set_fee \
    --fee_rate 30 \
    --fee_wallet $FEE_WALLET_ADDRESS

stellar contract invoke \
    --id $SWAP_CONTRACT_ADDRESS \
    --source admin \
    --network testnet \
    -- \
    create_order \
    --sender $USER1_ADDRESS \
    --send_token $SEND_TOKEN_ADDRESS \
    --recv_token $RECV_TOKEN_ADDRESS \
    --send_amount 500 \
    --recv_amount 50 \
    --min_recv_amount 10

