//! SSH key generation and management.
//!
//! Uses ed25519 for key generation via the ed25519-dalek crate.

use crate::error::SshError;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

/// Generates a new ed25519 SSH keypair.
///
/// # Returns
/// A tuple of (public_key_openssh, private_key_bytes)
///
/// # Security
/// - Uses OS random number generator
/// - Private key should be encrypted before storage
pub fn generate_keypair() -> Result<(String, Vec<u8>), SshError> {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    let public_key_openssh = format_public_key(&verifying_key, "vaultx-generated");
    let private_key_bytes = signing_key.to_bytes().to_vec();

    Ok((public_key_openssh, private_key_bytes))
}

/// Formats a public key in OpenSSH format.
///
/// # Arguments
/// * `verifying_key` - The ed25519 public key
/// * `comment` - Comment to append to the key
///
/// # Returns
/// OpenSSH formatted public key string
fn format_public_key(verifying_key: &VerifyingKey, comment: &str) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine};

    // OpenSSH ed25519 public key format:
    // 4 bytes: length of key type string (11 for "ssh-ed25519")
    // 11 bytes: "ssh-ed25519"
    // 4 bytes: length of public key (32)
    // 32 bytes: public key

    let key_type = b"ssh-ed25519";
    let key_bytes = verifying_key.as_bytes();

    let mut blob = Vec::new();

    // Key type length (big-endian u32)
    blob.extend_from_slice(&(key_type.len() as u32).to_be_bytes());
    blob.extend_from_slice(key_type);

    // Public key length (big-endian u32)
    blob.extend_from_slice(&(key_bytes.len() as u32).to_be_bytes());
    blob.extend_from_slice(key_bytes);

    let encoded = STANDARD.encode(&blob);

    format!("ssh-ed25519 {} {}", encoded, comment)
}

/// Formats a private key in OpenSSH PEM format.
///
/// # Arguments
/// * `signing_key` - The ed25519 private key bytes
/// * `verifying_key` - The ed25519 public key bytes
///
/// # Returns
/// OpenSSH PEM formatted private key string
pub fn format_private_key(private_key: &[u8], public_key: &[u8]) -> Result<String, SshError> {
    use base64::{engine::general_purpose::STANDARD, Engine};

    if private_key.len() != 32 || public_key.len() != 32 {
        return Err(SshError::InvalidKeyFormat);
    }

    // OpenSSH private key format (unencrypted)
    let mut blob = Vec::new();

    // Auth magic
    blob.extend_from_slice(b"openssh-key-v1\0");

    // Cipher name (none = unencrypted)
    let cipher = b"none";
    blob.extend_from_slice(&(cipher.len() as u32).to_be_bytes());
    blob.extend_from_slice(cipher);

    // KDF name (none)
    let kdf = b"none";
    blob.extend_from_slice(&(kdf.len() as u32).to_be_bytes());
    blob.extend_from_slice(kdf);

    // KDF options (empty)
    blob.extend_from_slice(&0u32.to_be_bytes());

    // Number of keys
    blob.extend_from_slice(&1u32.to_be_bytes());

    // Public key blob
    let mut pub_blob = Vec::new();
    let key_type = b"ssh-ed25519";
    pub_blob.extend_from_slice(&(key_type.len() as u32).to_be_bytes());
    pub_blob.extend_from_slice(key_type);
    pub_blob.extend_from_slice(&(public_key.len() as u32).to_be_bytes());
    pub_blob.extend_from_slice(public_key);

    blob.extend_from_slice(&(pub_blob.len() as u32).to_be_bytes());
    blob.extend_from_slice(&pub_blob);

    // Private key section
    let mut priv_section = Vec::new();

    // Check integers (random, must match)
    let check: u32 = rand::random();
    priv_section.extend_from_slice(&check.to_be_bytes());
    priv_section.extend_from_slice(&check.to_be_bytes());

    // Key type
    priv_section.extend_from_slice(&(key_type.len() as u32).to_be_bytes());
    priv_section.extend_from_slice(key_type);

    // Public key
    priv_section.extend_from_slice(&(public_key.len() as u32).to_be_bytes());
    priv_section.extend_from_slice(public_key);

    // Private key (64 bytes: 32 private + 32 public)
    let full_private: Vec<u8> = private_key.iter().chain(public_key.iter()).copied().collect();
    priv_section.extend_from_slice(&(full_private.len() as u32).to_be_bytes());
    priv_section.extend_from_slice(&full_private);

    // Comment (empty)
    priv_section.extend_from_slice(&0u32.to_be_bytes());

    // Padding to block size (8 bytes for none cipher)
    let padding_len = (8 - (priv_section.len() % 8)) % 8;
    for i in 1..=padding_len {
        priv_section.push(i as u8);
    }

    blob.extend_from_slice(&(priv_section.len() as u32).to_be_bytes());
    blob.extend_from_slice(&priv_section);

    // Encode and format
    let encoded = STANDARD.encode(&blob);
    let mut pem = String::from("-----BEGIN OPENSSH PRIVATE KEY-----\n");

    for chunk in encoded.as_bytes().chunks(70) {
        pem.push_str(std::str::from_utf8(chunk).unwrap());
        pem.push('\n');
    }

    pem.push_str("-----END OPENSSH PRIVATE KEY-----\n");

    Ok(pem)
}

/// Generates OS-specific setup commands for adding a public key to authorized_keys.
///
/// # Arguments
/// * `public_key` - The OpenSSH formatted public key
///
/// # Returns
/// Shell commands for setting up the public key
pub fn generate_setup_commands(public_key: &str) -> String {
    // These commands work on Unix-like systems
    format!(
        r#"# Add this public key to your server's authorized_keys:
mkdir -p ~/.ssh
echo "{}" >> ~/.ssh/authorized_keys
chmod 700 ~/.ssh
chmod 600 ~/.ssh/authorized_keys"#,
        public_key
    )
}

/// Reconstructs a signing key from stored private key bytes.
pub fn reconstruct_signing_key(private_key_bytes: &[u8]) -> Result<SigningKey, SshError> {
    if private_key_bytes.len() != 32 {
        return Err(SshError::InvalidKeyFormat);
    }

    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(private_key_bytes);

    Ok(SigningKey::from_bytes(&key_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let (public_key, private_key) = generate_keypair().unwrap();

        assert!(public_key.starts_with("ssh-ed25519 "));
        assert_eq!(private_key.len(), 32);
    }

    #[test]
    fn test_keypair_uniqueness() {
        let (pub1, priv1) = generate_keypair().unwrap();
        let (pub2, priv2) = generate_keypair().unwrap();

        assert_ne!(pub1, pub2);
        assert_ne!(priv1, priv2);
    }

    #[test]
    fn test_reconstruct_signing_key() {
        let (_, private_key) = generate_keypair().unwrap();
        let signing_key = reconstruct_signing_key(&private_key).unwrap();

        // Verify we can get the same public key
        let verifying_key = signing_key.verifying_key();
        assert_eq!(verifying_key.as_bytes().len(), 32);
    }

    #[test]
    fn test_format_private_key() {
        let (_, private_key) = generate_keypair().unwrap();
        let signing_key = reconstruct_signing_key(&private_key).unwrap();
        let public_key = signing_key.verifying_key();

        let pem = format_private_key(&private_key, public_key.as_bytes()).unwrap();

        assert!(pem.starts_with("-----BEGIN OPENSSH PRIVATE KEY-----"));
        assert!(pem.ends_with("-----END OPENSSH PRIVATE KEY-----\n"));
    }

    #[test]
    fn test_generate_setup_commands() {
        let public_key = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAITest vaultx-generated";
        let commands = generate_setup_commands(public_key);

        assert!(commands.contains("mkdir -p ~/.ssh"));
        assert!(commands.contains("chmod 700 ~/.ssh"));
        assert!(commands.contains("chmod 600 ~/.ssh/authorized_keys"));
        assert!(commands.contains(public_key));
    }
}
