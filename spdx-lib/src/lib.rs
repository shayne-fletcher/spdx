pub mod license_data;
pub mod license_exception_id;
pub mod license_expression_lexer;
pub mod license_expression_parser;
pub mod license_id;
pub mod license_provider;
pub mod store;

pub use license_exception_id::LicenseExceptionId;
pub use license_expression_parser::ast;
pub use license_id::LicenseId;

use anyhow::Result;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Store<K, V>: Debug + Send + Sync
where
    K: Send + Sync + Eq + Hash,
    V: Clone + Send + Sync,
{
    fn get(&self, key: &K) -> Result<Option<V>>;
    fn insert(&self, key: K, val: V) -> Result<()>;
    fn contains_key(&self, key: &K) -> Result<bool> {
        Ok(self.get(key)?.is_some())
    }
}

trait LicenseProvider {
    fn get_license(
        &self,
        license_id: &LicenseId,
    ) -> Result<Option<std::sync::Arc<serde_json::Value>>>;

    fn get_license_exception(
        &self,
        license_exception_id: &LicenseExceptionId,
    ) -> Result<Option<std::sync::Arc<serde_json::Value>>>;
}
