.PHONY: all build test install clean release help

# Default target
all: build

# Build the project
build:
	@echo "Building DB CLI..."
	@cargo build --release

# Run tests
test:
	@echo "Running tests..."
	@cargo test
	@cargo test --test '*' -- --test-threads=1

# Install to system
install: build
	@echo "Installing DB CLI..."
	@sudo cp target/release/db /usr/local/bin/db
	@sudo chmod +x /usr/local/bin/db
	@echo "DB CLI installed successfully!"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf target

# Build release for all platforms
release:
	@echo "Building releases..."
	@cargo build --release --target x86_64-unknown-linux-gnu
	@cargo build --release --target x86_64-apple-darwin
	@cargo build --release --target aarch64-apple-darwin

# Check code quality
check:
	@echo "Running code quality checks..."
	@cargo fmt -- --check
	@cargo clippy -- -D warnings

# Format code
fmt:
	@echo "Formatting code..."
	@cargo fmt

# Show help
help:
	@echo "Available targets:"
	@echo "  make build   - Build the project"
	@echo "  make test    - Run tests"
	@echo "  make install - Install to system"
	@echo "  make clean   - Clean build artifacts"
	@echo "  make release - Build for all platforms"
	@echo "  make check   - Run code quality checks"
	@echo "  make fmt     - Format code"
	@echo "  make help    - Show this help"