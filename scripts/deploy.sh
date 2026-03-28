#!/bin/bash

# Set NEAR environment (testnet or mainnet)
if [ -z "$1" ]; then
  echo "Usage: $0 <testnet|mainnet>"
  exit 1
fi

ENVIRONMENT=$1

# Validate environment
if [ "$ENVIRONMENT" != "testnet" ] && [ "$ENVIRONMENT" != "mainnet" ]; then
  echo "Invalid environment. Please choose 'testnet' or 'mainnet'."
  exit 1
fi

# Check account existence
ACCOUNT_NAME="your_account_name_here"  # Replace with your actual account name

if ! near state $ACCOUNT_NAME > /dev/null; then
  echo "Account $ACCOUNT_NAME does not exist. Exiting..."
  exit 1
fi

# Pre-deployment compilation
echo "Compiling NEAR contract..."
# Assume the compilation command below, replace with actual build command
near compile your_contract.wasm

# Deploy contract
echo "Deploying contract to $ENVIRONMENT..."
near deploy --wasmFile your_contract.wasm --contractId $ACCOUNT_NAME --networkId $ENVIRONMENT

# Post-deployment verification
echo "Verifying deployment..."
if near state $ACCOUNT_NAME | grep -q 'Your contract identifier'; then
  echo "Deployment successful."
else
  echo "Deployment failed."
  exit 1
fi

# Event log monitoring
echo "Monitoring events..."
# This is a placeholder for actual log monitoring command
# Replace with actual query command for events
while true; do
  near view $ACCOUNT_NAME get_logs
  sleep 10  # Adjust sleep time as necessary
done
