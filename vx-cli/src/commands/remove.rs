use crate::error::CliError;

pub fn execute(key: &str) -> Result<(), CliError> {
    println!("Removing secret: {}", key);
    // TODO: Implement actual removal logic
    Ok(())
}