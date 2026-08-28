# Validate the Cargo workspace and compile the standalone examples.

CARGO ?= cargo
RUSTC ?= rustc
RUSTFMT ?= rustfmt
BUILD_DIR ?= build
RUST_EDITION ?= 2021

EXAMPLE_SOURCES := $(shell find . -type f -path '*/examples/*.rs' | sort)
EXAMPLE_BINS := $(patsubst ./%.rs,$(BUILD_DIR)/%,$(EXAMPLE_SOURCES))

.DEFAULT_GOAL := help

.PHONY: help list build run test fmt fmt-check lint standalone check clean

help:
	@echo "Rust applications: 04-simple-programs"
	@echo
	@echo "  make list                 List Cargo packages and standalone examples"
	@echo "  make build                Build the Cargo workspace"
	@echo "  make run PACKAGE=name     Run one Cargo package"
	@echo "  make test                 Test the Cargo workspace"
	@echo "  make fmt                  Format the workspace and standalone examples"
	@echo "  make lint                 Run Clippy with warnings denied"
	@echo "  make standalone           Compile standalone examples into $(BUILD_DIR)/"
	@echo "  make check                Run all non-mutating quality checks"
	@echo "  make clean                Remove generated files"

list:
	@$(CARGO) metadata --no-deps --format-version 1 | sed -n 's/.*"name":"\([^"]*\)".*/Cargo package: \1/p'
	@printf 'Example: %s\n' $(EXAMPLE_SOURCES)

build:
	$(CARGO) build --workspace

run:
	@if [ -z "$(PACKAGE)" ]; then \
		echo "Usage: make run PACKAGE=calculator"; \
		exit 2; \
	fi
	$(CARGO) run --package "$(PACKAGE)"

test:
	$(CARGO) test --workspace --all-targets

fmt:
	$(CARGO) fmt --all
	$(RUSTFMT) --edition=$(RUST_EDITION) $(EXAMPLE_SOURCES)

fmt-check:
	$(CARGO) fmt --all -- --check
	$(RUSTFMT) --edition=$(RUST_EDITION) --check $(EXAMPLE_SOURCES)

lint:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

standalone: $(EXAMPLE_BINS)
	@echo "Compiled $(words $(EXAMPLE_BINS)) standalone examples into $(BUILD_DIR)/"

$(BUILD_DIR)/%: %.rs
	@mkdir -p "$(dir $@)"
	$(RUSTC) --edition=$(RUST_EDITION) -D warnings "$<" -o "$@"

check: fmt-check lint test standalone
	@echo "All checks passed for 04-simple-programs"

clean:
	$(CARGO) clean
	rm -rf "$(BUILD_DIR)"
