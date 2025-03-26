.PHONY: build run-ci

# Default values (can be overridden via command-line)
MADARA_MODE ?= devnet

# Default target
all: build madara

# Build the project
build:
	cargo build

# Run Madara on a specific mode and kills it after
madara:
	cargo run -p madara create $(MADARA_MODE) & echo $$! > process.pid
	@echo "Waiting for Madara container to start..."
	@until [ "$$(docker inspect -f '{{.State.Running}}' madara_runner 2>/dev/null)" = "true" ]; do \
	  sleep 5; \
	done
	@echo "Madara is now running!"
	@kill $$(cat process.pid)
	@rm -f process.pid

appchain:
	cargo run -p madara create app-chain & echo $$! > process.pid
	@until [ "$$(docker inspect -f "{{.State.Running}}" bootstrapper_l2 2>/dev/null)" = "true" ]; do \
	    echo "Waiting for Bootstrapper L2 container to start..."; \
	    sleep 1; \
	  done
	@echo "Waiting for Bootstrapper L2 container to finish..."
	@docker wait bootstrapper_l2
	@echo "Running transfer scripts..."
	@cd deps/scripts/transfer_from_L1
	@npm install
	@npm run transfer-l1
	@kill $$(cat process.pid)
	@rm -f process.pid

run-madara: build madara

run-appchain: build appchain