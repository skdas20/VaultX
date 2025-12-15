#!/bin/bash
# VaultX Windows Build Script
# Run this on Windows with Git Bash or WSL
# Prerequisites: Rust, Cargo, Visual Studio Build Tools

set -e

echo "🔨 Building VaultX for Windows..."
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

# Add Windows target if not already present
echo "📦 Setting up Windows target..."
rustup target add x86_64-pc-windows-msvc 2>/dev/null || true

# Build for Windows
echo "🔨 Compiling VaultX..."
cargo build --release --target x86_64-pc-windows-msvc --verbose

# Check if build succeeded
if [ ! -f "target/x86_64-pc-windows-msvc/release/vx.exe" ]; then
    echo "❌ Build failed: vx.exe not found"
    exit 1
fi

# Copy to dist folder
mkdir -p dist
cp target/x86_64-pc-windows-msvc/release/vx.exe dist/vx-windows-x64.exe
chmod +x dist/vx-windows-x64.exe

echo ""
echo "✅ Build completed successfully!"
echo ""
echo "📊 Binary Information:"
ls -lh dist/vx-windows-x64.exe

echo ""
echo "📝 Installation Instructions:"
echo "1. Move vx-windows-x64.exe to a folder in your PATH"
echo "   Example: C:\\Program Files\\vx\\"
echo ""
echo "2. Rename it to vx.exe (optional but recommended)"
echo ""
echo "3. Verify installation:"
echo "   vx --version"
