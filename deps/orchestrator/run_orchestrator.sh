#!/bin/sh

# Sleep until all the services are up and running
for i in $(seq 60 -1 1); do
    echo "Starting Orchestrator in $i seconds..."
    sleep 1
done

# Orchestrator setup
./orchestrator setup --aws --aws-s3 --aws-sqs --aws-sns --aws-event-bridge --event-bridge-type rule

# Orchestrator run
RUST_LOG=info ./orchestrator run --sharp --aws --settle-on-ethereum --aws-s3 --aws-sqs --aws-sns --da-on-ethereum --mongodb