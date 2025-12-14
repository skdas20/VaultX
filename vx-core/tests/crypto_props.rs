//! Property-based tests for crypto module.
//!
//! These tests verify correctness properties using proptest.

use proptest::prelude::*;
use vx_core::crypto::{decrypt, derive_key, encrypt, generate_salt, EncryptedData, KEY_SIZE};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Feature: vaultx-cli, Property 1: Secret Round-Trip Consistency**
    /// *For any* valid secret value and encryption key, encrypting then decrypting
    /// the secret SHALL produce the original value.
    /// **Validates: Requirements 2.4, 3.1**
    #[test]
    fn prop_secret_roundtrip(secret in prop::collection::vec(any::<u8>(), 0..1024)) {
        let salt = generate_salt();
        let password = b"test-password";
        let key = derive_key(password, &salt).unwrap();

        let encrypted = encrypt(&secret, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();

        prop_assert_eq!(decrypted, secret);
    }

    /// **Feature: vaultx-cli, Property 10: Nonce Uniqueness**
    /// *For any* two encryption operations, the generated nonces SHALL be different.
    /// **Validates: Requirements 8.1**
    #[test]
    fn prop_nonce_uniqueness(
        secret1 in prop::collection::vec(any::<u8>(), 1..100),
        secret2 in prop::collection::vec(any::<u8>(), 1..100)
    ) {
        let key = [0u8; KEY_SIZE];

        let encrypted1 = encrypt(&secret1, &key).unwrap();
        let encrypted2 = encrypt(&secret2, &key).unwrap();

        // Nonces should be different even for same input
        prop_assert_ne!(encrypted1.nonce, encrypted2.nonce);
    }

    /// Property: Different passwords produce different keys
    #[test]
    fn prop_different_passwords_different_keys(
        password1 in prop::collection::vec(any::<u8>(), 1..32),
        password2 in prop::collection::vec(any::<u8>(), 1..32)
    ) {
        prop_assume!(password1 != password2);

        let salt = generate_salt();
        let key1 = derive_key(&password1, &salt).unwrap();
        let key2 = derive_key(&password2, &salt).unwrap();

        prop_assert_ne!(key1, key2);
    }

    /// Property: Same password and salt always produce same key (deterministic)
    #[test]
    fn prop_key_derivation_deterministic(
        password in prop::collection::vec(any::<u8>(), 1..32)
    ) {
        let salt = generate_salt();

        let key1 = derive_key(&password, &salt).unwrap();
        let key2 = derive_key(&password, &salt).unwrap();

        prop_assert_eq!(key1, key2);
    }

    /// Property: Wrong key fails decryption
    #[test]
    fn prop_wrong_key_fails(
        secret in prop::collection::vec(any::<u8>(), 1..100)
    ) {
        let salt = generate_salt();
        let key1 = derive_key(b"password1", &salt).unwrap();
        let key2 = derive_key(b"password2", &salt).unwrap();

        let encrypted = encrypt(&secret, &key1).unwrap();
        let result = decrypt(&encrypted, &key2);

        prop_assert!(result.is_err());
    }
}
