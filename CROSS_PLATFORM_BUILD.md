# VaultX Cross-Platform Build Guide

## Summary

VaultX CLI tool has been successfully compiled for **Linux**. Below are instructions for building on each platform.

### ✅ Completed Builds

| Platform | Status | Binary | Location |
|----------|--------|--------|----------|
| Linux x64 | ✅ Done | `vx-linux-x64` | `dist/vx-linux-x64` |
| Windows x64 | 📝 Instructions Below | | |
| macOS x64 | 📝 Instructions Below | | |
| macOS ARM64 (M1/M2) | 📝 Instructions Below | | |

---

## Building on Windows (Recommended for Windows/All Platforms)

### Prerequisites

1. **Install Rust & Cargo**
   - Download from: https://rustup.rs/
   - Run the installer and restart your terminal

2. **Verify Installation**
   ```powershell
   rustc --version
   cargo --version
   ```

### Building for Windows (on Windows)

```powershell
cd path\to\VaultX
cargo build --release
```

**Output:** `target\release\vx.exe` (~2-3 MB)

### Building for Linux (on Windows)

**Option A: Using `cross` (Recommended)**

```powershell
# Install cross
cargo install cross

# Build for Linux
cross build --release --target x86_64-unknown-linux-gnu
```

**Output:** `target\x86_64-unknown-linux-gnu\release\vx` (~2-3 MB)

### Building for macOS (on Windows)

**⚠️ Important:** macOS binaries can only be built on macOS. You have two options:

**Option A: Use GitHub Actions (Easiest)**
1. Push code to GitHub
2. Tag a release: `git tag v1.0.0 && git push origin v1.0.0`
3. GitHub Actions automatically builds all platforms

**Option B: Build on Mac Manually**
```bash
# On macOS Intel
cargo build --release --target x86_64-apple-darwin

# On macOS ARM (M1/M2)
cargo build --release --target aarch64-apple-darwin
```

---

## Building All Platforms at Once (Windows)

We've provided a PowerShell script that automates multi-platform builds:

```powershell
cd path\to\VaultX
.\build-all.ps1
```

This will:
- Build for Windows x64
- Build for Linux x64 (using `cross`)
- Build for macOS x64 (if on Mac)
- Build for macOS ARM64 (if on Mac)
- Output all binaries to `dist/` folder

---

## Building on Linux (Your Current Environment)

### Linux x64 (Native - ✅ Completed)

```bash
cargo build --release
```

**Output:** `target/release/vx` (~2-3 MB)

### Windows x64 (on Linux)

```bash
# Install cross
cargo install cross

# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Note: Direct Windows GNU cross-compilation may require additional dependencies
# Recommended: Use Windows machine for native Windows builds
```

### macOS (on Linux)

**Not recommended on Linux.** Build on macOS:

```bash
# On macOS Intel
cargo build --release --target x86_64-apple-darwin

# On macOS ARM (M1/M2)
cargo build --release --target aarch64-apple-darwin
```

---

## Distribution & Installation

### Linux

```bash
# Copy to /usr/local/bin
sudo cp target/release/vx /usr/local/bin/

# Or add to PATH
export PATH="$PATH:$(pwd)/target/release"

# Verify
vx --version
vx --help
```

### Windows

```powershell
# Copy to a directory in PATH
Copy-Item target\release\vx.exe C:\Windows\System32\

# Or add to PATH environment variable
$env:Path += ";C:\path\to\VaultX\target\release"

# Verify
vx --version
vx --help
```

### macOS

```bash
# Copy to /usr/local/bin
sudo cp target/release/vx /usr/local/bin/

# Or add to PATH
export PATH="$PATH:$(pwd)/target/release"

# Make executable
chmod +x /usr/local/bin/vx

# Verify
vx --version
vx --help
```

---

## Binary Information

### Size Optimization

Binaries are already optimized for size in `Cargo.toml`:

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
strip = true        # Strip debug symbols
panic = "abort"     # Smaller panic handler
```

**Expected sizes:** 2-3 MB per binary

### Further Size Reduction (Optional)

Install UPX compression tool:

```bash
# Linux/macOS
brew install upx

# Windows
# Download from https://upx.github.io/
```

Compress binary:

```bash
upx --best --lzma target/release/vx
```

**Result:** Reduces to ~1-2 MB

---

## Testing the Build

After building, test the binary:

```bash
# Test version
./target/release/vx --version

# Test help
./target/release/vx --help

# Test initialization
./target/release/vx init test-project

# Test adding credentials
./target/release/vx add test-project MY_SECRET "my-secret-value"

# Test retrieving
./target/release/vx get test-project MY_SECRET

# Test audit
./target/release/vx audit
```

---

## GitHub Actions Automated Builds (Recommended for Releases)

We recommend using GitHub Actions for automated multi-platform builds:

1. Push code to GitHub repository
2. Create a release tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
3. GitHub Actions workflow automatically builds:
   - Windows x64
   - Linux x64
   - macOS x64
   - macOS ARM64
4. Binaries available in GitHub Releases

---

## Troubleshooting

### "cargo: command not found"

**Windows:**
```powershell
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
```

**Linux/macOS:**
```bash
source "$HOME/.cargo/env"
```

### Cross-compilation Issues

```bash
# Update cross
cargo install cross --force

# Update Rust toolchain
rustup update
```

### Windows Build Fails with Linker Errors

Install Visual Studio Build Tools:
- Download from: https://visualstudio.microsoft.com/downloads/
- Select "Desktop development with C++"

### macOS Build Issues

Ensure you have Xcode Command Line Tools:

```bash
xcode-select --install
```

---

## Next Steps

1. ✅ **Linux binary created:** `dist/vx-linux-x64`
2. 📝 **Windows:** Build on Windows machine using instructions above
3. 📝 **macOS:** Build on Mac or use GitHub Actions

All binaries are standalone - no dependencies or runtime required. Just copy and run!
