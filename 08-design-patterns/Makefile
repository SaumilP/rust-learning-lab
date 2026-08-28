# Build, test, run, and document every published design-pattern example.

CARGO ?= cargo
RUSTFMT ?= rustfmt

PACKAGES := abstract-factory builder factory factory-method prototype singleton \
	adapter bridge composite decorator facade flyweight proxy \
	chain-of-responsibility command iterator mediator memento observer state \
	strategy template-method visitor
PATTERN_DIRS := \
	creational/abstract-factory \
	creational/builder \
	creational/factory \
	creational/factory-method \
	creational/prototype \
	creational/singleton \
	structural/adapter \
	structural/bridge \
	structural/composite \
	structural/decorator \
	structural/facade \
	structural/flyweight \
	structural/proxy \
	behavioral/chain-of-responsibility \
	behavioral/command \
	behavioral/iterator \
	behavioral/mediator \
	behavioral/memento \
	behavioral/observer \
	behavioral/state \
	behavioral/strategy \
	behavioral/template-method \
	behavioral/visitor
DESIGN_FILES := $(addsuffix /design.puml,$(PATTERN_DIRS))
DESIGN_DOCS := $(addsuffix /DESIGN.md,$(PATTERN_DIRS))
STANDALONE_SOURCES := creational/examples.rs structural/examples.rs behavioral/examples.rs
STANDALONE_BINS := $(patsubst %.rs,build/%,$(STANDALONE_SOURCES))

.DEFAULT_GOAL := help

.PHONY: help list build run run-all test fmt fmt-check lint standalone design-check check clean

help:
	@echo "Rust design-pattern examples"
	@echo
	@echo "  make list                 List pattern packages"
	@echo "  make build                Build every pattern"
	@echo "  make run PATTERN=adapter  Run one pattern"
	@echo "  make run-all              Run every pattern"
	@echo "  make test                 Test every pattern"
	@echo "  make fmt                  Format every pattern"
	@echo "  make fmt-check            Check formatting"
	@echo "  make lint                 Run Clippy with warnings denied"
	@echo "  make standalone           Compile the category overview programs"
	@echo "  make design-check         Check PlantUML and design documentation"
	@echo "  make check                Run all non-mutating checks"
	@echo "  make clean                Remove target/ and build/"

list:
	@printf '%s\n' $(PACKAGES)

build:
	$(CARGO) build --workspace

run:
	@if [ -z "$(PATTERN)" ]; then \
		echo "Usage: make run PATTERN=adapter"; \
		exit 2; \
	fi
	$(CARGO) run --package "$(PATTERN)"

run-all:
	@set -eu; \
	for package in $(PACKAGES); do \
		echo "Running $$package"; \
		$(CARGO) run --quiet --package "$$package"; \
	done

test:
	$(CARGO) test --workspace --all-targets

fmt:
	$(CARGO) fmt --all
	$(RUSTFMT) --edition=2021 $(STANDALONE_SOURCES)

fmt-check:
	$(CARGO) fmt --all -- --check
	$(RUSTFMT) --edition=2021 --check $(STANDALONE_SOURCES)

lint:
	$(CARGO) clippy --workspace --all-targets --all-features -- -D warnings

standalone: $(STANDALONE_BINS)
	@echo "Compiled $(words $(STANDALONE_BINS)) category overview programs"

build/%: %.rs
	@mkdir -p "$(dir $@)"
	rustc --edition=2021 -D warnings "$<" -o "$@"

design-check:
	@set -eu; \
	for file in $(DESIGN_FILES); do \
		grep -q '^@startuml' "$$file"; \
		grep -q '^@enduml' "$$file"; \
	done; \
	for file in $(DESIGN_DOCS); do \
		test -s "$$file"; \
	done
	@echo "Checked $(words $(DESIGN_FILES)) C4-PlantUML designs"

check: fmt-check lint test standalone design-check
	@echo "All design-pattern checks passed"

clean:
	$(CARGO) clean
	rm -rf build
