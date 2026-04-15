#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Category {
    Malware,
    Phishing,
    Tracking,
    Ads,
    Gambling,
    Adult,
    Safe,
    Unknown,
}
