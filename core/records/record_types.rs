#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordType {
    A,
    AAAA,
    TXT,
    CNAME,
}

impl RecordType {
    pub fn to_u8(&self) -> u8 {
        match self {
            RecordType::A => 1,
            RecordType::AAAA => 2,
            RecordType::TXT => 3,
            RecordType::CNAME => 4,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::A),
            2 => Some(Self::AAAA),
            3 => Some(Self::TXT),
            4 => Some(Self::CNAME),
            _ => None,
        }
    }
}
