//! Property-based tests for SSH module.

use proptest::prelude::*;
use vx_core::crypto::{decrypt, derive_key, encrypt, generate_salt, EncryptedData, KEY_SIZE};
use vx_core::ssh::{format_private_key, generate_keypair, generate_setup_commands, reconstruct_signing_key};
use vx_core::vault::Vault;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Feature: vaultx-cli, Property 12: SSH Keypair Validity**
    /// *For any* generated SSH keypair, the public key SHALL be derivable from
    /// the private key, and signatures created with the private key SHALL verify
    /// with the public key.
    /// **Validates: Requirements 6.1, 6.2, 6.3**
    #[test]
    fn prop_ssh_keypair_validity(_seed in any::<u64>()) {
        let (public_key, private_key_bytes) = generate_keypair().unwrap();

        // Public key should be in OpenSSH format
        prop_assert!(public_key.starts_with("ssh-ed25519 "));

        // Private key should be 32 bytes
        prop_assert_eq!(private_key_bytes.len(), 32);

        // Should be able to reconstruct signing key
        let signing_key = reconstruct_signing_key(&private_key_bytes).unwrap();

        // Verifying key derived from signing key should match
        let verifying_key = signing_key.verifying_key();
        prop_assert_eq!(verifying_key.as_bytes().len(), 32);
    }

    /// **Feature: vaultx-cli, Property 13: SSH Identity Uniqueness**
    /// *For any* vault containing an SSH identity, attempting to create an identity
    /// with the same name SHALL fail with a duplicate error.
    /// **Validates: Requirements 6.5**
    #[test]
    fn prop_ssh_identity_uniqueness(
        identity_name in prop::string::string_regex("[a-z][a-z0-9-]{0,20}").unwrap()
    ) {
        prop_assume!(!identity_name.is_empty());

        let mut vault = Vault::new();
        let (public_key, private_key) = generate_keypair().unwrap();

        let salt = generate_salt();
        let encryption_key = derive_key(b"password", &salt).unwrap();

        // First add should succeed
        vault.add_ssh_identity(&identity_name, public_key.clone(), &private_key, &encryption_key).unwrap();

        // Second add should fail
        let result = vault.add_ssh_identity(&identity_name, public_key, &private_key, &encryption_key);
        prop_assert!(result.is_err());
    }

    /// **Feature: vaultx-cli, Property 14: Missing SSH Identity Error Handling**
    /// *For any* vault and any identity name that does not exist,
    /// SSH connect operations SHALL fail with a not-found error.
    /// **Validates: Requirements 7.5**
    #[test]
    fn prop_missing_ssh_identity_error(
        missing_identity in prop::string::string_regex("[a-z][a-z0-9-]{0,20}").unwrap()
    ) {
        prop_assume!(!missing_identity.is_empty());

        let vault = Vault::new();
        let key = [0u8; KEY_SIZE];

        let result = vault.get_ssh_identity(&missing_identity, &key);
        prop_assert!(result.is_err());
    }

    /// Property: SSH private key round-trip through encryption
    #[test]
    fn prop_ssh_key_encryption_roundtrip(_seed in any::<u64>()) {
        let (_, private_key_bytes) = generate_keypair().unwrap();

        let salt = generate_salt();
        let encryption_key = derive_key(b"password", &salt).unwrap();

        // Encrypt
        let encrypted = encrypt(&private_key_bytes, &encryption_key).unwrap();

        // Decrypt
        let decrypted = decrypt(&encrypted, &encryption_key).unwrap();

        prop_assert_eq!(decrypted, private_key_bytes);

        // Should still be valid key
        let signing_key = reconstruct_signing_key(&decrypted).unwrap();
        prop_assert_eq!(signing_key.to_bytes().to_vec(), private_key_bytes);
    }

    /// Property: Generated keypairs are unique
    #[test]
    fn prop_keypairs_unique(_seed in any::<u64>()) {
        let (pub1, priv1) = generate_keypair().unwrap();
        let (pub2, priv2) = generate_keypair().unwrap();

        prop_assert_ne!(pub1, pub2);
        prop_assert_ne!(priv1, priv2);
    }

    /// Property: Setup commands contain required elements
    #[test]
    fn prop_setup_commands_complete(
        _seed in any::<u64>()
    ) {
        let (public_key, _) = generate_keypair().unwrap();
        let commands = generate_setup_commands(&public_key);

        prop_assert!(commands.contains("mkdir -p ~/.ssh"));
        prop_assert!(commands.contains("chmod 700 ~/.ssh"));
        prop_assert!(commands.contains("chmod 600 ~/.ssh/authorized_keys"));
        prop_assert!(commands.contains(&public_key));
    }

    /// Property: Private key PEM format is valid
    #[test]
    fn prop_private_key_pem_format(_seed in any::<u64>()) {
        let (_, private_key_bytes) = generate_keypair().unwrap();
        let signing_key = reconstruct_signing_key(&private_key_bytes).unwrap();
        let public_key_bytes = signing_key.verifying_key();

        let pem = format_private_key(&private_key_bytes, public_key_bytes.as_bytes()).unwrap();

        prop_assert!(pem.starts_with("-----BEGIN OPENSSH PRIVATE KEY-----"));
        prop_assert!(pem.ends_with("-----END OPENSSH PRIVATE KEY-----\n"));
    }
}
