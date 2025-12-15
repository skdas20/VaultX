# VaultX (vx)

A production-grade, CLI-first, zero-trust developer vault.

VaultX is a local-first security tool for developers and DevOps engineers that securely stores secrets and manages SSH identities. No GUI, no backend server, no cloud dependency.

## Features

- **Encrypted Project-Based Vault** - Secrets grouped by project, stored in a single encrypted file
- **WebAssembly Crypto Core** - All cryptographic logic in Rust, compiled to WASM
- **TTL-Based Secrets** - Automatic expiration for ephemeral secrets
- **Security Audit** - Identify expired, long-lived, and high-risk secrets
- **SSH Identity Manager** - Generate, store, and use ed25519 SSH keys securely

## Security

- **Argon2id** for key derivation (64MB memory, 3 iterations)
- **AES-256-GCM** for encryption with unique nonces
- **ed25519** for SSH key generation
- Secrets never passed via CLI arguments
- Temp SSH keys deleted after use

## Installation

### Quick Install with npm (Recommended)

```bash
npm install -g vaultx
vx --version  # Verify installation
```

That's it! The package automatically detects your OS and downloads the right binary.

**Requires:** Node.js 14+ | **Works on:** Linux, macOS, Windows

See [npm/README.md](npm/README.md) for detailed instructions.

### Download Pre-built Binaries

Download from [Releases](https://github.com/skdas20/VaultX/releases):
- **Windows**: vx-windows-x64.exe (~3 MB)
- **Linux**: vx-linux-x64 (~2-3 MB)
- **macOS Intel**: vx-macos-x64 (~2-3 MB)
- **macOS ARM**: vx-macos-arm64 (~2-3 MB)

### Build from Source

Requirements: Rust 1.70+, Cargo

```bash
git clone https://github.com/skdas20/VaultX.git
cd VaultX
./build.sh          # Auto-detects your OS
# or
cargo build --release
```

See [PLATFORM_BUILD.md](PLATFORM_BUILD.md) for detailed instructions by platform.

### Quick Start (Windows)

```powershell
cd vaultx
cargo build --release
# Binary at: target\release\vx.exe (~2-3 MB)
```

See [QUICKSTART.md](QUICKSTART.md) for detailed instructions.

## Usage

### Initialize a Project

```bash
vx init my-project
```

### Add Secrets

```bash
# Interactive prompt (recommended)
vx add my-project DB_PASSWORD

# From file
vx add my-project API_KEY --file ./secret.txt

# From environment variable
vx add my-project TOKEN --env MY_TOKEN

# With TTL (auto-expires)
vx add my-project TEMP_KEY --ttl 6h
```

### Retrieve Secrets

```bash
vx get my-project DB_PASSWORD
```

### Security Audit

```bash
vx audit
```

### SSH Identity Management

```bash
# Generate new SSH identity
vx ssh init my-server

# Connect using stored identity
vx ssh connect my-server user@hostname

# With additional SSH arguments
vx ssh connect my-server user@hostname -p 2222
```

## Architecture

```
User
└── vx (CLI)
    └── vx-core (Rust → WASM)
        ├── Crypto Engine (Argon2, AES-256-GCM)
        ├── Vault Logic
        ├── TTL / Expiry Logic
        └── SSH Key Management (ed25519)
```

## Vault File

Secrets are stored in `~/.vaultx/vault.vx`:

```
+----------------+------------------+----------------------+
| Header (16B)   | Salt (32B)       | Encrypted Payload    |
+----------------+------------------+----------------------+
| Magic: "VX01"  | Argon2 salt      | AES-256-GCM encrypted|
| Version: u32   |                  | JSON vault data      |
+----------------+------------------+----------------------+
```

## Design Philosophy

- Never reinvent system tools (uses system `ssh`)
- Never store secrets in plaintext
- Never accept secrets via CLI arguments
- Prefer composability over features
- Security > convenience
- Local-first, zero-trust architecture

## License

MIT
