# =======================
# Rust Module Makefile
# =======================

PROJECT_NAME := $(notdir $(CURDIR))
CARGO := cargo

.DEFAULT_GOAL := help

# ----------------------------
# Help
# ----------------------------

help:
	@echo "🦀 Rust Module Makefile for $(PROJECT_NAME)"
	@echo ""
	@echo "Available commands:"
	@echo "  make build        - Build the Rust project"
	@echo "  make run          - Run the Rust project"
	@echo "  make test         - Run tests for the Rust project"
	@echo "  make fmt          - Format the Rust project"
	@echo "  make lint         - Run clippy on the Rust project"
	@echo "  make check        - Run fmt + lint + test"
	@echo "  make clean        - Clean the Rust project"
	@echo ""

build:
	@echo "🔨 Building $(PROJECT_NAME)..."
	@$(CARGO) build

run:
	@echo "🚀 Running $(PROJECT_NAME)..."
	@$(CARGO) run

test:
	@echo "🧪 Testing $(PROJECT_NAME)..."
	@$(CARGO) test

fmt:
	@echo "📝 Formatting $(PROJECT_NAME)..."
	@$(CARGO) fmt

lint:
	@echo "🔍 Linting $(PROJECT_NAME)..."
	@$(CARGO) clippy --all-targets --all_backends -- -D warnings

check:
	@echo "🔍 Checking $(PROJECT_NAME)..."
	@$(CARGO) fmt lint test
	@echo "✅ All checks passed for $(PROJECT_NAME)..."

clean:
	@echo "🧹 Cleaning $(PROJECT_NAME)..."
	@$(CARGO) clean
