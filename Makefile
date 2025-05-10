.PHONY: build run-ci

# Default values (can be overridden via command-line)
MADARA_MODE ?= devnet
BASE_PATH ?= ../data/madara-db

# Default target
all: build madara

# Build the project
build:
	cargo build

# Run Madara on a specific mode (not appchain)
madara:
	cargo run create $(MADARA_MODE) --base-path $(BASE_PATH)&
	@echo "Waiting for Madara container to start..."
	@until [ "$$(docker inspect -f '{{.State.Running}}' madara_runner 2>/dev/null)" = "true" ]; do \
	  sleep 5; \
	done
	@echo "Madara is now running!"

# Run Appchain with orchestrator and bootstrapper
appchain:
	cargo run create app-chain&
	@until [ "$$(docker inspect -f "{{.State.Running}}" bootstrapper_l2 2>/dev/null)" = "true" ]; do \
	    echo "Waiting for Bootstrapper L2 container to start..."; \
	    sleep 5; \
	  done
	@echo "Waiting for Bootstrapper L2 container to finish..."
	@docker wait bootstrapper_l2
	@until [ "$$(docker inspect -f "{{.State.Running}}" workaround 2>/dev/null)" = "true" ]; do \
	    echo "Waiting for Block Zero Workaround..."; \
	    sleep 1; \
	  done
	@echo "Waiting for Block Zero Workaround..."
	@docker wait workaround

# Run the transfer scripts
transfer:
	@until [ "$$(docker inspect -f "{{.State.Running}}" transfer 2>/dev/null)" = "true" ]; do \
	    sleep 1; \
	  done
	@echo "Waiting for transfer scripts..."
	@docker wait transfer
	@echo "Running transfer scripts..."
	@cd deps/scripts/transfer_from_L1 && npm install && npm run transfer-l1

# Stop a madara instance
stop-madara:
	@cd deps/madara && docker compose down

# Stop a appchain instance
stop-appchain:
	@cd deps && docker compose down

run-madara-ci: build madara stop-madara

run-appchain-transfer-ci: build appchain transfer stop-appchain