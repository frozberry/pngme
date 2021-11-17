use crc::crc32::checksum_ieee;
use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};

use crate::chunk_type::ChunkType;
use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Box<[u8]>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Result<Chunk> {
        let length_bytes: [u8; 4] = (data.len() as u32).to_be_bytes();
        let chunk_type_bytes = chunk_type.bytes();

        let crc_check = [&chunk_type_bytes, data.as_slice()].concat();
        let crc_bytes = checksum_ieee(crc_check.as_slice()).to_be_bytes();

        let bytes = [
            &length_bytes,
            &chunk_type_bytes,
            data.as_slice(),
            &crc_bytes,
        ]
        .concat();

        Chunk::try_from(bytes.as_slice())
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        // String::from_utf8(self.data().to_vec())
        match String::from_utf8(self.data().to_vec()) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into()),
        }
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        let length_bytes = self.length.to_be_bytes();
        let chunk_type_bytes = self.chunk_type.bytes();
        let data_bytes = self.data();
        let crc_bytes = self.crc.to_be_bytes();

        length_bytes
            .iter()
            .cloned()
            .chain(chunk_type_bytes.iter().cloned())
            .chain(data_bytes.iter().cloned())
            .chain(crc_bytes.iter().cloned())
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let mut reader = BufReader::new(bytes);

        let mut length_buffer: [u8; 4] = [0; 4];
        reader.read_exact(&mut length_buffer)?;
        let data_length = u32::from_be_bytes(length_buffer);

        let mut chunk_type_buffer: [u8; 4] = [0; 4];
        reader.read_exact(&mut chunk_type_buffer)?;
        let chunk_type = ChunkType::try_from(chunk_type_buffer)?;

        let mut data_buffer = vec![0; data_length as usize];
        reader.read_exact(&mut data_buffer)?;

        let mut crc_buffer: [u8; 4] = [0; 4];
        reader.read_exact(&mut crc_buffer)?;
        let crc = u32::from_be_bytes(crc_buffer);

        let check_bytes = [&chunk_type_buffer, data_buffer.as_slice()].concat();
        let calculated_crc = checksum_ieee(check_bytes.as_slice());

        match crc == calculated_crc {
            true => Ok(Chunk {
                length: data_length,
                chunk_type,
                chunk_data: data_buffer.into_boxed_slice(),
                crc,
            }),
            false => Err("CRC does not match".into()),
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
