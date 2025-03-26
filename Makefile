.PHONY: build run-ci

# Default values (can be overridden via command-line)
MADARA_MODE ?= devnet
BASE_PATH ?= ../data/madara-db

# Default target
all: build madara kill

# Build the project
build:
	cargo build

# Run Madara on a specific mode (not appchain)
madara:
	cargo run create $(MADARA_MODE) --base-path $(BASE_PATH) & echo $$! > process.pid
	@echo "Waiting for Madara container to start..."
	@until [ "$$(docker inspect -f '{{.State.Running}}' madara_runner 2>/dev/null)" = "true" ]; do \
	  sleep 5; \
	done
	@echo "Madara is now running!"

# Run Appchain with orchestrator and bootstrapper
appchain:
	cargo run create app-chain & echo $$! > process.pid
	@until [ "$$(docker inspect -f "{{.State.Running}}" bootstrapper_l2 2>/dev/null)" = "true" ]; do \
	    echo "Waiting for Bootstrapper L2 container to start..."; \
	    sleep 1; \
	  done
	@echo "Waiting for Bootstrapper L2 container to finish..."
	@docker wait bootstrapper_l2

# Run the transfer scripts
transfer:
	@echo "Running transfer scripts..."
	@cd deps/scripts/transfer_from_L1 && npm install && npm run transfer-l1

# Kill a process
kill:
	@kill $$(cat process.pid)
	@rm -f process.pid

run-madara: build madara kill

run-appchain: build appchain transfer kill