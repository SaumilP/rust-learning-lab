#!/usr/bin/env bash
set -euo pipefail

echo "🔍 Checking required tools..."

# -----------------------
# Helpers
# -----------------------
has_cmd() {
    command -v "$1" >/dev/null 2>&1
}

install_apt() {
    sudo apt-get update
    sudo apt-get install -y "$@"
}

# -----------------------
# System packages
# -----------------------
APT_PKGS=()

has_cmd dot     || APT_PKGS+=(graphviz)
has_cmd node    || APT_PKGS+=(nodejs)
has_cmd npm     || APT_PKGS+=(npm)
has_cmd dot     || APT_PKGS+=(jq)

if [ ${#APT_PKGS[@]} -gt 0 ]; then
    echo "📦 Installing apt packages: ${APT_PKGS[*]}"
    install_apt "${APT_PKGS[@]}"
else
    echo "✅ System packages already installed."
fi

# -----------------------
# Cargo tools
# -----------------------
if ! has_cmd cargo; then
    echo "❌ Cargo is not installed."
    echo "➡️  Install Rust first: https://rustup.rs"
    exit 1
fi

if ! cargo modules --help >/dev/null 2>&1; then
    echo "📦 Installing cargo-modules..."
    cargo install cargo-modules
else
    echo "✅ cargo-modules already installed."
fi

if ! cargo deps --help >/dev/null 2>&1; then
    echo "📦 Installing cargo-deps..."
    cargo install cargo-deps
else
    echo "✅ cargo-deps already installed."
fi

# -----------------------
# Mermaid CLI
# -----------------------
if ! has_cmd mmdc; then
    echo "📦 Installing Mermaid CLI..."
    npm install -g @mermaid-js/mermaid-cli
else
    echo "✅ Mermaid CLI already installed."
fi

# -----------------------
# Final check
# -----------------------
echo ""
echo "🎉 All required tools are installed:"
echo "✔ graphviz (dot)"
echo "✔ jq (dot)"
echo "✔ node / npm"
echo "✔ cargo-modules"
echo "✔ cargo-deps"
echo "✔ mermaid-cli (mmdc)"
echo ""