.PHONY: help fmt fmt-check clippy test check build release clean

help: ## Show available targets
	@grep -E '^[a-zA-Z_-]+:.*?## ' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-12s %s\n", $$1, $$2}'

fmt: ## Format all workspace crates
	cargo fmt --all

fmt-check: ## Check formatting without modifying files
	cargo fmt --all --check

clippy: ## Run Clippy with warnings denied
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test: ## Run all workspace tests
	cargo test --workspace --all-features

check: fmt-check clippy test ## Run fmt-check, clippy, and test

build: ## Build workspace (debug)
	cargo build --workspace

release: ## Build workspace (release)
	cargo build --workspace --release

clean: ## Remove build artifacts
	cargo clean
