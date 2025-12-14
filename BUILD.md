# VaultX Build Guide

## Prerequisites

### Install Rust (All Platforms)

**Windows:**
```powershell
# Download and run rustup-init.exe from:
# https://rustup.rs/

# Or via winget:
winget install Rustlang.Rustup
```

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:
```bash
rustc --version
cargo --version
```

## Building for Windows (on Windows)

```powershell
# Navigate to project
cd vaultx

# Build release version
cargo build --release

# Binary location:
# target\release\vx.exe

# Test it
.\target\release\vx.exe --version

# Optional: Strip debug symbols to reduce size
# (requires installing cargo-strip)
cargo install cargo-strip
cargo strip --release
```

**Expected size:** ~3-4 MB (2-3 MB after strip)

## Building for Linux (on Windows via WSL)

### Option 1: Use WSL (Windows Subsystem for Linux)

```powershell
# Install WSL if not already installed
wsl --install

# Enter WSL
wsl

# Inside WSL:
cd /mnt/c/path/to/vaultx
cargo build --release --target x86_64-unknown-linux-gnu

# Binary at: target/x86_64-unknown-linux-gnu/release/vx
```

### Option 2: Cross-compile (Advanced)

```powershell
# Add Linux target
rustup target add x86_64-unknown-linux-gnu

# Install cross-compilation tool
cargo install cross

# Build for Linux
cross build --release --target x86_64-unknown-linux-gnu

# Binary at: target\x86_64-unknown-linux-gnu\release\vx
```

## Building for macOS

**You need a Mac or GitHub Actions** - Apple doesn't allow cross-compilation from Windows easily.

Use GitHub Actions (see below) or:

```bash
# On macOS:
cd vaultx

# Intel Mac
cargo build --release --target x86_64-apple-darwin

# Apple Silicon (M1/M2)
cargo build --release --target aarch64-apple-darwin
```

## Automated Multi-Platform Builds (Recommended)

Use GitHub Actions to build all platforms automatically.

### Setup:

1. Push code to GitHub
2. Create a release tag:
   ```powershell
   git tag v0.1.0
   git push origin v0.1.0
   ```
3. GitHub Actions will build all platforms automatically

Binaries will be available in the release page.

## Manual Build All Platforms

If you have access to all platforms:

```bash
# Windows
cargo build --release --target x86_64-pc-windows-msvc

# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# macOS Intel
cargo build --release --target x86_64-apple-darwin

# macOS ARM (M1/M2)
cargo build --release --target aarch64-apple-darwin
```

## Size Optimization

To make binaries even smaller:

### 1. Add to `vx-cli/Cargo.toml`:

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
strip = true        # Strip symbols
panic = "abort"     # Smaller panic handler
```

### 2. Use UPX compression (optional):

```powershell
# Install UPX
# Windows: Download from https://upx.github.io/

# Compress binary
upx --best --lzma target\release\vx.exe
```

**Result:** Can reduce to ~1-2 MB

## Testing the Build

```powershell
# Run tests
cargo test

# Run property-based tests
cargo test --release

# Test the binary
.\target\release\vx.exe init test-project
.\target\release\vx.exe add test-project MY_SECRET
.\target\release\vx.exe get test-project MY_SECRET
.\target\release\vx.exe audit
```

## Distribution

### Single Binary Distribution

The `vx.exe` (or `vx`) binary is completely standalone:
- No dependencies
- No runtime required
- Just copy and run

### Installation

**Windows:**
```powershell
# Copy to a directory in PATH
copy target\release\vx.exe C:\Windows\System32\vx.exe

# Or add to user PATH
$env:Path += ";C:\path\to\vaultx\target\release"
```

**Linux/macOS:**
```bash
# Copy to /usr/local/bin
sudo cp target/release/vx /usr/local/bin/

# Or add to PATH
export PATH="$PATH:/path/to/vaultx/target/release"
```

## Troubleshooting

### "cargo: command not found"

Restart your terminal after installing Rust, or run:
```powershell
# Windows
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
```

### Build fails with linker errors

```powershell
# Install Visual Studio Build Tools (Windows)
# Download from: https://visualstudio.microsoft.com/downloads/
# Select "Desktop development with C++"
```

### Cross-compilation issues

Use `cross` instead of `cargo`:
```powershell
cargo install cross
cross build --release --target x86_64-unknown-linux-gnu
```
