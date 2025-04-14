use ascii::AsciiString;
use bitstream_io::{BigEndian, BitRecorder, BitWrite, BitWriter};
use std::{fmt, io};

use crate::{avial::AvailDescriptor, dtmf::DTMFDescriptor};


pub(crate) trait SpliceDescriptorExt {
    fn splice_descriptor_tag(&self) -> u8;

   
    fn validate_identifier_bytes(bytes: &[u8]) -> Result<(), &'static str> {
        const SCTE35_IDENTIFIER: u32 = 0x4355_4549;

        if bytes.len() < 4 {
            return Err("Buffer too short to contain SCTE-35 identifier");
        }

        let identifier = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if identifier != SCTE35_IDENTIFIER {
            return Err("Invalid SCTE-35 identifier");
        }

        Ok(())
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum SpliceDescriptor {
    Avail(AvailDescriptor),
    DTMF(DTMFDescriptor),
    Segmentation,
    Time,
    Audio,
    Unknown(u8, u32, Vec<u8>),
}

impl SpliceDescriptor {
    pub(crate) fn write_to<W>(&mut self, _buffer: &mut W) -> anyhow::Result<Vec<u8>>
    where
        W: io::Write,
    {
        match self {
            SpliceDescriptor::Avail(desc) => Ok(desc.to_bytes()),
            SpliceDescriptor::DTMF(desc) => Ok(desc.encode()?),
            SpliceDescriptor::Segmentation => unimplemented!(),
            SpliceDescriptor::Time => unimplemented!(),
            SpliceDescriptor::Audio => unimplemented!(),
            SpliceDescriptor::Unknown(_, _, _) => unimplemented!(),
        }
    }
}



#[derive(Copy, Clone, Debug, PartialEq)]
enum SpliceDescriptorTag {
    Avail,
    DTMF,
    Segmentation,
    Time,
    Audio,
    Reserved(u8),
    DVB(u8),
}

impl From<u8> for SpliceDescriptorTag {
    fn from(value: u8) -> Self {
        match value {
            0x0 => SpliceDescriptorTag::Avail,
            0x1 => SpliceDescriptorTag::DTMF,
            0x2 => SpliceDescriptorTag::Segmentation,
            0x3 => SpliceDescriptorTag::Time,
            0x4 => SpliceDescriptorTag::Audio,
            0x5..=0xEF => SpliceDescriptorTag::Reserved(value),
            _ => SpliceDescriptorTag::DVB(value),
        }
    }
}

impl From<SpliceDescriptorTag> for u8 {
    fn from(value: SpliceDescriptorTag) -> Self {
        match value {
            SpliceDescriptorTag::Avail => 0x0,
            SpliceDescriptorTag::DTMF => 0x1,
            SpliceDescriptorTag::Segmentation => 0x2,
            SpliceDescriptorTag::Time => 0x3,
            SpliceDescriptorTag::Audio => 0x4,
            SpliceDescriptorTag::Reserved(value) => value,
            SpliceDescriptorTag::DVB(value) => value,
        }
    }
}