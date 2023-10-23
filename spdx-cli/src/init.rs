use std::error::Error;
use std::fs;
use std::path::Path;

use crate::LICENSES;
use crate::LICENSE_EXCEPTIONS;

pub fn init_license_data(data_dir: &Path) -> Result<(), Box<dyn Error>> {
    let license_file = data_dir.join("licenses.json");
    let exceptions_file = data_dir.join("exceptions.json");
    let license_file_contents = fs::read_to_string(license_file)?;
    let exception_file_contents = fs::read_to_string(exceptions_file)?;
    LICENSES
        .set(serde_json::from_str(&license_file_contents)?)
        .map_err(|_| "Unexpected error initializing licenses")?;
    LICENSE_EXCEPTIONS
        .set(serde_json::from_str(&exception_file_contents)?)
        .map_err(|_| "Unexpected error initializing licenses")?;
    println!(
        "{} licenses loaded.",
        LICENSES
            .get()
            .ok_or("License data not available")?
            .licenses
            .len()
    );
    println!(
        "{} license exceptions loaded.",
        LICENSE_EXCEPTIONS
            .get()
            .ok_or("License data not available")?
            .exceptions
            .len()
    );

    Ok(())
}
