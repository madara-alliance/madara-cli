#!/bin/sh

if [ -f "$RPC_API_KEY_FILE" ]; then
  export RPC_API_KEY=$(cat "$RPC_API_KEY_FILE")
else
  echo "Error: RPC_API_KEY_FILE not found!" >&2
  exit 1
fi

exec tini -- ./pathfinder \
  --network custom \
  --chain-id MADARA_DEVNET \
  --ethereum.url wss://eth-sepolia.g.alchemy.com/v2/WIUR5JUZXieEBkze6Xs3IOXWhsS840TX \
  --gateway-url http://madara:8080/gateway --feeder-gateway-url http://madara:8080/feeder_gateway \
  --storage.state-tries archive \
  --data-directory /usr/share/pathfinder/data \
  --http-rpc 0.0.0.0:9545 
