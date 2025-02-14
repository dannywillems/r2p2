build: ## Build the project
		cargo build --all-targets --all-features


release: ## Build the project in release mode
		cargo build --release --all-targets --all-features


test-doc: ## Test the project's docs comments
		cargo test --all-features --release --doc

format: ## Format the code
		cargo +nightly fmt -- --check

lint: ## Lint the code
		cargo clippy --all-features --all-targets --tests -- -W clippy::all -D warnings

.PHONE: build release test-doc format lint
