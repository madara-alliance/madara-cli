#!/bin/bash

# Default Anvil private key
PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
ANVIL_URL="http://localhost:8545"
RPC_URL="http://localhost:9945"

# BLOCK_NUMBER=$(( $(curl --silent --location "$RPC_URL"/v0_7_1/ \
#   --header 'Content-Type: application/json' \
#   --data '{
#     "jsonrpc": "2.0",
#     "method": "starknet_blockNumber",
#     "params": [],
#     "id": 1
#   }' | jq -r '.result')))

# echo -e "\n🔍 Fetching state update for block $BLOCK_NUMBER..."

BLOCK_NUMBER="10"

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
VERIFIER_ADDRESS="0x5fbdb2315678afecb367f032d93f642f64180aa3"
CONTRACT_ADDRESS="0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"

# Initialize the contract with the required data
echo -e "🔧 Initializing contract...\n"

# Create the initialization data
PROGRAM_HASH="1865367024509426979036104162713508294334262484507712987283009063059134893433"
AGGREGATOR_PROGRAM_HASH="0"
CONFIG_HASH="1773546093672122186726825451867439478968296982619761985456743675021283370179"

# { program_hash: 1865367024509426979036104162713508294334262484507712987283009063059134893433, aggregate_program_hash: 0, verifier_address: 0x000000000000000000000000000000000000abcd, config_hash: 1773546093672122186726825451867439478968296982619761985456743675021283370179,

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
