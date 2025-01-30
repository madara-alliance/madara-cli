#!/bin/bash

RPC_URL="http://madara:9945"
BLOCK_NUMBER="0"

# Default Anvil private key
PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
ANVIL_URL="http://anvil:8545"

echo -e "\n🔍 Fetching state update for block $BLOCK_NUMBER..."

# Fetch state update from RPC with correct params structure
STATE_UPDATE=$(curl -s -X POST -H "Content-Type: application/json" --data "{
    \"jsonrpc\":\"2.0\",
    \"method\":\"starknet_getStateUpdate\",
    \"params\": {
        \"block_id\": {
            \"block_number\": $BLOCK_NUMBER
        }
    },
    \"id\":1
}" "$RPC_URL")

# Extract global root and block hash from the response
GLOBAL_ROOT=$(echo "$STATE_UPDATE" | jq -r '.result.new_root')
BLOCK_HASH=$(echo "$STATE_UPDATE" | jq -r '.result.block_hash')

if [ "$GLOBAL_ROOT" == "null" ] || [ "$BLOCK_HASH" == "null" ]; then
    echo "Error: Failed to fetch state update data"
    echo "Response: $STATE_UPDATE"
    exit 1
fi

echo -e "\n📊 State Update Data:"
echo "   Global Root: $GLOBAL_ROOT"
echo "   Block Hash: $BLOCK_HASH"
echo ""


# TODO: these might be input parameters (result from core_contract_deployment script)
VERIFIER_ADDRESS="0x5FbDB2315678afecb367f032d93F642f64180aa3"
CONTRACT_ADDRESS="0xe7f1725e7734ce288f8367e1bb143e90bb3f0512"

# Initialize the contract with the required data
echo -e "🔧 Initializing contract...\n"

# Create the initialization data
PROGRAM_HASH="853638403225561750106379562222782223909906501242604214771127703946595519856"
AGGREGATOR_PROGRAM_HASH="0"
CONFIG_HASH="1773546093672122186726825451867439478968296982619761985456743675021283370179"

# Encode the initialization data
INIT_DATA=$(cast abi-encode "f(uint256,uint256,address,uint256,uint256,int256,uint256)" \
    $PROGRAM_HASH \
    $AGGREGATOR_PROGRAM_HASH \
    $VERIFIER_ADDRESS \
    $CONFIG_HASH \
    $GLOBAL_ROOT \
    $BLOCK_NUMBER \
    $BLOCK_HASH)

# Call initializeContractState
INIT_RESULT=$(cast send \
    --private-key "$PRIVATE_KEY" \
    --rpc-url "$ANVIL_URL" \
    $CONTRACT_ADDRESS \
    "initializeContractState(bytes)" \
    $INIT_DATA)

if [ $? -eq 0 ]; then
    TX_HASH=$(echo "$INIT_RESULT" | grep "transactionHash" | awk '{print $2}')
    echo -e "✅ Contract initialized successfully!"
    echo -e "   Transaction: $TX_HASH\n"
else
    echo -e "❌ Error initializing contract\n"
    echo "$INIT_RESULT"
    exit 1
fi
