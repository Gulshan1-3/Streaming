use bitstream_io::{BitReader, BitWriter, BE};

     
   
     


use std::io::{Cursor, Result};

const DTMF_DESCRIPTOR_TAG: u8 = 0x01;
const CUE_IDENTIFIER: u32 = 0x43554549; // 'CUEI' in ASCII
const RESERVED: u8 = 0b11111;


#[derive(Debug, Clone, PartialEq, Eq)]

pub struct DTMFDescriptor {
    
    pub json_type: u32,
   
    pub preroll: u8,
    
    pub dtmf_chars: String,
}

impl Default for DTMFDescriptor {
     fn default() -> Self {
        Self {
            json_type: DTMF_DESCRIPTOR_TAG as u32,
            preroll: 0,
            dtmf_chars: String::new(),
        }
    }
}


impl DTMFDescriptor {
    pub fn decode(data: &[u8]) -> Result<Self> {
        let mut reader = BitReader::endian(Cursor::new(data), BE);
        let _tag = reader.read::<u8>(8)?;
        let _length = reader.read::<u8>(8)?; // descriptor_length
        let _identifier = reader.read::<u32>(32)?; // identifier

        let preroll = reader.read::<u8>(8)?;
        let dtmf_count = reader.read::<u8>(3)? as usize;
        reader.skip(5)?; // reserved

        let mut chars = String::new();
        for _ in 0..dtmf_count {
            let c = reader.read::<u8>(8)?;
            chars.push(c as char);
        }

        Ok(DTMFDescriptor {
            json_type: DTMF_DESCRIPTOR_TAG as u32,
            preroll,
            dtmf_chars: chars,
        })
    }
        pub fn encode(&self) -> Result<Vec<u8>> {
            let mut buf = Vec::new();
            {
                let mut writer = BitWriter::endian(&mut buf, BE);
    
                writer.write(8, DTMF_DESCRIPTOR_TAG)?; // splice_descriptor_tag
                writer.write(8, self.length() as u8)?; // descriptor_length
                writer.write(32, CUE_IDENTIFIER)?;     // identifier
    
                writer.write(8, self.preroll)?; // preroll
                writer.write(3, self.dtmf_chars.len() as u8)?; // dtmf_count
                writer.write(5, RESERVED)?; // reserved
    
                for c in self.dtmf_chars.bytes() {
                    writer.write(8, c)?;
                }
            }
    
            Ok(buf)
    
    
}

pub fn length(&self) -> usize {
    32 / 8 + 1 + 1 + (self.dtmf_chars.len()) // identifier + preroll + dtmf_count/reserved + chars
}

}
