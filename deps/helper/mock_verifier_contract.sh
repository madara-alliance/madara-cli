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

# Check if .env file exists
ENV_FILE="/tmp/data/.env"
if [ -f "$ENV_FILE" ]; then
    # Check if verifier_address exists in .env file
    CURRENT_ADDRESS=$(grep "^verifier_address=" "$ENV_FILE" | cut -d= -f2)
    
    if [ "$CURRENT_ADDRESS" = "$VERIFIER_ADDRESS" ]; then
        echo "Verifier address in .env file is already up to date."
    else
        # Address is different or doesn't exist, update it
        if [ -n "$CURRENT_ADDRESS" ]; then
            # Replace existing verifier_address line
            sed -i "s/^verifier_address=.*/verifier_address=$VERIFIER_ADDRESS/" "$ENV_FILE"
            echo "Updated verifier_address in .env file."
        else
            # Add verifier_address line if it doesn't exist
            echo "verifier_address=$VERIFIER_ADDRESS" >> "$ENV_FILE"
            echo "Added verifier_address to .env file."
        fi
    fi
else
    # Create .env file if it doesn't exist
    echo "verifier_address=$VERIFIER_ADDRESS" > "$ENV_FILE"
    echo "Created .env file with verifier_address."
fi
