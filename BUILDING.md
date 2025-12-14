# Building VaultX - Complete Guide

## TL;DR - Quick Build on Windows

```powershell
# 1. Install Rust from https://rustup.rs/
# 2. Restart terminal
# 3. Run:
cd vaultx
.\build.ps1
```

Binary will be at `target\release\vx.exe` (~2-3 MB)

---

## Expected Binary Sizes

| Platform | Size (optimized) | Size (with UPX) |
|----------|------------------|-----------------|
| Windows x64 | 2-3 MB | 1-2 MB |
| Linux x64 | 2-3 MB | 1-2 MB |
| macOS x64 | 2-3 MB | 1-2 MB |
| macOS ARM | 2-3 MB | 1-2 MB |

**Why so small?**
- Rust compiles to native code (no runtime)
- Static linking (no dependencies)
- Optimized for size (`opt-level = "z"`)
- Stripped debug symbols
- Minimal dependencies

---

## Method 1: Build on Windows (Easiest)

### Prerequisites
1. Install Rust: https://rustup.rs/
2. Restart terminal

### Build
```powershell
cd vaultx
cargo build --release
```

**Output:** `target\release\vx.exe`

### Test
```powershell
.\target\release\vx.exe --version
.\target\release\vx.exe --help
```

---

## Method 2: Build All Platforms (Windows)

### Using the script:
```powershell
cd vaultx
.\build-all.ps1
```

This builds:
- Windows x64
- Linux x64
- macOS x64
- macOS ARM

**Output:** `dist/` folder with all binaries

### Manual cross-compilation:
```powershell
# Install cross
cargo install cross

# Add targets
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build each
cargo build --release --target x86_64-pc-windows-msvc
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-apple-darwin
cross build --release --target aarch64-apple-darwin
```

---

## Method 3: GitHub Actions (Recommended for Releases)

### Setup:
1. Push code to GitHub
2. Create a tag:
   ```powershell
   git tag v0.1.0
   git push origin v0.1.0
   ```
3. GitHub Actions automatically builds all platforms
4. Download from Releases page

### Workflow file:
Already created at `.github/workflows/release.yml`

Builds:
- ✅ Windows x64
- ✅ Linux x64
- ✅ macOS Intel
- ✅ macOS ARM (M1/M2)

---

## Method 4: Build on Linux

```bash
cd vaultx
cargo build --release

# Binary at: target/release/vx
# Size: ~2-3 MB
```

---

## Method 5: Build on macOS

```bash
cd vaultx

# Intel Mac
cargo build --release --target x86_64-apple-darwin

# Apple Silicon (M1/M2)
cargo build --release --target aarch64-apple-darwin

# Universal binary (both)
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo -create \
  target/x86_64-apple-darwin/release/vx \
  target/aarch64-apple-darwin/release/vx \
  -output vx-universal
```

---

## Size Optimization Tips

### 1. Already Applied (in Cargo.toml)
```toml
[profile.release]
opt-level = "z"       # Optimize for size
lto = true            # Link-time optimization
codegen-units = 1     # Better optimization
strip = true          # Strip debug symbols
panic = "abort"       # Smaller panic handler
```

### 2. Additional: UPX Compression
```powershell
# Install UPX from https://upx.github.io/
upx --best --lzma target\release\vx.exe

# Result: ~1-2 MB (50% smaller)
```

### 3. Cargo Bloat Analysis
```powershell
cargo install cargo-bloat
cargo bloat --release -n 10
```

---

## Verification

### Check binary size:
```powershell
# Windows
(Get-Item target\release\vx.exe).Length / 1MB

# Linux/macOS
ls -lh target/release/vx
```

### Run tests:
```powershell
cargo test --release
```

### Verify it works:
```powershell
.\target\release\vx.exe init test
.\target\release\vx.exe add test SECRET
.\target\release\vx.exe get test SECRET
```

---

## Distribution

### Single Binary
The `vx` binary is completely standalone:
- ✅ No dependencies
- ✅ No runtime required
- ✅ No installation needed
- ✅ Just copy and run

### Installation Options

**Windows:**
```powershell
# Option 1: System-wide
copy target\release\vx.exe C:\Windows\System32\vx.exe

# Option 2: User PATH
mkdir C:\Users\$env:USERNAME\bin
copy target\release\vx.exe C:\Users\$env:USERNAME\bin\vx.exe
# Add C:\Users\USERNAME\bin to PATH
```

**Linux/macOS:**
```bash
sudo cp target/release/vx /usr/local/bin/
chmod +x /usr/local/bin/vx
```

---

## Troubleshooting

### "cargo: command not found"
- Restart terminal after installing Rust
- Check: `$env:Path` includes `C:\Users\USERNAME\.cargo\bin`

### Linker errors on Windows
- Install Visual Studio Build Tools
- Download: https://visualstudio.microsoft.com/downloads/
- Select: "Desktop development with C++"

### Cross-compilation fails
- Use `cross` instead of `cargo`
- Or use GitHub Actions (easier)

### Build is slow
- First build takes 2-5 minutes (downloads dependencies)
- Subsequent builds are much faster (~30 seconds)
- Use `cargo build` (debug) for faster iteration

---

## Build Times

| Build Type | Time (first) | Time (incremental) |
|------------|--------------|-------------------|
| Debug | 1-2 min | 10-30 sec |
| Release | 2-5 min | 30-60 sec |
| All platforms | 10-15 min | 5-10 min |

---

## What Gets Built

```
target/
├── release/
│   ├── vx.exe          # Windows binary (2-3 MB)
│   └── deps/           # Dependencies (can delete)
└── x86_64-unknown-linux-gnu/
    └── release/
        └── vx          # Linux binary (2-3 MB)
```

You only need the `vx` or `vx.exe` file!

---

## Next Steps

1. **Build:** `.\build.ps1`
2. **Test:** `.\target\release\vx.exe --help`
3. **Use:** See [QUICKSTART.md](QUICKSTART.md)
4. **Distribute:** See [README.md](README.md)

For automated multi-platform builds, push to GitHub and use the Actions workflow.
