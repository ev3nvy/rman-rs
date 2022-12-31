use std::fmt::{Debug, Display, Formatter, Result};

use super::CursorError;

#[derive(Debug)]
pub enum Error<T = Box<dyn std::error::Error>> {
    ReadFileError(T),
    CursorError(CursorError<T>),
    InvalidMagicBytes(u32),
    #[cfg(feature = "version_error")]
    InvalidMajor(u8),
    #[cfg(feature = "version_error")]
    InvalidMinor(u8),
    InvalidOffset(u32, u32),
    CompressedSizeTooLarge(u32, u32),
    ConversionFailure(String, String, T),
    ZstdDecompressError(T),
    FlatbufferError(T),
}

impl<T: std::error::Error> std::error::Error for Error<T> {}

impl<T: std::error::Error> Display for Error<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::ReadFileError(error) => writeln!(f, "Failed reading file: {}", error),
            Error::CursorError(error) => writeln!(f, "{}", error),
            Error::InvalidMagicBytes(magic) => writeln!(
                f,
                "Invalid magic bytes (expected: \"0x4E414D52\", was: \"{:#010x}\").",
                magic
            ),
            #[cfg(feature = "version_error")]
            Error::InvalidMajor(major) => writeln!(
                f,
                "Unsupported major version (expected: \"2\", was: \"{}\").",
                major
            ),
            #[cfg(feature = "version_error")]
            Error::InvalidMinor(minor) => writeln!(
                f,
                "Unsupported minor version (expected: \"0\", was: \"{}\").",
                minor
            ),
            Error::InvalidOffset(size, offset) => writeln!(
                f,
                "Offset points outside of the file (file_size: \"{}\", offset: \"{}\").",
                size, offset,
            ),
            Error::CompressedSizeTooLarge(size, compressed_size) => writeln!(
                f,
                "Compressed size overflows the file (file_size: \"{}\", compressed_size: \"{}\").",
                size, compressed_size,
            ),
            Error::ConversionFailure(from, to, error) => writeln!(
                f,
                "Conversion from \"{}\" to \"{}\" failed. Error: {}",
                from, to, error
            ),
            Error::ZstdDecompressError(error) => writeln!(f, "{}", error),
            Error::FlatbufferError(error) => {
                writeln!(f, "{}", error)
            }
        }
    }
}
