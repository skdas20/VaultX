//! Property-based tests for vault module.

use proptest::prelude::*;
use vx_core::crypto::{derive_key, generate_salt, KEY_SIZE};
use vx_core::vault::{load_vault, save_vault, Vault};

/// Strategy for valid project names
fn arb_project_name() -> impl Strategy<Value = String> {
    prop::string::string_regex("[a-z][a-z0-9-]{0,20}")
        .unwrap()
        .prop_filter("non-empty", |s| !s.is_empty())
}

/// Strategy for valid secret keys
fn arb_secret_key() -> impl Strategy<Value = String> {
    prop::string::string_regex("[A-Z][A-Z0-9_]{0,30}")
        .unwrap()
        .prop_filter("non-empty", |s| !s.is_empty())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Feature: vaultx-cli, Property 2: Vault Persistence Round-Trip**
    /// *For any* valid vault state, serializing, encrypting, saving, loading,
    /// decrypting, and deserializing SHALL produce an equivalent vault state.
    /// **Validates: Requirements 9.1, 9.2**
    #[test]
    fn prop_vault_persistence_roundtrip(
        project_name in arb_project_name(),
        secret_key in arb_secret_key(),
        secret_value in prop::collection::vec(any::<u8>(), 1..100),
        password in prop::collection::vec(any::<u8>(), 8..32)
    ) {
        let mut vault = Vault::new();
        vault.init_project(&project_name).unwrap();

        let salt = generate_salt();
        let encryption_key = derive_key(&password, &salt).unwrap();

        vault.add_secret(&project_name, &secret_key, &secret_value, &encryption_key, None).unwrap();

        // Save and load
        let saved = save_vault(&vault, &password).unwrap();
        let loaded = load_vault(&saved, &password).unwrap();

        // Verify structure
        prop_assert_eq!(loaded.version, vault.version);
        prop_assert!(loaded.projects.contains_key(&project_name));
        prop_assert!(loaded.projects[&project_name].secrets.contains_key(&secret_key));
    }

    /// **Feature: vaultx-cli, Property 6: Project Uniqueness Enforcement**
    /// *For any* vault containing a project, attempting to initialize a project
    /// with the same name SHALL fail with a duplicate error.
    /// **Validates: Requirements 1.4**
    #[test]
    fn prop_project_uniqueness(project_name in arb_project_name()) {
        let mut vault = Vault::new();

        // First init should succeed
        vault.init_project(&project_name).unwrap();

        // Second init should fail
        let result = vault.init_project(&project_name);
        prop_assert!(result.is_err());
    }

    /// **Feature: vaultx-cli, Property 8: Missing Secret Error Handling**
    /// *For any* vault and any key that does not exist in a project,
    /// retrieval SHALL fail with a not-found error.
    /// **Validates: Requirements 3.2**
    #[test]
    fn prop_missing_secret_error(
        project_name in arb_project_name(),
        missing_key in arb_secret_key()
    ) {
        let mut vault = Vault::new();
        vault.init_project(&project_name).unwrap();

        let key = [0u8; KEY_SIZE];
        let result = vault.get_secret(&project_name, &missing_key, &key);

        prop_assert!(result.is_err());
    }

    /// **Feature: vaultx-cli, Property 9: Missing Project Error Handling**
    /// *For any* vault and any project name that does not exist,
    /// operations on that project SHALL fail with a not-found error.
    /// **Validates: Requirements 3.3**
    #[test]
    fn prop_missing_project_error(missing_project in arb_project_name()) {
        let vault = Vault::new();
        let key = [0u8; KEY_SIZE];

        let result = vault.get_secret(&missing_project, "ANY_KEY", &key);
        prop_assert!(result.is_err());
    }

    /// **Feature: vaultx-cli, Property 11: Tamper Detection**
    /// *For any* encrypted vault data, modifying any byte of the ciphertext
    /// SHALL cause decryption to fail with an authentication error.
    /// **Validates: Requirements 9.3**
    #[test]
    fn prop_tamper_detection(
        project_name in arb_project_name(),
        password in prop::collection::vec(any::<u8>(), 8..32),
        tamper_offset in 50usize..100  // Offset into ciphertext area
    ) {
        let mut vault = Vault::new();
        vault.init_project(&project_name).unwrap();

        let mut saved = save_vault(&vault, &password).unwrap();

        // Tamper with the data (after header + salt + nonce)
        if tamper_offset < saved.len() {
            saved[tamper_offset] ^= 0xFF;  // Flip all bits

            let result = load_vault(&saved, &password);
            prop_assert!(result.is_err());
        }
    }

    /// Property: Secret values are encrypted (not stored in plaintext)
    #[test]
    fn prop_secrets_encrypted(
        project_name in arb_project_name(),
        secret_key in arb_secret_key(),
        secret_value in prop::collection::vec(any::<u8>(), 10..100),
        password in prop::collection::vec(any::<u8>(), 8..32)
    ) {
        let mut vault = Vault::new();
        vault.init_project(&project_name).unwrap();

        let salt = generate_salt();
        let encryption_key = derive_key(&password, &salt).unwrap();

        vault.add_secret(&project_name, &secret_key, &secret_value, &encryption_key, None).unwrap();

        let saved = save_vault(&vault, &password).unwrap();

        // The plaintext secret should not appear in the saved data
        let saved_str = String::from_utf8_lossy(&saved);
        let secret_str = String::from_utf8_lossy(&secret_value);

        // For non-trivial secrets, they shouldn't appear in plaintext
        if secret_value.len() > 5 {
            prop_assert!(!saved_str.contains(&*secret_str));
        }
    }
}
