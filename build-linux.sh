#!/bin/bash
# VaultX Linux Build Script
# Run this on Linux to build native binaries
# Prerequisites: Rust, Cargo, build tools

set -e

echo "🔨 Building VaultX for Linux..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "📋 Build Information:"
rustc --version
cargo --version
echo ""

# Detect distro and install dependencies
if [ -f /etc/os-release ]; then
    . /etc/os-release
    case "$ID" in
        ubuntu|debian)
            echo "📦 Installing Ubuntu/Debian dependencies..."
            sudo apt-get update
            sudo apt-get install -y build-essential pkg-config libssl-dev
            ;;
        fedora|rhel|centos)
            echo "📦 Installing Fedora/RHEL dependencies..."
            sudo dnf install -y gcc pkg-config openssl-devel
            ;;
        arch)
            echo "📦 Installing Arch Linux dependencies..."
            sudo pacman -S base-devel --noconfirm
            ;;
        *)
            echo "⚠️  Unknown distro: $ID"
            echo "   Please ensure you have build-essential and pkg-config installed"
            ;;
    esac
fi

echo ""

# Build standard binary
echo "🔨 Building standard binary (x86_64-unknown-linux-gnu)..."
cargo build --release --target x86_64-unknown-linux-gnu --verbose

if [ ! -f target/x86_64-unknown-linux-gnu/release/vx ]; then
    echo "❌ Build failed: vx not found"
    exit 1
fi

mkdir -p dist
cp target/x86_64-unknown-linux-gnu/release/vx dist/vx-linux-x64
strip dist/vx-linux-x64 2>/dev/null || true
chmod +x dist/vx-linux-x64

echo "✅ Built standard binary"

# Build static binary (musl)
echo ""
echo "🔨 Building static binary (x86_64-unknown-linux-musl)..."

# Check if musl target is available
if ! rustup target list | grep -q "x86_64-unknown-linux-musl (installed)"; then
    echo "📦 Installing musl target..."
    rustup target add x86_64-unknown-linux-musl
fi

# Install musl tools if needed
if ! command -v musl-gcc &> /dev/null; then
    echo "📦 Installing musl-tools..."
    if [ -f /etc/os-release ]; then
        case "$ID" in
            ubuntu|debian)
                sudo apt-get install -y musl-tools
                ;;
            fedora|rhel|centos)
                sudo dnf install -y musl-tools
                ;;
            arch)
                sudo pacman -S musl --noconfirm
                ;;
        esac
    fi
fi

cargo build --release --target x86_64-unknown-linux-musl --verbose

if [ -f target/x86_64-unknown-linux-musl/release/vx ]; then
    cp target/x86_64-unknown-linux-musl/release/vx dist/vx-linux-x64-static
    strip dist/vx-linux-x64-static 2>/dev/null || true
    chmod +x dist/vx-linux-x64-static
    echo "✅ Built static binary (musl)"
fi

echo ""
echo "✅ Build completed successfully!"
echo ""
echo "📊 Binary Information:"
ls -lh dist/vx-linux-*

echo ""
echo "📝 Installation Instructions:"
echo ""
echo "Standard binary (dynamic linking):"
echo "  sudo cp dist/vx-linux-x64 /usr/local/bin/vx"
echo ""
echo "Static binary (no dependencies):"
echo "  sudo cp dist/vx-linux-x64-static /usr/local/bin/vx"
echo ""
echo "Verify installation:"
echo "  vx --version"
echo "  vx --help"
