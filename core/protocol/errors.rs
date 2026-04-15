use std::fmt;

#[derive(Debug)]
pub enum ProtocolError {
    InvalidVersion,
    InvalidPacket,
    SerializationError,
    DeserializationError,
    BufferTooSmall,
    UnknownMessageType,
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolError::InvalidVersion => write!(f, "Invalid protocol version"),
            ProtocolError::InvalidPacket => write!(f, "Invalid packet"),
            ProtocolError::SerializationError => write!(f, "Serialization failed"),
            ProtocolError::DeserializationError => write!(f, "Deserialization failed"),
            ProtocolError::BufferTooSmall => write!(f, "Buffer too small"),
            ProtocolError::UnknownMessageType => write!(f, "Unknown message type"),
        }
    }
}

impl std::error::Error for ProtocolError {}
