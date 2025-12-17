//! Vault data structures and operations.
//!
//! The vault stores secrets organized by project, plus SSH identities.
//! All data is encrypted using AES-256-GCM before persistence.

use crate::crypto::{self, EncryptedData, KEY_SIZE, NONCE_SIZE, SALT_SIZE};
use crate::error::VaultError;
use crate::ttl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Magic bytes identifying a VaultX file
const VAULT_MAGIC: &[u8; 4] = b"VX01";

/// Current vault format version
const VAULT_VERSION: u32 = 1;

/// Header size in bytes (magic + version + reserved)
const HEADER_SIZE: usize = 16;

/// A secret stored in the vault.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub key: String,
    #[serde(with = "base64_serde")]
    pub encrypted_value: Vec<u8>,
    #[serde(with = "nonce_serde")]
    pub nonce: [u8; NONCE_SIZE],
    pub created_at: u64,
    pub expires_at: Option<u64>,
}

/// A project containing secrets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub secrets: HashMap<String, Secret>,
    pub created_at: u64,
}

/// An SSH identity stored in the vault.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshIdentity {
    pub name: String,
    pub public_key: String,
    #[serde(with = "base64_serde")]
    pub encrypted_private_key: Vec<u8>,
    #[serde(with = "nonce_serde")]
    pub nonce: [u8; NONCE_SIZE],
    pub created_at: u64,
}

/// An SSH server configuration stored in the vault.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshServerConfig {
    pub name: String,
    pub username: String,
    pub ip_address: String,
    pub identity_name: String,
    pub created_at: u64,
}

/// The main vault structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    pub version: u32,
    pub projects: HashMap<String, Project>,
    pub ssh_identities: HashMap<String, SshIdentity>,
    #[serde(default)]
    pub ssh_servers: HashMap<String, SshServerConfig>,
}

/// Internal vault data (JSON serialized before encryption)
#[derive(Debug, Serialize, Deserialize)]
struct VaultData {
    version: u32,
    projects: HashMap<String, Project>,
    ssh_identities: HashMap<String, SshIdentity>,
    #[serde(default)]
    ssh_servers: HashMap<String, SshServerConfig>,
}

impl Vault {
    /// Creates a new empty vault.
    pub fn new() -> Self {
        Self {
            version: VAULT_VERSION,
            projects: HashMap::new(),
            ssh_identities: HashMap::new(),
            ssh_servers: HashMap::new(),
        }
    }

    /// Initializes a new project in the vault.
    pub fn init_project(&mut self, name: &str) -> Result<(), VaultError> {
        if self.projects.contains_key(name) {
            return Err(VaultError::ProjectAlreadyExists(name.to_string()));
        }

        let project = Project {
            name: name.to_string(),
            secrets: HashMap::new(),
            created_at: ttl::current_timestamp(),
        };

        self.projects.insert(name.to_string(), project);
        Ok(())
    }

    /// Adds a secret to a project.
    ///
    /// # Arguments
    /// * `project` - Project name
    /// * `key` - Secret key
    /// * `value` - Secret value (plaintext)
    /// * `encryption_key` - Key for encrypting the secret value
    /// * `ttl_seconds` - Optional TTL in seconds
    pub fn add_secret(
        &mut self,
        project: &str,
        key: &str,
        value: &[u8],
        encryption_key: &[u8; KEY_SIZE],
        ttl_seconds: Option<u64>,
    ) -> Result<(), VaultError> {
        let proj = self
            .projects
            .get_mut(project)
            .ok_or_else(|| VaultError::ProjectNotFound(project.to_string()))?;

        let encrypted = crypto::encrypt(value, encryption_key)?;
        let now = ttl::current_timestamp();

        let secret = Secret {
            key: key.to_string(),
            encrypted_value: encrypted.ciphertext,
            nonce: encrypted.nonce,
            created_at: now,
            expires_at: ttl_seconds.and_then(|ttl| ttl::calculate_expiry(ttl, now)),
        };

        proj.secrets.insert(key.to_string(), secret);
        Ok(())
    }

    /// Retrieves and decrypts a secret from a project.
    pub fn get_secret(
        &self,
        project: &str,
        key: &str,
        encryption_key: &[u8; KEY_SIZE],
    ) -> Result<Vec<u8>, VaultError> {
        let proj = self
            .projects
            .get(project)
            .ok_or_else(|| VaultError::ProjectNotFound(project.to_string()))?;

        let secret = proj
            .secrets
            .get(key)
            .ok_or_else(|| VaultError::SecretNotFound(key.to_string()))?;

        // Check expiration
        let now = ttl::current_timestamp();
        if ttl::is_expired(secret.expires_at, now) {
            return Err(VaultError::SecretExpired(key.to_string()));
        }

        let encrypted = EncryptedData {
            ciphertext: secret.encrypted_value.clone(),
            nonce: secret.nonce,
        };

        crypto::decrypt(&encrypted, encryption_key).map_err(VaultError::CryptoError)
    }

    /// Adds an SSH identity to the vault.
    pub fn add_ssh_identity(
        &mut self,
        name: &str,
        public_key: String,
        private_key: &[u8],
        encryption_key: &[u8; KEY_SIZE],
    ) -> Result<(), VaultError> {
        if self.ssh_identities.contains_key(name) {
            return Err(VaultError::IdentityAlreadyExists(name.to_string()));
        }

        let encrypted = crypto::encrypt(private_key, encryption_key)?;

        let identity = SshIdentity {
            name: name.to_string(),
            public_key,
            encrypted_private_key: encrypted.ciphertext,
            nonce: encrypted.nonce,
            created_at: ttl::current_timestamp(),
        };

        self.ssh_identities.insert(name.to_string(), identity);
        Ok(())
    }

    /// Retrieves and decrypts an SSH identity's private key.
    pub fn get_ssh_identity(
        &self,
        name: &str,
        encryption_key: &[u8; KEY_SIZE],
    ) -> Result<(String, Vec<u8>), VaultError> {
        let identity = self
            .ssh_identities
            .get(name)
            .ok_or_else(|| VaultError::IdentityNotFound(name.to_string()))?;

        let encrypted = EncryptedData {
            ciphertext: identity.encrypted_private_key.clone(),
            nonce: identity.nonce,
        };

        let private_key = crypto::decrypt(&encrypted, encryption_key)?;

        Ok((identity.public_key.clone(), private_key))
    }

    /// Adds an SSH server configuration to the vault.
    pub fn add_ssh_server(
        &mut self,
        name: &str,
        username: String,
        ip_address: String,
        identity_name: String,
    ) -> Result<(), VaultError> {
        // Validate that the identity exists
        if !self.ssh_identities.contains_key(&identity_name) {
            return Err(VaultError::IdentityNotFound(identity_name));
        }

        let server = SshServerConfig {
            name: name.to_string(),
            username,
            ip_address,
            identity_name,
            created_at: ttl::current_timestamp(),
        };

        self.ssh_servers.insert(name.to_string(), server);
        Ok(())
    }

    /// Retrieves an SSH server configuration.
    pub fn get_ssh_server(&self, name: &str) -> Result<&SshServerConfig, VaultError> {
        self.ssh_servers
            .get(name)
            .ok_or_else(|| VaultError::ServerNotFound(name.to_string()))
    }

    /// Checks if an SSH server configuration exists.
    pub fn has_ssh_server(&self, name: &str) -> bool {
        self.ssh_servers.contains_key(name)
    }

    /// Removes a project and all its secrets.
    pub fn remove_project(&mut self, name: &str) -> Result<(), VaultError> {
        if self.projects.remove(name).is_some() {
            Ok(())
        } else {
            Err(VaultError::ProjectNotFound(name.to_string()))
        }
    }

    /// Removes a secret from a project.
    pub fn remove_secret(&mut self, project: &str, key: &str) -> Result<(), VaultError> {
        let proj = self
            .projects
            .get_mut(project)
            .ok_or_else(|| VaultError::ProjectNotFound(project.to_string()))?;

        if proj.secrets.remove(key).is_some() {
            Ok(())
        } else {
            Err(VaultError::SecretNotFound(key.to_string()))
        }
    }
}

impl Default for Vault {
    fn default() -> Self {
        Self::new()
    }
}

/// Serializes and encrypts a vault for storage.
///
/// # File Format
/// ```text
/// +----------------+------------------+----------------------+
/// | Header (16B)   | Salt (32B)       | Encrypted Payload    |
/// +----------------+------------------+----------------------+
/// | Magic: "VX01"  | Argon2 salt      | AES-256-GCM encrypted|
/// | Version: u32   |                  | JSON vault data      |
/// | Reserved: 8B   |                  | + Auth Tag (16B)     |
/// +----------------+------------------+----------------------+
/// ```
/// Saves a vault with optional salt preservation.
/// If salt is provided, it will be used (for updating existing vaults).
/// If salt is None, a new salt will be generated (for creating new vaults).
pub fn save_vault_with_salt(
    vault: &Vault,
    password: &[u8],
    salt: Option<&[u8; SALT_SIZE]>,
) -> Result<Vec<u8>, VaultError> {
    // Use provided salt or generate new one
    let salt = if let Some(s) = salt {
        s.clone()
    } else {
        crypto::generate_salt()
    };

    // Derive encryption key
    let key = crypto::derive_key(password, &salt)?;

    // Serialize vault to JSON
    let vault_data = VaultData {
        version: vault.version,
        projects: vault.projects.clone(),
        ssh_identities: vault.ssh_identities.clone(),
        ssh_servers: vault.ssh_servers.clone(),
    };

    let json = serde_json::to_vec(&vault_data)
        .map_err(|e| VaultError::SerializationError(e.to_string()))?;

    // Encrypt the JSON
    let encrypted = crypto::encrypt(&json, &key)?;

    // Build the file
    let mut output = Vec::with_capacity(HEADER_SIZE + SALT_SIZE + encrypted.ciphertext.len());

    // Header
    output.extend_from_slice(VAULT_MAGIC);
    output.extend_from_slice(&VAULT_VERSION.to_le_bytes());
    output.extend_from_slice(&[0u8; 8]); // Reserved

    // Salt
    output.extend_from_slice(&salt);

    // Nonce + Ciphertext
    output.extend_from_slice(&encrypted.nonce);
    output.extend_from_slice(&encrypted.ciphertext);

    Ok(output)
}

/// Convenience function: saves a new vault with generated salt.
pub fn save_vault(vault: &Vault, password: &[u8]) -> Result<Vec<u8>, VaultError> {
    save_vault_with_salt(vault, password, None)
}

/// Loads and decrypts a vault from storage.
pub fn load_vault(data: &[u8], password: &[u8]) -> Result<Vault, VaultError> {
    // Minimum size check
    let min_size = HEADER_SIZE + SALT_SIZE + NONCE_SIZE;
    if data.len() < min_size {
        return Err(VaultError::CorruptedVault);
    }

    // Verify magic
    if &data[0..4] != VAULT_MAGIC {
        return Err(VaultError::InvalidFormat("Invalid magic bytes".to_string()));
    }

    // Read version
    let version = u32::from_le_bytes(data[4..8].try_into().unwrap());
    if version != VAULT_VERSION {
        return Err(VaultError::InvalidFormat(format!(
            "Unsupported version: {}",
            version
        )));
    }

    // Extract salt
    let salt: [u8; SALT_SIZE] = data[HEADER_SIZE..HEADER_SIZE + SALT_SIZE]
        .try_into()
        .map_err(|_| VaultError::CorruptedVault)?;

    // Derive key
    let key = crypto::derive_key(password, &salt)?;

    // Extract nonce and ciphertext
    let nonce_start = HEADER_SIZE + SALT_SIZE;
    let nonce: [u8; NONCE_SIZE] = data[nonce_start..nonce_start + NONCE_SIZE]
        .try_into()
        .map_err(|_| VaultError::CorruptedVault)?;

    let ciphertext = data[nonce_start + NONCE_SIZE..].to_vec();

    let encrypted = EncryptedData { ciphertext, nonce };

    // Decrypt
    let json = crypto::decrypt(&encrypted, &key).map_err(|_| VaultError::AuthenticationFailed)?;

    // Deserialize
    let vault_data: VaultData =
        serde_json::from_slice(&json).map_err(|e| VaultError::SerializationError(e.to_string()))?;

    Ok(Vault {
        version: vault_data.version,
        projects: vault_data.projects,
        ssh_identities: vault_data.ssh_identities,
        ssh_servers: vault_data.ssh_servers,
    })
}

// Custom serde modules for binary data
mod base64_serde {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&STANDARD.encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        STANDARD.decode(&s).map_err(serde::de::Error::custom)
    }
}

mod nonce_serde {
    use super::NONCE_SIZE;
    use base64::{engine::general_purpose::STANDARD, Engine};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8; NONCE_SIZE], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&STANDARD.encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; NONCE_SIZE], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = STANDARD.decode(&s).map_err(serde::de::Error::custom)?;
        bytes
            .try_into()
            .map_err(|_| serde::de::Error::custom("Invalid nonce length"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vault() {
        let vault = Vault::new();
        assert_eq!(vault.version, VAULT_VERSION);
        assert!(vault.projects.is_empty());
        assert!(vault.ssh_identities.is_empty());
    }

    #[test]
    fn test_init_project() {
        let mut vault = Vault::new();
        vault.init_project("my-project").unwrap();

        assert!(vault.projects.contains_key("my-project"));
    }

    #[test]
    fn test_duplicate_project() {
        let mut vault = Vault::new();
        vault.init_project("my-project").unwrap();

        let result = vault.init_project("my-project");
        assert!(matches!(result, Err(VaultError::ProjectAlreadyExists(_))));
    }

    #[test]
    fn test_add_and_get_secret() {
        let mut vault = Vault::new();
        vault.init_project("test").unwrap();

        let key = [0u8; KEY_SIZE];
        let secret_value = b"my-secret-value";

        vault
            .add_secret("test", "DB_PASSWORD", secret_value, &key, None)
            .unwrap();

        let retrieved = vault.get_secret("test", "DB_PASSWORD", &key).unwrap();
        assert_eq!(retrieved, secret_value);
    }

    #[test]
    fn test_secret_not_found() {
        let vault = Vault::new();
        let key = [0u8; KEY_SIZE];

        let result = vault.get_secret("nonexistent", "KEY", &key);
        assert!(matches!(result, Err(VaultError::ProjectNotFound(_))));
    }

    #[test]
    fn test_vault_save_load_roundtrip() {
        let mut vault = Vault::new();
        vault.init_project("test").unwrap();

        let password = b"test-password";
        let key = crypto::derive_key(password, &crypto::generate_salt()).unwrap();

        vault
            .add_secret("test", "SECRET", b"value", &key, None)
            .unwrap();

        let saved = save_vault(&vault, password).unwrap();
        let loaded = load_vault(&saved, password).unwrap();

        assert_eq!(loaded.version, vault.version);
        assert!(loaded.projects.contains_key("test"));
    }

    #[test]
    fn test_wrong_password_fails() {
        let vault = Vault::new();
        let saved = save_vault(&vault, b"correct-password").unwrap();

        let result = load_vault(&saved, b"wrong-password");
        assert!(matches!(result, Err(VaultError::AuthenticationFailed)));
    }
}
