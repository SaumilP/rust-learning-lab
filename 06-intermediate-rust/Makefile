# Build and document every standalone Rust example in this module.

PROJECT_NAME := $(notdir $(CURDIR))
RUSTC ?= rustc
RUSTDOC ?= rustdoc
RUSTFMT ?= rustfmt
BUILD_DIR ?= build
RUST_EDITION ?= 2021

# Supplementary review examples are Draft material and are intentionally outside
# the canonical module checks until their own validation is added.
EXAMPLE_SOURCES := $(shell find . -type f -path '*/examples/*.rs' ! -path './supplementary/*' | sort)
EXAMPLE_BINS := $(patsubst ./%.rs,$(BUILD_DIR)/%,$(EXAMPLE_SOURCES))

.DEFAULT_GOAL := help

.PHONY: help list build run run-all test docs fmt fmt-check check clean

help:
	@echo "Rust examples: $(PROJECT_NAME)"
	@echo
	@echo "  make list                         List example source files"
	@echo "  make build                        Compile every example into $(BUILD_DIR)/"
	@echo "  make run EXAMPLE=path/to/file.rs  Compile and run one example"
	@echo "  make run-all                      Compile and run every example"
	@echo "  make test                         Compile and run embedded unit tests"
	@echo "  make docs                         Generate rustdoc pages in $(BUILD_DIR)/docs/"
	@echo "  make fmt                          Format every example"
	@echo "  make fmt-check                    Check formatting without changing files"
	@echo "  make check                        Run all non-mutating quality checks"
	@echo "  make clean                        Remove generated files"

list:
	@printf '%s\n' $(EXAMPLE_SOURCES)

build: $(EXAMPLE_BINS)
	@echo "Compiled $(words $(EXAMPLE_BINS)) examples into $(BUILD_DIR)/"

$(BUILD_DIR)/%: %.rs
	@mkdir -p "$(dir $@)"
	$(RUSTC) --edition=$(RUST_EDITION) -D warnings "$<" -o "$@"

run:
	@if [ -z "$(EXAMPLE)" ]; then \
		echo "Usage: make run EXAMPLE=path/to/file.rs"; \
		exit 2; \
	fi
	@source="$(EXAMPLE)"; \
	source=$${source#./}; \
	if [ ! -f "$$source" ]; then \
		echo "Example not found: $$source"; \
		exit 2; \
	fi; \
	binary="$(BUILD_DIR)/$${source%.rs}"; \
	$(MAKE) "$$binary"; \
	"$$binary"

run-all: build
	@set -eu; \
	for binary in $(EXAMPLE_BINS); do \
		echo "Running $$binary"; \
		"$$binary"; \
	done

test:
	@mkdir -p "$(BUILD_DIR)/tests"
	@set -eu; \
	for source in $(EXAMPLE_SOURCES); do \
		name=$$(printf '%s' "$$source" | sed 's|^./||; s|\.rs$$||; s|/|__|g'); \
		binary="$(BUILD_DIR)/tests/$$name"; \
		echo "Testing $$source"; \
		$(RUSTC) --edition=$(RUST_EDITION) -D warnings --test "$$source" -o "$$binary"; \
		"$$binary"; \
	done

docs:
	@mkdir -p "$(BUILD_DIR)/docs"
	@set -eu; \
	for source in $(EXAMPLE_SOURCES); do \
		relative=$${source#./}; \
		output="$(BUILD_DIR)/docs/$${relative%.rs}"; \
		crate_name=$$(printf '%s' "$${relative%.rs}" | tr '/-' '__'); \
		echo "Documenting $$source"; \
		mkdir -p "$$output"; \
		$(RUSTDOC) --edition=$(RUST_EDITION) -D warnings \
			--crate-name "$$crate_name" "$$source" --out-dir "$$output"; \
	done
	@echo "Generated documentation in $(BUILD_DIR)/docs/"

fmt:
	$(RUSTFMT) --edition=$(RUST_EDITION) $(EXAMPLE_SOURCES)

fmt-check:
	$(RUSTFMT) --edition=$(RUST_EDITION) --check $(EXAMPLE_SOURCES)

check: fmt-check build test docs
	@echo "All checks passed for $(PROJECT_NAME)"

clean:
	rm -rf "$(BUILD_DIR)"
