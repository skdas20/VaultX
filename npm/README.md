# NPM Installation Guide

VaultX can be installed globally via npm with automatic platform detection and binary download.

## Quick Install

```bash
npm install -g vaultx
```

That's it! The install script will:
1. ✅ Detect your OS (Linux, macOS, Windows)
2. ✅ Detect your CPU architecture (x64, arm64)
3. ✅ Download the correct pre-built binary
4. ✅ Make it globally available as `vx` command

## Usage

After installation, use it like any other CLI tool:

```bash
# Check version
vx --version

# Show help
vx --help

# Create a vault
vx init my-project

# Add a secret
vx add my-project API_KEY

# List projects
vx list

# Get a secret
vx get my-project API_KEY
```

## What Gets Installed

When you run `npm install -g vaultx`, you get:

### Binary Downloads (Automatic)
| Platform | Size | Type |
|----------|------|------|
| Linux x64 | 1.6 MB | Dynamic ELF |
| macOS Intel | ~2 MB | Mach-O |
| macOS ARM64 (M1/M2) | ~2 MB | Mach-O |
| Windows x64 | ~3 MB | PE/COFF |

### NPM Package Files
- `bin/vx.js` - Entry point wrapper script
- `scripts/install.js` - Platform detection & download script
- `scripts/uninstall.js` - Cleanup script
- `package.json` - Package metadata

## Platform Support

### Linux
- **Supported**: x86_64 (Intel/AMD processors)
- **Supported**: Ubuntu 18.04+, Debian 10+, Fedora, CentOS, Arch, etc.
- **Architecture**: amd64 (dynamic binary with glibc)

### macOS
- **Supported**: Intel Macs (x86_64)
- **Supported**: Apple Silicon Macs (arm64/M1/M2/M3)
- **Version**: macOS 10.13+

### Windows
- **Supported**: Windows 7 and later
- **Supported**: Windows 10, Windows 11
- **Architecture**: x86_64 (64-bit only)

## Prerequisites

### System Requirements
- **Node.js**: 14.0.0 or later
- **npm**: 6.0.0 or later
- **Internet connection**: For downloading the binary

### Platform-Specific

#### Linux
- glibc-based systems (standard for most distros)
- For musl-based systems (Alpine), build from source

#### macOS
- Xcode Command Line Tools (usually auto-installed)
- For M1/M2 Macs: Automatically detects and installs correct binary

#### Windows
- No additional requirements
- Works on Windows 7 and later

## Installation Process

### Step 1: Install from npm Registry
```bash
npm install -g vaultx
```

### Step 2: Automatic Download (During Installation)
The postinstall script runs automatically and:
- Detects your platform
- Downloads appropriate binary
- Makes it executable
- Verifies the download

### Step 3: Verify Installation
```bash
vx --version
# Output: vx 0.1.0
```

## Updating

To update to the latest version:

```bash
npm update -g vaultx
```

Or reinstall from scratch:
```bash
npm uninstall -g vaultx
npm install -g vaultx
```

## Uninstallation

```bash
npm uninstall -g vaultx
```

This removes the `vx` command and cleans up downloaded binaries.

## Troubleshooting

### "vx: command not found"

**Solution 1**: Check npm's global bin directory is in PATH
```bash
# Show where npm installs globals
npm config get prefix

# Add to PATH if needed (Linux/macOS)
export PATH=$(npm config get prefix)/bin:$PATH
```

**Solution 2**: Use full path
```bash
$(npm config get prefix)/bin/vx --version
```

### "Binary not found" Error

This might happen if:
1. **Installation was interrupted**
   - Solution: Reinstall with `npm install -g vaultx`

2. **Download failed due to network**
   - Solution: Check internet connection and retry
   - ```bash
     npm cache clean --force
     npm install -g vaultx
     ```

3. **Platform not supported**
   - Solution: Build from source
   - ```bash
     git clone https://github.com/skdas20/VaultX.git
     cd VaultX
     ./build.sh
     ```

### Installation Takes Too Long

The first installation might take 1-2 minutes because:
- Binary is being downloaded from GitHub (1-3 MB)
- npm is registering the global command

Subsequent installations are much faster (30 seconds).

### "HTTP 404" During Download

This means the binary release doesn't exist yet. Solutions:
1. **Wait for release**: GitHub might be processing the release
2. **Use latest tag**: The install script uses `latest` tag
3. **Build from source**: 
   ```bash
   git clone https://github.com/skdas20/VaultX.git
   cd VaultX
   npm run build
   ```

### Windows-Specific Issues

#### "Permission denied" Error
```powershell
# Run PowerShell as Administrator
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
npm install -g vaultx
```

#### Binary won't execute
```powershell
# Unblock file
Unblock-File -Path "$(npm config get prefix)\bin\vx.js"
```

## How It Works

### Architecture

```
npm install -g vaultx
    ↓
package.json (postinstall hook)
    ↓
scripts/install.js (runs automatically)
    ↓
├─ Detect OS & CPU architecture
├─ Download correct binary from GitHub Releases
├─ Make executable (Linux/macOS)
└─ Create symlink in npm bin directory
    ↓
bin/vx.js (entry point) ← when user runs 'vx'
    ↓
Find binary in node_modules/vaultx/binaries/
    ↓
Execute binary with arguments
```

### Download Sources

Binaries are downloaded from GitHub Releases:
```
https://github.com/skdas20/VaultX/releases/download/latest/vx-{platform}-{arch}
```

Examples:
- `vx-linux-x64` (Linux Intel)
- `vx-macos-x64` (macOS Intel)
- `vx-macos-arm64` (macOS Apple Silicon)
- `vx-windows-x64.exe` (Windows)

### Binary Locations

After installation, binaries are stored at:
- **Linux/macOS**: `~/.npm/_npx/vaultx/binaries/vx-*`
- **Windows**: `%APPDATA%\npm\node_modules\vaultx\binaries\vx-*`

## Security

### Binary Verification
- Binaries are downloaded directly from GitHub Releases
- GitHub provides HTTPS encryption
- npm uses integrity verification for package contents

### Permissions
- Binaries are made executable with `755` permissions
- Global installation requires no special privileges on Linux/macOS
- Windows handles permissions automatically

### No External Dependencies
- The vx binary is self-contained
- No runtime dependencies needed
- Works on fresh system without additional packages

## Advanced Usage

### Installing Specific Version

```bash
npm install -g vaultx@0.1.0
```

### Local Installation (Not Global)

```bash
npm install vaultx
npx vaultx --version
```

### Development Installation

If you want to contribute to VaultX:
```bash
git clone https://github.com/skdas20/VaultX.git
cd VaultX
npm link
```

## CI/CD Integration

### GitHub Actions
```yaml
- name: Install VaultX
  run: npm install -g vaultx
```

### Docker
```dockerfile
FROM node:16
RUN npm install -g vaultx
```

### GitLab CI
```yaml
before_script:
  - npm install -g vaultx
```

## Feedback & Support

- **Issues**: https://github.com/skdas20/VaultX/issues
- **Discussions**: https://github.com/skdas20/VaultX/discussions
- **Documentation**: https://github.com/skdas20/VaultX/blob/main/README.md

## FAQ

**Q: Does npm version auto-update?**
A: No, it uses a fixed release tag. Use `npm update -g vaultx` to update.

**Q: Can I use this offline?**
A: The first installation requires internet to download the binary. After that, it works offline.

**Q: Does this work with Node.js package managers other than npm?**
A: Yes! Also works with yarn, pnpm, bun:
```bash
yarn global add vaultx
pnpm add -g vaultx
bun add -g vaultx
```

**Q: What if my platform isn't supported?**
A: Build from source:
```bash
git clone https://github.com/skdas20/VaultX.git
cd VaultX
./build.sh
sudo cp dist/vx* /usr/local/bin/
```

**Q: Can I customize the installation?**
A: Yes, clone and modify package.json, then install locally:
```bash
npm install -g ./npm
```

