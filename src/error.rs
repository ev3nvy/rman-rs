use thiserror::Error;

/// Alias for a [`Result`][core::result::Result] with the error type [`ManifestError`].
pub type Result<T> = core::result::Result<T, ManifestError>;

/// This enum represents all possible errors that may occur when parsing a file.
#[derive(Error, Debug)]
pub enum ManifestError {
    /// The error was caused by a failure to seek to a desired offset.
    ///
    /// This error occurs when [`Seek`][std::io::Seek] fails.
    #[error("Could not seek to desired position. Error: {0}")]
    SeekError(std::io::Error),
    /// The error was caused by invalid magic bytes.
    ///
    /// This error occurs when the first four bytes (magic bytes) do not equal `0x52`, `0x4D`,
    /// `0x41` and `0x4E` (or `R`, `M`, `A`, `N` in ascii) respectively.
    ///
    /// Usually caused by providing a file that is not a Riot Manifest file.
    #[error("Invalid magic bytes (expected: \"0x4E414D52\", was: \"{0:#010x}\").")]
    InvalidMagicBytes(u32),
    /// The error was caused by invalid major version.
    ///
    /// This error occurs when the major version in the file header doesn't equal 2.
    ///
    /// Should only occur if the manifest format gets a major change or an update, Parser
    /// may no longer function if this happens.
    ///
    /// NOTE: The feature `version_error` must be enabled for this error to occur.
    #[error("Unsupported major version (expected: \"2\", was: \"{0}\").")]
    #[cfg(feature = "version_error")]
    InvalidMajor(u8),
    /// The error was caused by invalid minor version.
    ///
    /// This error occurs when the minor version in the file header doesn't equal 0.
    ///
    /// Should only occur if the manifest format gets a minor change or an update. Parser
    /// should still be functional if this happens,
    ///
    /// NOTE: The feature `version_error` must be enabled for this error to occur.
    #[error("Unsupported minor version (expected: \"0\", was: \"{0}\").")]
    #[cfg(feature = "version_error")]
    InvalidMinor(u8),
    /// The error was caused by an invalid offset.
    ///
    /// This error occurs when the offset is smaller or larger than the file itself.
    ///
    /// Should never happen for official, Riot-made manifests.
    #[error("Offset ({0}) is larger than the total file size.")]
    InvalidOffset(u32),
    /// The error was caused by compressed size being too large.
    ///
    /// This error occurs when the compressed size is larger than the file itself.
    ///
    /// Should never happen for official, Riot-made manifests.
    #[error("Compressed size ({0}) is larger than the total file size.")]
    CompressedSizeTooLarge(u32),
    /// The error was caused by a failure to read or write bytes on an IO stream.
    ///
    /// This error occurs when [`read_exact`][std::io::Read::read_exact], any `read_` method in
    /// [`byteorder::ReadBytesExt`], or [`File::open`][std::fs::File::open] fails.
    ///
    /// Usually caused by invalid or inaccessible path or unexpected eof when parsing a header.
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    /// The error was caused by a failure to convert from one number type to another.
    ///
    /// This error occurs when the conversion from or into [`usize`] fails.
    ///
    /// Should never fail on 32-bit (or higher) targets.
    #[error("Conversion failed. Error: {0}")]
    ConversionFailure(#[from] std::num::TryFromIntError),
    /// The error was caused by a failure to decompress zstd data.
    ///
    /// This error occurs when [`decompress`][zstd::bulk::decompress] fails.
    ///
    /// Should never happen for official, Riot-made manifests.
    #[error("{0}")]
    ZstdDecompressError(std::io::Error),
    /// The error was caused by a failure to parse [`FileEntry`][crate::entries::FileEntry] into
    /// [`File`][crate::File].
    ///
    /// This error occurs when either a [`directory_id`](crate::entries::FileEntry::directory_id)
    /// or [`parent_id`](crate::entries::DirectoryEntry::parent_id) points to an invalid
    /// [`DirectoryEntry`][crate::entries::DirectoryEntry] or when
    /// [`chunk_id`](crate::entries::FileEntry::chunk_ids) refers to an invalid
    /// [`ChunkEntry`][crate::entries::ChunkEntry].
    ///
    /// Should never happen for official, Riot-made manifests.
    #[error("{0}")]
    FileParseError(String),
    /// The error was caused by an invalid flatbuffer.
    ///
    /// This error occurs when a flatbuffer fails to verify.
    ///
    /// Should never happen for official, Riot-made manifests.
    #[error("{0}")]
    FlatbufferError(#[from] flatbuffers::InvalidFlatbuffer),
    /// The error was caused by a failure to process a [`Request`][reqwest::Request].
    ///
    /// This error occurs when [`Client::send()`](reqwest::RequestBuilder::send) fails.
    ///
    /// Usually occurs when there is no internet connection, or when an invalid bundle url was
    /// provided.
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),
}
