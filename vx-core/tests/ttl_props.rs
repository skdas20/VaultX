//! Property-based tests for TTL module.

use proptest::prelude::*;
use vx_core::ttl::{calculate_expiry, is_expired, parse_ttl};

/// Strategy for generating valid TTL strings
fn arb_ttl_string() -> impl Strategy<Value = (String, u64)> {
    (1u64..10000, prop_oneof!["m", "h", "d", "w"]).prop_map(|(n, unit)| {
        let multiplier = match unit.as_str() {
            "m" => 60,
            "h" => 3600,
            "d" => 86400,
            "w" => 604800,
            _ => unreachable!(),
        };
        (format!("{}{}", n, unit), n * multiplier)
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Feature: vaultx-cli, Property 3: TTL Parsing Correctness**
    /// *For any* valid TTL string in supported formats (minutes, hours, days, weeks),
    /// parsing SHALL produce the correct duration in seconds.
    /// **Validates: Requirements 4.3**
    #[test]
    fn prop_ttl_parsing_correctness((ttl_str, expected_seconds) in arb_ttl_string()) {
        let parsed = parse_ttl(&ttl_str).unwrap();
        prop_assert_eq!(parsed, expected_seconds);
    }

    /// **Feature: vaultx-cli, Property 4: TTL Expiration Enforcement**
    /// *For any* secret with an expiration timestamp in the past,
    /// retrieval SHALL fail with an expiration error.
    /// **Validates: Requirements 3.4, 4.2**
    #[test]
    fn prop_expired_secrets_detected(
        expires_at in 1u64..1000000,
        now in 1000001u64..2000000
    ) {
        // now > expires_at, so should be expired
        prop_assert!(is_expired(Some(expires_at), now));
    }

    /// **Feature: vaultx-cli, Property 5: Non-Expiring Secret Retrieval**
    /// *For any* secret created without a TTL, retrieval SHALL succeed
    /// regardless of the current time.
    /// **Validates: Requirements 4.4**
    #[test]
    fn prop_non_expiring_secrets_always_valid(now in any::<u64>()) {
        // None means no expiration
        prop_assert!(!is_expired(None, now));
    }

    /// Property: Not-yet-expired secrets are valid
    #[test]
    fn prop_not_yet_expired_valid(
        now in 1u64..1000000,
        future_offset in 1u64..1000000
    ) {
        let expires_at = now.saturating_add(future_offset);
        prop_assert!(!is_expired(Some(expires_at), now));
    }

    /// Property: Expiry calculation is correct
    #[test]
    fn prop_expiry_calculation(
        ttl_seconds in 1u64..1000000,
        now in 0u64..1000000
    ) {
        let expiry = calculate_expiry(ttl_seconds, now);
        prop_assert_eq!(expiry, Some(now + ttl_seconds));
    }

    /// Property: Invalid TTL formats are rejected
    #[test]
    fn prop_invalid_ttl_rejected(
        invalid in prop::string::string_regex("[a-z]{1,5}").unwrap()
    ) {
        // Strings without numbers should fail
        let result = parse_ttl(&invalid);
        prop_assert!(result.is_err());
    }
}
