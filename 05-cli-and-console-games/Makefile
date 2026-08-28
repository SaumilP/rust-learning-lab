# Validate the Cargo workspace and compile dependency-free standalone examples.

CARGO ?= cargo
RUSTC ?= rustc
RUSTFMT ?= rustfmt
BUILD_DIR ?= build
RUST_EDITION ?= 2021
RANDOM_MANIFEST := random_numbers/Cargo.toml

ALL_EXAMPLE_SOURCES := $(shell find . -type f -path '*/examples/*.rs' | sort)
EXAMPLE_SOURCES := $(filter-out ./random_numbers/examples/dice_roller.rs,$(ALL_EXAMPLE_SOURCES))
EXAMPLE_BINS := $(patsubst ./%.rs,$(BUILD_DIR)/%,$(EXAMPLE_SOURCES))

.DEFAULT_GOAL := help

.PHONY: help list build run test fmt fmt-check lint standalone random-check check clean

help:
	@echo "Rust applications: 05-cli-and-console-games"
	@echo
	@echo "  make list                 List Cargo packages and standalone examples"
	@echo "  make build                Build the main Cargo workspace"
	@echo "  make run PACKAGE=name     Run a workspace package"
	@echo "  make test                 Test the main Cargo workspace"
	@echo "  make fmt                  Format all examples and Cargo packages"
	@echo "  make lint                 Run Clippy with warnings denied"
	@echo "  make standalone           Compile dependency-free examples into $(BUILD_DIR)/"
	@echo "  make random-check         Check the separate randomness package"
	@echo "  make check                Run all non-mutating quality checks"
	@echo "  make clean                Remove generated files"

list:
	@$(CARGO) metadata --no-deps --format-version 1 | sed -n 's/.*"name":"\([^"]*\)".*/Cargo package: \1/p'
	@printf 'Example: %s\n' $(ALL_EXAMPLE_SOURCES)

build:
	$(CARGO) build --workspace

run:
	@if [ -z "$(PACKAGE)" ]; then \
		echo "Usage: make run PACKAGE=hangman"; \
		exit 2; \
	fi
	$(CARGO) run --package "$(PACKAGE)"

test:
	$(CARGO) test --workspace --all-targets

fmt:
	$(CARGO) fmt --all
	$(CARGO) fmt --manifest-path $(RANDOM_MANIFEST)
	$(RUSTFMT) --edition=$(RUST_EDITION) $(EXAMPLE_SOURCES)

fmt-check:
	$(CARGO) fmt --all -- --check
	$(CARGO) fmt --manifest-path $(RANDOM_MANIFEST) -- --check
	$(RUSTFMT) --edition=$(RUST_EDITION) --check $(EXAMPLE_SOURCES)

lint:
	$(CARGO) clippy --workspace --all-targets -- -D warnings
	$(CARGO) clippy --manifest-path $(RANDOM_MANIFEST) --all-targets -- -D warnings

standalone: $(EXAMPLE_BINS)
	@echo "Compiled $(words $(EXAMPLE_BINS)) dependency-free examples into $(BUILD_DIR)/"

$(BUILD_DIR)/%: %.rs
	@mkdir -p "$(dir $@)"
	$(RUSTC) --edition=$(RUST_EDITION) -D warnings "$<" -o "$@"

random-check:
	$(CARGO) test --manifest-path $(RANDOM_MANIFEST) --all-targets

check: fmt-check lint test standalone random-check
	@echo "All checks passed for 05-cli-and-console-games"

clean:
	$(CARGO) clean
	$(CARGO) clean --manifest-path $(RANDOM_MANIFEST)
	rm -rf "$(BUILD_DIR)"
