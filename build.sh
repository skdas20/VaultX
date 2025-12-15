#!/bin/bash
# VaultX Master Build Script
# Automatically detects your platform and builds the appropriate binaries
# Supports Linux, macOS, and Windows

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIST_DIR="$PROJECT_DIR/dist"

echo "🔨 VaultX Build System"
echo "═════════════════════════════════════════════════════════"
echo ""

# Detect OS
OS_TYPE="unknown"
if [[ "$OSTYPE" == "linux-gnu"* ]] || [[ "$OSTYPE" == "linux"* ]]; then
    OS_TYPE="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS_TYPE="macos"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "win32" ]]; then
    OS_TYPE="windows"
fi

echo "Detected OS: $OS_TYPE"
echo "Project Dir: $PROJECT_DIR"
echo "Dist Dir: $DIST_DIR"
echo ""

# Create dist directory
mkdir -p "$DIST_DIR"

# Run platform-specific build
case "$OS_TYPE" in
    linux)
        echo "📋 Running Linux build script..."
        "$PROJECT_DIR/build-linux.sh"
        ;;
    macos)
        echo "📋 Running macOS build script..."
        "$PROJECT_DIR/build-macos.sh"
        ;;
    windows)
        echo "📋 Running Windows build script..."
        "$PROJECT_DIR/build-windows.sh"
        ;;
    *)
        echo "❌ Unsupported OS type: $OS_TYPE"
        echo ""
        echo "Please run the appropriate build script manually:"
        echo "  Linux:   ./build-linux.sh"
        echo "  macOS:   ./build-macos.sh"
        echo "  Windows: ./build-windows.sh"
        exit 1
        ;;
esac

echo ""
echo "═════════════════════════════════════════════════════════"
echo "✅ Build complete!"
echo ""
echo "📂 Binaries available in: $DIST_DIR"
echo ""
ls -lh "$DIST_DIR"/vx-* 2>/dev/null || echo "No binaries found"
