# VaultX Architecture

## Overview

VaultX follows a two-layer architecture separating concerns between user interaction and cryptographic operations.

```
┌─────────────────────────────────────────────────────────────┐
│                        User Layer                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                    vx-cli                            │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐            │   │
│  │  │  init    │ │   add    │ │   get    │            │   │
│  │  └──────────┘ └──────────┘ └──────────┘            │   │
│  │  ┌──────────┐ ┌──────────┐                         │   │
│  │  │  audit   │ │   ssh    │                         │   │
│  │  └──────────┘ └──────────┘                         │   │
│  │                                                     │   │
│  │  ┌─────────────────────────────────────────────┐   │   │
│  │  │  Secure Input │ Storage │ Temp File Manager │   │   │
│  │  └─────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────┘   │
│                            │                                │
│                            ▼                                │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                    vx-core (WASM)                    │   │
│  │  ┌──────────────┐ ┌──────────────┐                  │   │
│  │  │ Crypto Engine│ │ Vault Logic  │                  │   │
│  │  │  - Argon2id  │ │  - Projects  │                  │   │
│  │  │  - AES-GCM   │ │  - Secrets   │                  │   │
│  │  └──────────────┘ └──────────────┘                  │   │
│  │  ┌──────────────┐ ┌──────────────┐                  │   │
│  │  │ TTL Manager  │ │ SSH Manager  │                  │   │
│  │  │  - Parsing   │ │  - ed25519   │                  │   │
│  │  │  - Expiry    │ │  - OpenSSH   │                  │   │
│  │  └──────────────┘ └──────────────┘                  │   │
│  └─────────────────────────────────────────────────────┘   │
│                            │                                │
└────────────────────────────┼────────────────────────────────┘
                             │
                             ▼
                    ┌────────────────┐
                    │  ~/.vaultx/    │
                    │   vault.vx     │
                    └────────────────┘
```

## Components

### vx-cli

The command-line interface handles:
- User interaction and argument parsing (clap)
- Secure password/secret input (rpassword)
- File system operations
- SSH process execution
- Temporary file management

**Key Principle**: No cryptographic operations in CLI layer.

### vx-core

The cryptographic core handles:
- Key derivation (Argon2id)
- Encryption/decryption (AES-256-GCM)
- SSH key generation (ed25519)
- Vault serialization
- TTL management

**Key Principle**: All security-critical code isolated here.

## Data Flow

### Adding a Secret

```
1. User: vx add project KEY
2. CLI: Prompt for password (no echo)
3. CLI: Prompt for secret value (no echo)
4. CLI: Load vault file
5. Core: Derive key from password + salt
6. Core: Decrypt vault
7. Core: Encrypt secret value
8. Core: Add to vault structure
9. Core: Re-encrypt vault
10. CLI: Atomic write to disk
```

### SSH Connect

```
1. User: vx ssh connect identity user@host
2. CLI: Prompt for password
3. CLI: Load vault
4. Core: Decrypt SSH private key
5. CLI: Write key to temp file (0600)
6. CLI: Execute: ssh -i tempfile user@host
7. CLI: Wait for SSH to exit
8. CLI: Delete temp file
```

## Security Model

### Threat Model

- **Protected against**: Disk theft, memory dumps, shoulder surfing
- **Not protected against**: Compromised system, keyloggers, root access

### Key Security Decisions

1. **Argon2id Parameters**
   - Memory: 64 MB (resists GPU attacks)
   - Iterations: 3
   - Parallelism: 4

2. **AES-256-GCM**
   - 96-bit random nonce per encryption
   - Authenticated encryption prevents tampering

3. **No Secrets in CLI Args**
   - Prevents exposure in shell history
   - Prevents exposure in process listings

4. **Temp Key Cleanup**
   - SSH keys written to temp files
   - Deleted immediately after use
   - Permissions set to 0600

## File Format

### Vault File (vault.vx)

```
Offset  Size    Field
------  ----    -----
0       4       Magic ("VX01")
4       4       Version (u32 LE)
8       8       Reserved
16      32      Salt (Argon2)
48      12      Nonce (AES-GCM)
60      var     Ciphertext + Auth Tag
```

### Decrypted Structure (JSON)

```json
{
  "version": 1,
  "projects": {
    "project-name": {
      "name": "project-name",
      "secrets": {
        "KEY": {
          "key": "KEY",
          "encrypted_value": "base64...",
          "nonce": "base64...",
          "created_at": 1702500000,
          "expires_at": null
        }
      },
      "created_at": 1702400000
    }
  },
  "ssh_identities": {
    "identity-name": {
      "name": "identity-name",
      "public_key": "ssh-ed25519 AAAA...",
      "encrypted_private_key": "base64...",
      "nonce": "base64...",
      "created_at": 1702400000
    }
  }
}
```

## Error Handling

### Security-Sensitive Errors

- Authentication failures are generic (prevents oracle attacks)
- Secret values never appear in error messages
- Constant-time comparison for auth tags

### Error Categories

| Error | User Message |
|-------|--------------|
| Wrong password | "Invalid password or corrupted vault" |
| Tampered vault | "Invalid password or corrupted vault" |
| Missing project | "Project 'X' not found" |
| Missing secret | "Secret 'X' not found" |
| Expired secret | "Secret 'X' has expired" |

## Testing Strategy

### Unit Tests
- Inline in each module
- Test specific functions

### Property-Based Tests
- Using `proptest` crate
- 100 iterations per property
- Test invariants across all inputs

### Key Properties Tested
1. Encryption round-trip
2. Vault persistence round-trip
3. TTL parsing correctness
4. Tamper detection
5. SSH keypair validity
6. Audit accuracy
