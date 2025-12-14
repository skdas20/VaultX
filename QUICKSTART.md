# VaultX Quick Start

## Step 1: Install Rust (if not installed)

**Windows:**
```powershell
# Visit https://rustup.rs/ and download rustup-init.exe
# Or use winget:
winget install Rustlang.Rustup
```

After installation, **restart your terminal**.

## Step 2: Build VaultX

```powershell
# Navigate to the vaultx directory
cd vaultx

# Build the release version (takes 2-5 minutes first time)
cargo build --release

# The binary will be at: target\release\vx.exe
```

## Step 3: Test It

```powershell
# Check version
.\target\release\vx.exe --version

# See all commands
.\target\release\vx.exe --help
```

## Step 4: Use VaultX

### Initialize a project
```powershell
.\target\release\vx.exe init my-project
# Enter a master password when prompted
```

### Add a secret
```powershell
.\target\release\vx.exe add my-project DB_PASSWORD
# Enter master password
# Enter secret value
```

### Retrieve a secret
```powershell
.\target\release\vx.exe get my-project DB_PASSWORD
# Enter master password
# Secret is printed to stdout
```

### Add secret with TTL (auto-expires)
```powershell
.\target\release\vx.exe add my-project TEMP_TOKEN --ttl 6h
```

### Security audit
```powershell
.\target\release\vx.exe audit
```

### SSH identity management
```powershell
# Generate SSH key
.\target\release\vx.exe ssh init my-server

# Connect using stored key
.\target\release\vx.exe ssh connect my-server user@hostname
```

## Step 5: Install Globally (Optional)

**Option 1: Copy to System32**
```powershell
copy target\release\vx.exe C:\Windows\System32\vx.exe
```

**Option 2: Add to PATH**
```powershell
# Add the directory to your PATH
$env:Path += ";$PWD\target\release"

# Make it permanent (run as admin)
[Environment]::SetEnvironmentVariable("Path", $env:Path, [System.EnvironmentVariableTarget]::User)
```

Now you can use `vx` from anywhere:
```powershell
vx --version
vx init my-project
```

## Binary Sizes

After `cargo build --release`:
- **vx.exe**: ~2-3 MB (with optimizations)
- **Vault file**: Few KB (grows with secrets)

## Building for Other Platforms

See [BUILD.md](BUILD.md) for:
- Linux builds
- macOS builds
- Cross-compilation
- GitHub Actions automation

## Troubleshooting

### "cargo: command not found"
- Restart your terminal after installing Rust
- Or run: `refreshenv` (if using Chocolatey)

### Build errors
- Make sure you have Visual Studio Build Tools installed
- Download from: https://visualstudio.microsoft.com/downloads/
- Select "Desktop development with C++"

### Permission denied
- Run PowerShell as Administrator
- Or use `Set-ExecutionPolicy RemoteSigned -Scope CurrentUser`

## Next Steps

- Read [README.md](README.md) for full documentation
- Check [docs/architecture.md](docs/architecture.md) for design details
- See [BUILD.md](BUILD.md) for advanced build options
