# Shared commands for one design-pattern Cargo package.

CARGO ?= cargo

.DEFAULT_GOAL := help

.PHONY: help build run test fmt fmt-check lint check clean

help:
	@echo "Design pattern: $(notdir $(CURDIR))"
	@echo
	@echo "  make build       Build the example"
	@echo "  make run         Run the example"
	@echo "  make test        Run its tests"
	@echo "  make fmt         Format its Rust code"
	@echo "  make fmt-check   Check Rust formatting"
	@echo "  make lint        Run Clippy with warnings denied"
	@echo "  make check       Run all non-mutating checks"
	@echo "  make clean       Remove this package's Cargo artifacts"

build:
	$(CARGO) build --manifest-path Cargo.toml

run:
	$(CARGO) run --manifest-path Cargo.toml

test:
	$(CARGO) test --manifest-path Cargo.toml --all-targets

fmt:
	$(CARGO) fmt --manifest-path Cargo.toml

fmt-check:
	$(CARGO) fmt --manifest-path Cargo.toml -- --check

lint:
	$(CARGO) clippy --manifest-path Cargo.toml --all-targets -- -D warnings

check: fmt-check lint test
	@echo "All checks passed for $(notdir $(CURDIR))"

clean:
	$(CARGO) clean --manifest-path Cargo.toml
