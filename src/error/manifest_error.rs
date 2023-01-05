use thiserror::Error;

use super::CursorError;

#[derive(Error, Debug)]
pub enum ManifestError<T = std::io::Error> {
    #[error("Failed reading file: {0}")]
    ReadFileError(T),
    #[error("{0}")]
    CursorError(#[from] CursorError),
    #[error("Invalid magic bytes (expected: \"0x4E414D52\", was: \"{0:#010x}\").")]
    InvalidMagicBytes(u32),
    #[error("Unsupported major version (expected: \"2\", was: \"{0}\").")]
    #[cfg(feature = "version_error")]
    InvalidMajor(u8),
    #[error("Unsupported minor version (expected: \"0\", was: \"{0}\").")]
    #[cfg(feature = "version_error")]
    InvalidMinor(u8),
    #[error(
        "Offset points outside of the file (file_size: \"{file_size}\", offset: \"{offset}\")."
    )]
    InvalidOffset { file_size: u32, offset: u32 },
    #[error("Compressed size overflows the file (file_size: \"{file_size}\", compressed_size: \"{compressed_size}\").")]
    CompressedSizeTooLarge {
        file_size: u32,
        compressed_size: u32,
    },
    #[error("Conversion failed. Error: {0}")]
    ConversionFailure(#[from] std::num::TryFromIntError),
    #[error("{0}")]
    ZstdDecompressError(T),
    #[error("{0}")]
    FlatbufferError(T),
}
