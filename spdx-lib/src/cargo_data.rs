use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CargoToml {
    pub package: Package,
    // pub dependencies: Option<toml::Value>,
    // pub bin: Option<Vec<toml::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<String>>,
    pub edition: Option<String>,
    pub rust_version: Option<String>,
    pub license: Option<crate::LicenseId>,
    pub license_file: Option<String>,
}
