#!/bin/bash
# VaultX macOS Build Script
# Run this on macOS to build native binaries
# Prerequisites: Xcode Command Line Tools, Rust, Cargo

set -e

echo "🔨 Building VaultX for macOS..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if Xcode CLT is installed
if ! command -v xcode-select &> /dev/null; then
    echo "❌ Xcode Command Line Tools not found."
    echo "   Install with: xcode-select --install"
    exit 1
fi

echo "📋 Build Information:"
rustc --version
cargo --version
echo ""

# Detect architecture
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    NATIVE_TARGET="aarch64-apple-darwin"
    NATIVE_NAME="vx-macos-arm64"
    CROSS_TARGET="x86_64-apple-darwin"
    CROSS_NAME="vx-macos-x64"
    BUILD_CROSS=true
else
    NATIVE_TARGET="x86_64-apple-darwin"
    NATIVE_NAME="vx-macos-x64"
    CROSS_TARGET="aarch64-apple-darwin"
    CROSS_NAME="vx-macos-arm64"
    BUILD_CROSS=true
fi

echo "🖥️  Detected architecture: $ARCH"
echo ""

# Add targets
echo "📦 Setting up build targets..."
rustup target add "$NATIVE_TARGET" 2>/dev/null || true
if [ "$BUILD_CROSS" = true ]; then
    rustup target add "$CROSS_TARGET" 2>/dev/null || true
fi

# Build for native architecture
echo "🔨 Building for $NATIVE_TARGET..."
cargo build --release --target "$NATIVE_TARGET" --verbose

# Check if build succeeded
if [ ! -f "target/$NATIVE_TARGET/release/vx" ]; then
    echo "❌ Build failed: vx not found for $NATIVE_TARGET"
    exit 1
fi

# Copy to dist
mkdir -p dist
cp "target/$NATIVE_TARGET/release/vx" "dist/$NATIVE_NAME"
strip "dist/$NATIVE_NAME" 2>/dev/null || true
chmod +x "dist/$NATIVE_NAME"

echo "✅ Built for $NATIVE_NAME"

# Build for other architecture if on Apple Silicon
if [ "$BUILD_CROSS" = true ]; then
    echo ""
    echo "🔨 Building for $CROSS_TARGET (universal binary support)..."
    cargo build --release --target "$CROSS_TARGET" --verbose
    
    if [ -f "target/$CROSS_TARGET/release/vx" ]; then
        cp "target/$CROSS_TARGET/release/vx" "dist/$CROSS_NAME"
        strip "dist/$CROSS_NAME" 2>/dev/null || true
        chmod +x "dist/$CROSS_NAME"
        echo "✅ Built for $CROSS_NAME"
        
        # Create universal binary
        echo ""
        echo "🌍 Creating universal binary..."
        lipo -create \
            "dist/$NATIVE_NAME" \
            "dist/$CROSS_NAME" \
            -output dist/vx-macos-universal
        chmod +x dist/vx-macos-universal
        echo "✅ Created universal binary (vx-macos-universal)"
    fi
fi

echo ""
echo "✅ Build completed successfully!"
echo ""
echo "📊 Binary Information:"
ls -lh dist/vx-macos-*

echo ""
echo "📝 Installation Instructions:"
echo "1. Install your binary:"
if [ -f "dist/vx-macos-universal" ]; then
    echo "   sudo cp dist/vx-macos-universal /usr/local/bin/vx"
else
    echo "   sudo cp dist/$NATIVE_NAME /usr/local/bin/vx"
fi
echo ""
echo "2. Verify installation:"
echo "   vx --version"
echo "   vx --help"
