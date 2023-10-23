mod commands;
mod init;

use clap::{Parser, Subcommand};
use std::error::Error;
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
// cargo run -p spdx-cli -- --data-dir ./data name --license-id "AGPL-3.0"
#[derive(Subcommand)]
enum Commands {
    Repl,
    Name {
        #[arg(short, long, value_parser = clap::value_parser!(LicenseId))]
        license_id: LicenseId,
    },
}

static LICENSES: OnceLock<LicenseList> = OnceLock::new();
static LICENSE_EXCEPTIONS: OnceLock<LicenseExceptionList> = OnceLock::new();

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    init::init_license_data(&args.data_dir)?;

    match &args.command {
        Some(Commands::Repl) => commands::repl(),
        Some(Commands::Name { license_id }) => commands::name(license_id),
        None => (),
    }

    Ok(())
}
