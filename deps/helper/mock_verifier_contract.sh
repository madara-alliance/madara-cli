#!/bin/bash

# Default Anvil private key
PRIVATE_KEY="$1"  # Accept private key as a command-line argument
ANVIL_URL="http://anvil:8545"

# Deploy the verifier contract using forge create
echo -e "🚀 Deploying verifier contract...\n"
VERIFIER_RESULT=$(forge create \
    --rpc-url "$ANVIL_URL" \
    --private-key "$PRIVATE_KEY" \
    --broadcast \
    "MockGPSVerifier.sol:MockGPSVerifier" \
    2>&1)

if [ $? -ne 0 ]; then
    echo "Error deploying verifier contract:"
    echo "$VERIFIER_RESULT"
    exit 1
fi

# Extract contract address from forge create output
VERIFIER_ADDRESS=$(echo "$VERIFIER_RESULT" | grep "Deployed to" | awk '{print $3}')
echo -e "📦 Verifier deployed at: $VERIFIER_ADDRESS\n"
