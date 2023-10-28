use std::error::Error;
use std::time::Instant;

use crate::LICENSES;
use crate::LICENSE_EXCEPTIONS;

pub fn init_license_data() -> Result<(), Box<dyn Error>> {
    use reqwest::blocking;

    let start = Instant::now();
    let license_json = blocking::get("https://spdx.org/licenses/licenses.json")?.text()?;
    let exception_json = blocking::get("https://spdx.org/licenses/exceptions.json")?.text()?;
    LICENSES
        .set(serde_json::from_str(&license_json)?)
        .map_err(|_| "Unexpected error initializing licenses")?;
    LICENSE_EXCEPTIONS
        .set(serde_json::from_str(&exception_json)?)
        .map_err(|_| "Unexpected error initializing licenses")?;

    tracing::info!("License data loaded in {:?}.", start.elapsed());
    tracing::info!(
        "{} licenses loaded.",
        LICENSES
            .get()
            .ok_or("License data not available")?
            .licenses
            .len()
    );
    tracing::info!(
        "{} license exceptions loaded.",
        LICENSE_EXCEPTIONS
            .get()
            .ok_or("License data not available")?
            .exceptions
            .len()
    );

    Ok(())
}
