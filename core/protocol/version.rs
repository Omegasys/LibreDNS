#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version(pub u8);

impl Version {
    pub fn current() -> Self {
        Version(crate::core::protocol::constants::PROTOCOL_VERSION)
    }

    pub fn is_compatible(&self) -> bool {
        self.0 == crate::core::protocol::constants::PROTOCOL_VERSION
    }
}
