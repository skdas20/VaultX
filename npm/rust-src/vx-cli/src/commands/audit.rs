//! Audit the vault for security issues.

use crate::error::CliError;
use crate::input;
use crate::storage;
use vx_core::ttl::current_timestamp;
use vx_core::ttl::is_expired;

/// Number of days after which a secret is considered long-lived
const LONG_LIVED_DAYS: u64 = 90;

/// Seconds in a day
const SECONDS_PER_DAY: u64 = 86400;

/// High-risk patterns in secret names
const HIGH_RISK_PATTERNS: &[&str] = &[
    "password",
    "passwd",
    "secret",
    "token",
    "api_key",
    "apikey",
    "private_key",
    "privatekey",
    "credential",
];

/// Executes the audit command.
pub fn execute() -> Result<(), CliError> {
    // Load vault
    let password = input::read_password("Enter master password: ")?;
    let (vault, _key) = storage::load_vault_with_key(password.as_bytes())?;

    let now = current_timestamp();
    let long_lived_threshold = now.saturating_sub(LONG_LIVED_DAYS * SECONDS_PER_DAY);

    let mut total_secrets = 0;
    let mut expired_count = 0;
    let mut long_lived_count = 0;
    let mut high_risk_count = 0;

    let mut issues: Vec<String> = Vec::new();

    println!("\n=== VaultX Security Audit ===\n");

    for (project_name, project) in &vault.projects {
        let mut project_expired = 0;
        let mut project_long_lived = 0;
        let mut project_high_risk = 0;

        for (key, secret) in &project.secrets {
            total_secrets += 1;

            // Check if expired
            if is_expired(secret.expires_at, now) {
                expired_count += 1;
                project_expired += 1;
                issues.push(format!(
                    "  [EXPIRED] {}/{} - Secret has expired",
                    project_name, key
                ));
            }

            // Check if long-lived
            if secret.created_at < long_lived_threshold {
                long_lived_count += 1;
                project_long_lived += 1;
                let age_days = (now - secret.created_at) / SECONDS_PER_DAY;
                issues.push(format!(
                    "  [LONG-LIVED] {}/{} - {} days old (consider rotation)",
                    project_name, key, age_days
                ));
            }

            // Check for high-risk patterns
            let key_lower = key.to_lowercase();
            for pattern in HIGH_RISK_PATTERNS {
                if key_lower.contains(pattern) {
                    // Only flag if no TTL set
                    if secret.expires_at.is_none() {
                        high_risk_count += 1;
                        project_high_risk += 1;
                        issues.push(format!(
                            "  [HIGH-RISK] {}/{} - Sensitive secret without TTL",
                            project_name, key
                        ));
                        break;
                    }
                }
            }
        }

        // Project summary
        let project_total = project.secrets.len();
        let project_flagged = project_expired + project_long_lived + project_high_risk;

        println!(
            "Project '{}': {} secrets ({} expired, {} long-lived, {} high-risk)",
            project_name, project_total, project_expired, project_long_lived, project_high_risk
        );

        if project_flagged > 0 {
            for issue in issues
                .iter()
                .filter(|i| i.contains(&format!("{}/", project_name)))
            {
                println!("{}", issue);
            }
            println!();
        }
    }

    // SSH identities summary
    let ssh_count = vault.ssh_identities.len();
    if ssh_count > 0 {
        println!("SSH Identities: {}", ssh_count);
        for (name, identity) in &vault.ssh_identities {
            let age_days = (now - identity.created_at) / SECONDS_PER_DAY;
            if age_days > LONG_LIVED_DAYS {
                println!(
                    "  [LONG-LIVED] {} - {} days old (consider rotation)",
                    name, age_days
                );
            }
        }
        println!();
    }

    // Overall summary
    println!("=== Summary ===");
    println!("Total secrets: {}", total_secrets);
    println!("Expired: {}", expired_count);
    println!("Long-lived (>90 days): {}", long_lived_count);
    println!("High-risk without TTL: {}", high_risk_count);

    let total_issues = expired_count + long_lived_count + high_risk_count;
    if total_issues == 0 {
        println!("\n✓ No security issues found.");
    } else {
        println!("\n⚠ {} issue(s) found. Review and remediate.", total_issues);
    }

    Ok(())
}
