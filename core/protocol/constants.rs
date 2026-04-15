pub const PROTOCOL_VERSION: u8 = 1;

// Header sizes
pub const HEADER_SIZE: usize = 8;

// Flags
pub const FLAG_RESPONSE: u16 = 0x01;
pub const FLAG_ERROR: u16 = 0x02;

// Max sizes
pub const MAX_PACKET_SIZE: usize = 4096;
