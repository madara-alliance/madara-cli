#!/bin/bash

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed. Please install jq first."
    exit 1
fi

# Check if curl is installed
if ! command -v curl &> /dev/null; then
    echo "Error: curl is required but not installed. Please install curl first."
    exit 1
fi

# Read arguments
ABI_FILE='./Starknet.json'

# Default Anvil private key
PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
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

# Now deploy the main Starknet contract
echo -e "🚀 Deploying Starknet contract...\n"

# Extract bytecode from the JSON file
BYTECODE=$(jq -r '.bytecode.object' "$ABI_FILE" | sed 's/^0x//')

if [ "$BYTECODE" == "null" ] || [ -z "$BYTECODE" ]; then
    echo "Error: No bytecode found in the JSON file"
    exit 1
fi

# Deploy the contract using cast
RESULT=$(cast send \
    --private-key "$PRIVATE_KEY" \
    --rpc-url "$ANVIL_URL" \
    --create "0x$BYTECODE" \
    2>&1)

# Check if deployment was successful
if [ $? -eq 0 ]; then
    # Extract contract address from the result using grep and awk
    CONTRACT_ADDRESS=$(echo "$RESULT" | grep "contractAddress" | awk '{print $2}')
    
    if [ -n "$CONTRACT_ADDRESS" ]; then
        echo -e "📦 Starknet contract deployed successfully at: $CONTRACT_ADDRESS\n"

        # sleep for 2 seconds
        sleep 2
    
    else
        echo "❌ Error: Could not extract contract address from output"
        exit 1
    fi
else
    echo "❌ Error deploying contract:"
    echo "$RESULT"
    exit 1
fi