use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestError<T = std::io::Error> {
    #[error("Failed reading file: {0}")]
    ReadFileError(T),
    #[error("Could not read value from buffer. Error: {0}")]
    ReadBytesError(#[from] std::io::Error),
    #[error("Could not seek to desired position. Error: {0}")]
    SeekError(std::io::Error),
    #[error("Invalid magic bytes (expected: \"0x4E414D52\", was: \"{0:#010x}\").")]
    InvalidMagicBytes(u32),
    #[error("Unsupported major version (expected: \"2\", was: \"{0}\").")]
    #[cfg(feature = "version_error")]
    InvalidMajor(u8),
    #[error("Unsupported minor version (expected: \"0\", was: \"{0}\").")]
    #[cfg(feature = "version_error")]
    InvalidMinor(u8),
    #[error("Offset ({0}) is larger than the total file size.")]
    InvalidOffset(u32),
    #[error("Compressed size ({0}) is larger than the total file size.")]
    CompressedSizeTooLarge(u32),
    #[error("Conversion failed. Error: {0}")]
    ConversionFailure(#[from] std::num::TryFromIntError),
    #[error("{0}")]
    ZstdDecompressError(T),
    #[error("{0}")]
    FlatbufferError(T),
}
