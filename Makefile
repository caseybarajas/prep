.PHONY: build release install clean test lint fmt check help

# Default target
help:
	@echo "prep - Makefile commands"
	@echo ""
	@echo "Usage:"
	@echo "  make build     Build debug version"
	@echo "  make release   Build optimized release version"
	@echo "  make install   Install to ~/.cargo/bin"
	@echo "  make test      Run tests"
	@echo "  make lint      Run clippy linter"
	@echo "  make fmt       Format code"
	@echo "  make check     Run all checks (fmt, lint, test)"
	@echo "  make clean     Remove build artifacts"

# Build debug version
build:
	cargo build

# Build release version
release:
	cargo build --release

# Install to cargo bin
install: release
	cargo install --path .

# Run tests
test:
	cargo test

# Run clippy
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Format code
fmt:
	cargo fmt

# Check formatting
fmt-check:
	cargo fmt --all -- --check

# Run all checks
check: fmt-check lint test
	@echo "All checks passed!"

# Clean build artifacts
clean:
	cargo clean

# Generate shell completions
completions:
	@mkdir -p completions
	./target/release/prep completions bash > completions/prep.bash
	./target/release/prep completions zsh > completions/_prep
	./target/release/prep completions fish > completions/prep.fish
	@echo "Shell completions generated in ./completions/"
