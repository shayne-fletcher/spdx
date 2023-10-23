mod commands;

use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use spdx_lib::{LicenseExceptionList, LicenseId, LicenseList};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    data_dir: PathBuf,
    #[command(subcommand)]
    command: Option<Commands>,
}

// cargo run -p spdx-cli -- --data-dir ./data repl
// cargo run -p spdx-cli -- --data-dir ./data name --license "AGPL-3.0"
#[derive(Subcommand)]
enum Commands {
    Repl,
    Name {
        #[arg(short, long, value_parser = clap::value_parser!(LicenseId))]
        license: LicenseId,
    },
}

static LICENSES: OnceLock<LicenseList> = OnceLock::new();
static LICENSE_EXCEPTIONS: OnceLock<LicenseExceptionList> = OnceLock::new();

fn init_license_data(args: &Args) -> Result<(), Box<dyn Error>> {
    let data_dir = &args.data_dir;
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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    init_license_data(&args)?;

    match &args.command {
        Some(Commands::Repl) => commands::repl(),
        Some(Commands::Name { license }) => commands::name(license),
        None => (),
    }

    Ok(())
}
