use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LicenseId(pub String);

impl std::fmt::Display for LicenseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.deref().fmt(f)
    }
}

impl Deref for LicenseId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LicenseId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<String> for LicenseId {
    fn as_ref(&self) -> &String {
        self.deref()
    }
}

impl AsMut<String> for LicenseId {
    fn as_mut(&mut self) -> &mut String {
        self.deref_mut()
    }
}
