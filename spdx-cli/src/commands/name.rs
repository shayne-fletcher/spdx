use crate::LICENSES;
use spdx_lib::LicenseId;

pub fn name(license_id: &LicenseId) {
    for license in LICENSES
        .get()
        .unwrap()
        .licenses
        .iter()
        .filter(|s| &s.license_id == license_id)
    {
        println!("{}", license.name);
    }
}
