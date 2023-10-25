mod commands;
mod init;

use spdx_lib::{LicenseExceptionList, LicenseId, LicenseList};

use clap::{Parser, Subcommand};

use std::error::Error;
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    num_threads: Option<usize>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // cargo run -p spdx-cli -- repl
    Repl,
    // cargo run -p spdx-cli -- name --license-id "AGPL-3.0"
    Name {
        #[arg(short, long, value_parser = clap::value_parser!(LicenseId))]
        license_id: LicenseId,
    },
    // cargo run -p spdx-cli -- list --root "$(pwd)"
    List {
        #[arg(short, long)]
        root: PathBuf,
    },
}

static LICENSES: OnceLock<LicenseList> = OnceLock::new();
static LICENSE_EXCEPTIONS: OnceLock<LicenseExceptionList> = OnceLock::new();

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if let Some(num_threads) = args.num_threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .unwrap();
    }

    init::init_license_data()?;

    match &args.command {
        Some(Commands::Repl) => commands::repl(),
        Some(Commands::Name { license_id }) => commands::name(license_id),
        Some(Commands::List { root }) => commands::list(root),
        None => (),
    }

    Ok(())
}
