use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LicenseExceptionId(String);

impl Deref for LicenseExceptionId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LicenseExceptionId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<String> for LicenseExceptionId {
    fn as_ref(&self) -> &String {
        self.deref()
    }
}

impl AsMut<String> for LicenseExceptionId {
    fn as_mut(&mut self) -> &mut String {
        self.deref_mut()
    }
}
