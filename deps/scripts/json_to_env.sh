#!/bin/bash

# Check if jq is installed
if ! command -v jq &>/dev/null; then
    echo "jq is required but not installed. Install it and try again."
    exit 1
fi

# Check for input JSON file
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <json_file>"
    exit 1
fi

JSON_FILE="$1"
OUTPUT_FILE="data/.env"

# Validate the JSON file
if ! jq empty "$JSON_FILE" 2>/dev/null; then
    echo "Invalid JSON file: $JSON_FILE"
    exit 1
fi

# Read JSON and append keys and values to .env
jq -r 'to_entries | map("\(.key)=\(.value|tostring)") | .[]' "$JSON_FILE" >> "$OUTPUT_FILE"

echo "Environment variables written to $OUTPUT_FILE"
