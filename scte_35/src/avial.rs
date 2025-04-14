use std::fmt;

/// SCTE-35 Avail Descriptor Tag
pub const AVAIL_DESCRIPTOR_TAG: u8 = 0x00;
pub const CUE_IDENTIFIER: u32 = 0x4355_4549; // "CUEI"

/// Represents the `avail_descriptor` in SCTE-35.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvailDescriptor {
    pub provider_avail_id: u32,
}

impl AvailDescriptor {
    /// Total fixed length in bytes: 2 (tag + length) + 4 (identifier) + 4 (provider_avail_id)
    pub const DESCRIPTOR_LENGTH: usize = 10;

    /// Parses an `AvailDescriptor` from a 10-byte buffer.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, AvailDescriptorError> {
        if bytes.len() != Self::DESCRIPTOR_LENGTH {
            return Err(AvailDescriptorError::InvalidLength(bytes.len()));
        }

        if bytes[0] != AVAIL_DESCRIPTOR_TAG {
            return Err(AvailDescriptorError::InvalidTag(bytes[0]));
        }

        if bytes[1] != 0x08 {
            return Err(AvailDescriptorError::InvalidDescriptorLength(bytes[1]));
        }

        let identifier = u32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]);
        if identifier != CUE_IDENTIFIER {
            return Err(AvailDescriptorError::InvalidIdentifier(identifier));
        }

        let provider_avail_id = u32::from_be_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]);

        Ok(Self { provider_avail_id })
    }

    /// Serializes the `AvailDescriptor` into a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::DESCRIPTOR_LENGTH);
        buf.push(AVAIL_DESCRIPTOR_TAG);
        buf.push(0x08); // length: 4 bytes identifier + 4 bytes avail_id
        buf.extend(&CUE_IDENTIFIER.to_be_bytes());
        buf.extend(&self.provider_avail_id.to_be_bytes());
        buf
    }

    pub fn length(&self) -> usize {
        let length_bits = 32  // identifier
                        + 32; // provider_avail_id
        length_bits / 8
    }

    
}

/// Custom errors for AvailDescriptor parsing.
#[derive(Debug)]
pub enum AvailDescriptorError {
    InvalidLength(usize),
    InvalidTag(u8),
    InvalidDescriptorLength(u8),
    InvalidIdentifier(u32),
}

impl fmt::Display for AvailDescriptorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidLength(len) => write!(f, "Expected 10 bytes, got {}", len),
            Self::InvalidTag(tag) => write!(f, "Invalid tag: expected 0x00, got 0x{:02x}", tag),
            Self::InvalidDescriptorLength(dl) => {
                write!(f, "Invalid descriptor length: expected 0x08, got 0x{:02x}", dl)
            }
            Self::InvalidIdentifier(id) => write!(f, "Invalid identifier: expected 0x43554549 (CUEI), got 0x{:08x}", id),
        }
    }
}

impl std::error::Error for AvailDescriptorError {}
