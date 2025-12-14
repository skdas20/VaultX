//! TTL (Time-To-Live) parsing and expiration logic.
//!
//! Supports duration formats:
//! - `30m` - 30 minutes
//! - `6h` - 6 hours
//! - `7d` - 7 days
//! - `2w` - 2 weeks

use crate::error::TtlError;

/// Seconds per minute
const SECONDS_PER_MINUTE: u64 = 60;
/// Seconds per hour
const SECONDS_PER_HOUR: u64 = 3600;
/// Seconds per day
const SECONDS_PER_DAY: u64 = 86400;
/// Seconds per week
const SECONDS_PER_WEEK: u64 = 604800;

/// Parses a TTL string into seconds.
///
/// # Supported Formats
/// - `m` - minutes (e.g., "30m" = 1800 seconds)
/// - `h` - hours (e.g., "6h" = 21600 seconds)
/// - `d` - days (e.g., "7d" = 604800 seconds)
/// - `w` - weeks (e.g., "2w" = 1209600 seconds)
///
/// # Examples
/// ```
/// use vx_core::ttl::parse_ttl;
///
/// assert_eq!(parse_ttl("30m").unwrap(), 1800);
/// assert_eq!(parse_ttl("6h").unwrap(), 21600);
/// assert_eq!(parse_ttl("7d").unwrap(), 604800);
/// assert_eq!(parse_ttl("2w").unwrap(), 1209600);
/// ```
pub fn parse_ttl(input: &str) -> Result<u64, TtlError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(TtlError::InvalidFormat(input.to_string()));
    }

    // Split into numeric part and unit
    let (num_str, unit) = input.split_at(input.len() - 1);

    let value: u64 = num_str
        .parse()
        .map_err(|_| TtlError::InvalidFormat(input.to_string()))?;

    if value == 0 {
        return Err(TtlError::ZeroOrNegative);
    }

    let unit_char = unit.chars().next().ok_or(TtlError::InvalidFormat(input.to_string()))?;

    let multiplier = match unit_char {
        'm' => SECONDS_PER_MINUTE,
        'h' => SECONDS_PER_HOUR,
        'd' => SECONDS_PER_DAY,
        'w' => SECONDS_PER_WEEK,
        c => return Err(TtlError::InvalidUnit(c)),
    };

    value
        .checked_mul(multiplier)
        .ok_or(TtlError::Overflow)
}

/// Checks if a secret has expired.
///
/// # Arguments
/// * `expires_at` - Optional expiration timestamp (Unix seconds)
/// * `now` - Current timestamp (Unix seconds)
///
/// # Returns
/// `true` if the secret has expired, `false` otherwise.
/// Secrets without expiration (`None`) never expire.
pub fn is_expired(expires_at: Option<u64>, now: u64) -> bool {
    match expires_at {
        Some(expiry) => now >= expiry,
        None => false,
    }
}

/// Calculates the expiration timestamp.
///
/// # Arguments
/// * `ttl_seconds` - TTL duration in seconds
/// * `now` - Current timestamp (Unix seconds)
///
/// # Returns
/// Expiration timestamp (Unix seconds)
pub fn calculate_expiry(ttl_seconds: u64, now: u64) -> Option<u64> {
    now.checked_add(ttl_seconds)
}

/// Returns the current Unix timestamp in seconds.
pub fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System time before Unix epoch")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minutes() {
        assert_eq!(parse_ttl("30m").unwrap(), 1800);
        assert_eq!(parse_ttl("1m").unwrap(), 60);
        assert_eq!(parse_ttl("60m").unwrap(), 3600);
    }

    #[test]
    fn test_parse_hours() {
        assert_eq!(parse_ttl("1h").unwrap(), 3600);
        assert_eq!(parse_ttl("6h").unwrap(), 21600);
        assert_eq!(parse_ttl("24h").unwrap(), 86400);
    }

    #[test]
    fn test_parse_days() {
        assert_eq!(parse_ttl("1d").unwrap(), 86400);
        assert_eq!(parse_ttl("7d").unwrap(), 604800);
        assert_eq!(parse_ttl("30d").unwrap(), 2592000);
    }

    #[test]
    fn test_parse_weeks() {
        assert_eq!(parse_ttl("1w").unwrap(), 604800);
        assert_eq!(parse_ttl("2w").unwrap(), 1209600);
        assert_eq!(parse_ttl("4w").unwrap(), 2419200);
    }

    #[test]
    fn test_invalid_format() {
        assert!(parse_ttl("").is_err());
        assert!(parse_ttl("abc").is_err());
        assert!(parse_ttl("10").is_err());
        assert!(parse_ttl("h").is_err());
    }

    #[test]
    fn test_invalid_unit() {
        assert!(matches!(parse_ttl("10x"), Err(TtlError::InvalidUnit('x'))));
        assert!(matches!(parse_ttl("5s"), Err(TtlError::InvalidUnit('s'))));
    }

    #[test]
    fn test_zero_value() {
        assert!(matches!(parse_ttl("0h"), Err(TtlError::ZeroOrNegative)));
        assert!(matches!(parse_ttl("0d"), Err(TtlError::ZeroOrNegative)));
    }

    #[test]
    fn test_is_expired() {
        // Not expired
        assert!(!is_expired(Some(1000), 500));
        assert!(!is_expired(Some(1000), 999));

        // Expired
        assert!(is_expired(Some(1000), 1000));
        assert!(is_expired(Some(1000), 1001));

        // No expiration
        assert!(!is_expired(None, 0));
        assert!(!is_expired(None, u64::MAX));
    }

    #[test]
    fn test_calculate_expiry() {
        assert_eq!(calculate_expiry(3600, 1000), Some(4600));
        assert_eq!(calculate_expiry(86400, 0), Some(86400));
    }
}
