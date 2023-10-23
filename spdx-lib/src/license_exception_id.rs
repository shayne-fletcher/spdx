#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LicenseExceptionId(pub String);

impl std::fmt::Display for LicenseExceptionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.as_ref().fmt(f)
    }
}

impl AsRef<String> for LicenseExceptionId {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsMut<String> for LicenseExceptionId {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
