#![allow(dead_code)]
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Default)]
pub struct Cursor<'a> {
    pub cursor: std::io::Cursor<&'a [u8]>,
}

impl<'a> Cursor<'a> {
    pub fn new(cursor: std::io::Cursor<&'a [u8]>) -> Self {
        Self { cursor }
    }
}

impl<'a> From<&'a [u8]> for Cursor<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        let cursor = std::io::Cursor::new(bytes);
        Self { cursor }
    }
}

impl<'a> From<std::io::Cursor<&'a [u8]>> for Cursor<'a> {
    fn from(cursor: std::io::Cursor<&'a [u8]>) -> Self {
        Self { cursor }
    }
}

impl<'a> Cursor<'a> {
    pub fn seek(&mut self, style: SeekFrom) -> Result<u64, crate::error::CursorError> {
        match self.cursor.seek(style) {
            Ok(result) => Ok(result),
            Err(error) => Err(crate::error::CursorError::SeekError(error)),
        }
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), crate::error::CursorError> {
        match self.cursor.read_exact(buf) {
            Ok(result) => Ok(result),
            Err(error) => Err(crate::error::CursorError::ReadExactError(error)),
        }
    }
}

macro_rules! read {
    ($type: ident, $read_type: ident) => {
        paste::item! {
            impl<'a> Cursor<'a> {
                pub fn [<read_$type>](&mut self) -> Result<$type, crate::error::CursorError> {
                    let mut buffer = [0u8; std::mem::size_of::<$type>()];

                    if let Err(error) = self.cursor.read_exact(&mut buffer) {
                        return Err(crate::error::ReadError::$read_type(error).into());
                    }

                    let result = $type::from_le_bytes(buffer);

                    Ok(result)
                }
            }
        }
    };
}

read!(i8, I8);
read!(i16, I16);
read!(i32, I32);
read!(i64, I64);
read!(u8, U8);
read!(u16, U16);
read!(u32, U32);
read!(u64, U64);
