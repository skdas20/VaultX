# Building VaultX for Your Platform

VaultX can be built on Windows, macOS, and Linux. Choose your platform below.

## Quick Start - Pre-built Binaries

If you want to skip building and just use the tool:

### Linux
```bash
curl -L https://github.com/skdas20/VaultX/releases/download/latest/vx-linux-x64 -o vx
chmod +x vx
sudo mv vx /usr/local/bin/
```

### Windows
```powershell
# Download from GitHub releases
# https://github.com/skdas20/VaultX/releases

# Then run:
vx --version
```

### macOS
```bash
# Download for your architecture:
# Intel Mac (x86_64):
curl -L https://github.com/skdas20/VaultX/releases/download/latest/vx-macos-x64 -o vx

# Apple Silicon (ARM64):
curl -L https://github.com/skdas20/VaultX/releases/download/latest/vx-macos-arm64 -o vx

chmod +x vx
sudo mv vx /usr/local/bin/
```

---

## Building from Source

### Prerequisites

#### All Platforms
- **Rust 1.70+** - [Install from rustup.rs](https://rustup.rs/)
- **Cargo** - Comes with Rust

Verify installation:
```bash
rustc --version
cargo --version
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get install -y build-essential pkg-config libssl-dev
```

#### Linux (Fedora/RHEL)
```bash
sudo dnf install -y gcc pkg-config openssl-devel
```

#### macOS
- Xcode Command Line Tools (automatically installed when needed)
- Or install via: `xcode-select --install`

#### Windows
- [Visual Studio 2017 or later](https://www.visualstudio.com/)
- Or [Build Tools for Visual Studio](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with C++ workload

---

## Build Instructions

### Linux

#### Standard Build (dynamic linking)
```bash
cd VaultX
cargo build --release
# Binary: target/release/vx
sudo cp target/release/vx /usr/local/bin/vx
```

#### Static Build (for distribution)
```bash
cd VaultX
cargo build --release --target x86_64-unknown-linux-musl
# Binary: target/x86_64-unknown-linux-musl/release/vx
sudo cp target/x86_64-unknown-linux-musl/release/vx /usr/local/bin/vx-static
```

### macOS

#### Intel Mac (x86_64)
```bash
cd VaultX
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
# Binary: target/x86_64-apple-darwin/release/vx
sudo cp target/x86_64-apple-darwin/release/vx /usr/local/bin/vx
```

#### Apple Silicon (ARM64)
```bash
cd VaultX
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
# Binary: target/aarch64-apple-darwin/release/vx
sudo cp target/aarch64-apple-darwin/release/vx /usr/local/bin/vx
```

#### Universal Binary (works on both Intel and Apple Silicon)
```bash
cd VaultX

# Build for both architectures
rustup target add x86_64-apple-darwin aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create universal binary
lipo -create \
  target/x86_64-apple-darwin/release/vx \
  target/aarch64-apple-darwin/release/vx \
  -output /usr/local/bin/vx

chmod +x /usr/local/bin/vx
```

### Windows

#### PowerShell (Recommended)
```powershell
cd VaultX
cargo build --release --target x86_64-pc-windows-msvc
# Binary: target\x86_64-pc-windows-msvc\release\vx.exe

# Copy to your PATH directory
Copy-Item -Path "target\x86_64-pc-windows-msvc\release\vx.exe" -Destination "C:\Program Files\vx\vx.exe"
```

#### Command Prompt
```cmd
cd VaultX
cargo build --release --target x86_64-pc-windows-msvc
REM Binary: target\x86_64-pc-windows-msvc\release\vx.exe

REM Copy to your PATH directory (requires admin)
copy target\x86_64-pc-windows-msvc\release\vx.exe "C:\Program Files\vx\vx.exe"
```

---

## Verify Installation

After building and installing, verify everything works:

```bash
vx --version
# Output: vx 0.1.0

vx --help
# Shows all available commands
```

Test with a quick workflow:
```bash
# Create a test vault
vx init my-project

# Add a secret
vx add my-project DB_PASSWORD

# List projects
vx list

# Get a secret
vx get my-project DB_PASSWORD
```

---

## Cross-Platform Builds

### Using Docker (Linux to Windows)
```bash
# Install cross
cargo install cross

# Build for Windows
cross build --release --target x86_64-pc-windows-gnu
```

### Using GitHub Actions (Automated)

Push a git tag to trigger automatic builds on all platforms:
```bash
git tag v0.2.0
git push origin v0.2.0
```

This triggers the `.github/workflows/release.yml` workflow which builds binaries for:
- Windows (x86_64)
- Linux (x86_64)
- macOS (Intel x86_64)
- macOS (Apple Silicon ARM64)

Binaries are automatically uploaded to GitHub Releases.

---

## Troubleshooting

### "cargo: command not found"
Install Rust from [rustup.rs](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### "link.exe not found" (Windows)
Install Visual Studio Build Tools with C++ workload:
- Download: https://visualstudio.microsoft.com/visual-cpp-build-tools/
- Run installer and select "C++ build tools"
- Restart your terminal after installation

### "xcrun not found" (macOS)
Install Xcode Command Line Tools:
```bash
xcode-select --install
```

### Build takes too long
Use the pre-built binaries from GitHub Releases instead. The first build compiles all dependencies, which can take 5-10 minutes depending on your system.

---

## Build Performance

**First Build**: 5-10 minutes (compiles all dependencies)
**Subsequent Builds**: 30 seconds - 2 minutes

To speed up builds on multi-core systems:
```bash
CARGO_BUILD_JOBS=$(nproc) cargo build --release
```

---

## Binary Locations After Build

- **Linux**: `target/release/vx` or `target/x86_64-unknown-linux-musl/release/vx`
- **macOS**: `target/aarch64-apple-darwin/release/vx` or `target/x86_64-apple-darwin/release/vx`
- **Windows**: `target\x86_64-pc-windows-msvc\release\vx.exe`

---

## Next Steps

After installation:
1. Run `vx --help` to see all commands
2. Check [README.md](README.md) for usage examples
3. Read [QUICKSTART.md](QUICKSTART.md) for getting started guide

