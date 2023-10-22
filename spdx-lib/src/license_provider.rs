use super::LicenseExceptionId;
use super::LicenseId;
use super::Store;

use anyhow::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct LicenseProvider {
    licenses: Arc<dyn Store<LicenseId, Arc<serde_json::Value>>>,
    license_exceptions: Arc<dyn Store<LicenseExceptionId, Arc<serde_json::Value>>>,
}

impl LicenseProvider {
    pub fn new(
        licenses: Arc<dyn Store<LicenseId, Arc<serde_json::Value>>>,
        license_exceptions: Arc<dyn Store<LicenseExceptionId, Arc<serde_json::Value>>>,
    ) -> Self {
        Self {
            licenses,
            license_exceptions,
        }
    }
}

impl super::LicenseProvider for LicenseProvider {
    fn get_license(&self, license_id: &LicenseId) -> Result<Option<Arc<serde_json::Value>>> {
        self.licenses.get(license_id)
    }

    fn get_license_exception(
        &self,
        license_exception_id: &LicenseExceptionId,
    ) -> Result<Option<Arc<serde_json::Value>>> {
        self.license_exceptions.get(license_exception_id)
    }
}
