use jwalk::WalkDir;

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn is_cargo_toml(path: &Path) -> bool {
    if let Some(name) = path.file_name() {
        name == Path::new("Cargo.toml")
    } else {
        false
    }
}

fn find_cargo_tomls(path: impl AsRef<Path>) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| is_cargo_toml(path))
}

fn read(p: &Path) -> Result<(), Box<dyn Error>> {
    let toml_contents = fs::read_to_string(p)?;
    let cargo_toml: spdx_lib::cargo_data::CargoToml = toml::from_str(&toml_contents)?;
    println!("{}", p.display());
    match &cargo_toml.package.license {
        Some(license) => {
            println!("  {}", license);
        }
        None => {}
    }
    match &cargo_toml.package.license_file {
        Some(license_file) => {
            println!("  {}", license_file);
        }
        None => {}
    }
    Ok(())
}

pub fn list(path: impl AsRef<Path>) {
    for p in find_cargo_tomls(path) {
        let _ = read(&p);
    }
}
