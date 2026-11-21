#!/usr/bin/env bash
set -e

echo "=========================================================="
echo "  Rust & Cargo Toolchain Setup for PHP/Laravel Developers"
echo "=========================================================="
echo ""

# 1. Check build essentials
echo "[1/3] Checking system build tools..."
if ! command -v gcc &> /dev/null || ! command -v make &> /dev/null; then
    echo "gcc or make not found. Please install build-essential:"
    echo "  sudo apt update && sudo apt install -y build-essential gcc pkg-config libssl-dev"
    echo ""
fi

# 2. Check if rustup is already installed
if command -v rustup &> /dev/null; then
    echo "[2/3] Rust is already installed!"
    rustc --version
    cargo --version
    exit 0
fi

# 3. Install rustup
echo "[2/3] Downloading and installing rustup (official toolchain)..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# 4. Source cargo environment
echo "[3/3] Activating Rust environment..."
source "$HOME/.cargo/env"

echo ""
echo "=========================================================="
echo "  SUCCESS: Rust and Cargo are installed!"
echo "=========================================================="
rustc --version
cargo --version
echo ""
echo "To ensure Rust is available in all your terminal windows, run:"
echo "  source \"\$HOME/.cargo/env\""
echo "or restart your terminal."
