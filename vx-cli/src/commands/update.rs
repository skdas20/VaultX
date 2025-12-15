use self_update::cargo_crate_version;
use crate::error::CliError;

pub fn execute(yes: bool) -> Result<(), CliError> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("skdas20")
        .repo_name("VaultX")
        .bin_name("vx")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .no_confirm(yes)
        .build()
        .map_err(|e| CliError::UpdateError(e.to_string()))?
        .update()
        .map_err(|e| CliError::UpdateError(e.to_string()))?;

    println!("Update status: `{}`!", status.version());
    Ok(())
}