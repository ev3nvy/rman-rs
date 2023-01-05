use thiserror::Error;

use super::CursorError;

#[derive(Error, Debug)]
pub enum ManifestError<T = Box<dyn std::error::Error>> {
    #[error("Failed reading file: {0}")]
    ReadFileError(#[from] std::io::Error),
    #[error("{0}")]
    CursorError(CursorError<T>),
    #[error("Invalid magic bytes (expected: \"0x4E414D52\", was: \"{0:#010x}\").")]
    InvalidMagicBytes(u32),
    #[error("Unsupported major version (expected: \"2\", was: \"{0}\").")]
    #[cfg(feature = "version_error")]
    InvalidMajor(u8),
    #[error("Unsupported minor version (expected: \"0\", was: \"{0}\").")]
    #[cfg(feature = "version_error")]
    InvalidMinor(u8),
    #[error("Offset points outside of the file (file_size: \"{0}\", offset: \"{1}\").")]
    InvalidOffset(u32, u32),
    #[error("Compressed size overflows the file (file_size: \"{0}\", compressed_size: \"{1}\").")]
    CompressedSizeTooLarge(u32, u32),
    #[error("Conversion from \"{0}\" to \"{1}\" failed. Error: {2}")]
    ConversionFailure(String, String, T),
    #[error("{0}")]
    ZstdDecompressError(T),
    #[error("{0}")]
    FlatbufferError(T),
}
