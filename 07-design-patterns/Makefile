# =======================
# Rust Module Makefile
# =======================

PROJECT_NAME := $(notdir $(CURDIR))
CARGO := cargo
MERMAID := mmdc

.DEFAULT_GOAL := help
.PHONY: design clean-docs check-tools

DOCS_DIR := docs/design
SRC_FILES := $(shell find src -name '*.rs')
RA := rust-analyzer

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
	@echo "  make design       - Generate mermaid diagrams"
	@echo "  make check-tools  - Check required tools are installed"
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

design: check-tools $(DOCS_DIR)/DESIGN.md
	@echo "✅ Design documentation generated."

clean-docs:
	rm -rf $(DOCS_DIR)
	@echo "🧹 Cleaned design documentation."

render:
	@echo "🖼️  Rendering mermaid diagrams to PNG..."
	@for file in $(DOCS_DIR)/*.mmd ; do \
		$(MERMAID) -i $$file -o $${file%.mmd}.png ; \
	done
	@echo "✅ Mermaid diagrams rendered."

check-tools:
	@command -v rust-analyzer >/dev/null || (echo "❌ rust-analyzer missing" && exit 1)
	@command -v jq >/dev/null || (echo "❌ jq missing" && exit 1)
	@command -v dot >/dev/null || (echo "❌ graphviz (dot) missing" && exit 1)
	@command -v mmdc >/dev/null || (echo "❌ mermaid-cli missing" && exit 1)
	@echo "✅ All required tools are installed."

$(DOCS_DIR):
	@mkdir -p $(DOCS_DIR)

# ---------------------------------------------------------
# Workspace metadata
# ---------------------------------------------------------
$(DOCS_DIR)/metadata.json: | $(DOCS_DIR)
	@echo "🔍 Collecting workspace metadata..."
	@$(CARGO) metadata --format-version 1 > $@

# ---------------------------------------------------------
# rust-analyzer analysis
# ---------------------------------------------------------
$(DOCS_DIR)/analysis.json: | $(DOCS_DIR)
	@echo "🧠 Running rust-analyzer analysis..."
	@$(RA) analysis-stats --json > $@

# ---------------------------------------------------------
# Module hierarchy (indent-aware via JSON)
# ---------------------------------------------------------
$(DOCS_DIR)/modules.mmd: $(DOCS_DIR)/analysis.json
	@echo "📐 Generating module hierarchy diagram..."
	@echo "graph TD" > $@
	@jq -r '
	  .crates[] |
	  .modules[]? |
	  select(.parent != null) |
	  "\(.parent) --> \(.name)"
	' $< | sed 's/::/_/g' >> $@

# ---------------------------------------------------------
# Module dependencies
# ---------------------------------------------------------
$(DOCS_DIR)/module_deps.mmd: $(DOCS_DIR)/analysis.json
	@echo "🔗 Generating module dependency graph..."
	@echo "graph LR" > $@
	@jq -r '
	  .crates[] |
	  .dependencies[]? |
	  "\(.from) --> \(.to)"
	' $< | sed 's/::/_/g' >> $@

# ---------------------------------------------------------
# Crate dependencies (workspace-aware)
# ---------------------------------------------------------
$(DOCS_DIR)/crate_deps.mmd: $(DOCS_DIR)/metadata.json
	@echo "📦 Generating crate dependency graph..."
	@echo "graph LR" > $@
	@jq -r '
	  .packages[] as $$p |
	  $$p.dependencies[]? |
	  "\($$p.name) --> \(.name)"
	' $< >> $@

# ---------------------------------------------------------
# File layout (accurate, workspace-safe)
# ---------------------------------------------------------
$(DOCS_DIR)/files.mmd: | $(DOCS_DIR)
	@echo "📁 Generating file layout..."
	@echo "graph TD" > $@
	@if [ -d src ]; then \
		find src -name '*.rs' | \
			sed 's|/|_|g' | \
			awk '{print "src --> " $$0}' >> $@; \
	else \
		echo "⚠️ No src/ directory found, skipping file layout." >&2; \
	fi

# ---------------------------------------------------------
# Assemble DESIGN.md
# ---------------------------------------------------------
$(DOCS_DIR)/DESIGN.md: \
	$(DOCS_DIR)/modules.mmd \
	$(DOCS_DIR)/module_deps.mmd \
	$(DOCS_DIR)/crate_deps.mmd \
	$(DOCS_DIR)/files.mmd
	@echo "📝 Writing DESIGN.md..."
	@echo "# Design Documentation — $(PROJECT_NAME)" > $@
	@for f in $^; do \
		echo "" >> $@; \
		echo "## $$(basename $$f .mmd | tr '_' ' ')" >> $@; \
		echo '```mermaid' >> $@; \
		cat $$f >> $@; \
		echo '```' >> $@; \
	done

