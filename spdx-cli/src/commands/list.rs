use jwalk::WalkDir;

use std::collections::hash_map;
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

fn licenses_of_<'a>(e: &'a spdx_lib::ast::CompoundExpr, ls: &mut Vec<&'a str>) {
    match e {
        spdx_lib::ast::CompoundExpr::SimpleExpr(e) => {
            if let spdx_lib::ast::SimpleExpr::LicenseId(l) = e {
                ls.push(&l.id.0);
            }
        }
        spdx_lib::ast::CompoundExpr::SimpleExprWithException(_) => {}
        spdx_lib::ast::CompoundExpr::OrExpr(e) => {
            licenses_of_(&e.lexpr, ls);
            licenses_of_(&e.rexpr, ls);
        }
        spdx_lib::ast::CompoundExpr::AndExpr(e) => {
            licenses_of_(&e.lexpr, ls);
            licenses_of_(&e.rexpr, ls);
        }
        spdx_lib::ast::CompoundExpr::ParenExpr(e) => {
            licenses_of_(&e.expr, ls);
        }
    }
}

fn licenses_of<'a>(e: &'a spdx_lib::ast::LicenseExpr, ls: &mut Vec<&'a str>) {
    match e {
        spdx_lib::ast::LicenseExpr::SimpleExpr(e) => match e {
            spdx_lib::ast::SimpleExpr::LicenseId(l) => ls.push(&l.id.0),
            spdx_lib::ast::SimpleExpr::LicenseRef(_) => (),
        },
        spdx_lib::ast::LicenseExpr::CompoundExpr(e) => {
            licenses_of_(e, ls);
        }
    }
}

fn read(
    p: &Path,
    ls: &mut std::collections::HashMap<String, Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let toml_contents = fs::read_to_string(p)?;
    let cargo_toml: spdx_lib::cargo_data::CargoToml = toml::from_str(&toml_contents)?;
    // tracing::info!("{}", p.display());
    let package_name = &cargo_toml.package.name;
    match &cargo_toml.package.license {
        Some(le) => match le.as_ref().parse::<spdx_lib::ast::LicenseExpr>() {
            Ok(l) => {
                let mut licenses = Vec::new();
                licenses_of(&l, &mut licenses);
                for l in licenses {
                    match ls.entry(l.to_owned()) {
                        hash_map::Entry::Vacant(e) => {
                            e.insert(vec![package_name.clone()]);
                        }
                        hash_map::Entry::Occupied(mut e) => {
                            e.get_mut().push(package_name.clone());
                        }
                    }
                }
            }
            Err(_) => {
                panic!("Couldn't parse \"{package_name}\"'s license!");
            }
        },
        None => {}
    }
    // match &cargo_toml.package.license_file {
    //     Some(license_file) => {
    //         println!("  {}", license_file);
    //     }
    //     None => {}
    // }
    Ok(())
}

pub fn list(path: impl AsRef<Path>) {
    let mut ls = std::collections::HashMap::new();
    for p in find_cargo_tomls(path) {
        let _ = read(&p, &mut ls);
    }
    println!("{:#?}", ls)
}
