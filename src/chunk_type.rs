use crate::{Error, Result};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct ChunkType {
    bytes: [u8; 4],
}

#[allow(dead_code)]
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_critical(&self) -> bool {
        let byte = self.bytes[0];
        let fifth_bit = (byte >> 5) & 1;

        fifth_bit == 0
    }

    pub fn is_public(&self) -> bool {
        let byte = self.bytes[1];
        let fifth_bit = (byte >> 5) & 1;

        fifth_bit == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let byte = self.bytes[2];
        let fifth_bit = (byte >> 5) & 1;

        fifth_bit == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let byte = self.bytes[3];
        let fifth_bit = (byte >> 5) & 1;

        fifth_bit == 1
    }

    pub fn is_valid(&self) -> bool {
        let is_alpha = self
            .bytes
            .iter()
            .map(|b| ChunkType::is_valid_byte(*b))
            .all(|b| b);

        is_alpha && self.is_reserved_bit_valid()
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        // byte.is_ascii_uppercase() || byte.is_ascii_lowercase()
        byte.is_ascii_alphabetic()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        let c = ChunkType { bytes };
        let is_alpha = c
            .bytes
            .iter()
            .map(|b| ChunkType::is_valid_byte(*b))
            .all(|b| b);

        if is_alpha {
            Ok(c)
        } else {
            Err("Invalid ChunkType".into())
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.bytes))
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes_vec = s.as_bytes();
        let bytes = [bytes_vec[0], bytes_vec[1], bytes_vec[2], bytes_vec[3]];

        let c = ChunkType { bytes };
        let is_alpha = c
            .bytes
            .iter()
            .map(|b| ChunkType::is_valid_byte(*b))
            .all(|b| b);

        match is_alpha {
            true => Ok(c),
            false => Err("Invalid ChunkType".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
