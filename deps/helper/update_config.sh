#!/bin/bash
source /tmp/data/.env

# Check for required arguments
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <target_yaml_file> <mode>"
    echo "Modes: bootstrapper_l1, bootstrapper_l2, madara"
    exit 1
fi

TARGET_FILE="$1"
MODE="$2"

# Ensure the target file exists
if [ ! -f "$TARGET_FILE" ]; then
    echo "Error: Target file $TARGET_FILE not found."
    exit 1
fi

# Update Bootstrapper verifier address
if [ "$MODE" == "bootstrapper_l1" ]; then
    # Ensure the environment variable is set
    if [ -z "$verifier_address" ]; then
        echo "Error: verifier_address environment variable is not set."
        exit 1
    fi

    # Use jq to replace verifier_address
    jq --arg new_verifier "$verifier_address" '.verifier_address = $new_verifier' "$TARGET_FILE" > tmpfile && mv tmpfile "$TARGET_FILE"

    echo "Updated verifier_address in $TARGET_FILE"
    exit 0
fi

# Update Bootstrapper core contract address
if [ "$MODE" == "bootstrapper_l2" ]; then
    # Ensure the environment variable is set
    if [ -z "$starknet_contract_address" ] || [ -z "$starknet_contract_implementation_address" ]; then
        echo "Error: starknet_contract_address and starknet_contract_implementation_address environment variables must be set."
        exit 1
    fi

    # Use jq to replace core contract address
    jq --arg new_value "$starknet_contract_address" '.core_contract_address = $new_value' "$TARGET_FILE" > tmpfile && mv tmpfile "$TARGET_FILE"

    # Use jq to replace core contract implementation address
    jq --arg new_value "$starknet_contract_implementation_address" '.core_contract_implementation_address = $new_value' "$TARGET_FILE" > tmpfile && mv tmpfile "$TARGET_FILE"

    echo "Updated verifier_address in $TARGET_FILE"
    exit 0
fi

# Update Madara core contract and verifier address
if [ "$MODE" == "madara" ]; then
    # Ensure required environment variables are set
    if [ -z "$starknet_contract_address" ] || [ -z "$verifier_address" ]; then
        echo "Error: starknet_contract_address and verifier_address environment variables must be set."
        exit 1
    fi

    # Replace core contract address and verifier address
    awk -v new_eth_core="$starknet_contract_address" -v new_verifier="$verifier_address" '
    /^\s*eth_core_contract_address:/ {
        print "eth_core_contract_address: " new_eth_core;
        next;
    }
    /^\s*eth_gps_statement_verifier:/ {
        print "eth_gps_statement_verifier: " new_verifier;
        next;
    }
    { print }
    ' "$TARGET_FILE" > tmpfile && mv tmpfile "$TARGET_FILE"

    echo "Updated eth_core_contract_address and eth_gps_statement_verifier in $TARGET_FILE"
    exit 0
fi

# Invalid mode handling
echo "Error: Invalid mode '$MODE'."
echo "Available modes: bootstrapper, madara, tbd"
exit 1
