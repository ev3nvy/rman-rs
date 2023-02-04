pub mod header;
pub mod manifest;

use header::Header;
use manifest::ManifestData;

use std::fs;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;

use log::debug;

use crate::error::ManifestError;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RiotManifest {
    pub header: Header,
    pub data: ManifestData,
}

impl RiotManifest {
    pub fn try_from_path<P>(path: P) -> Result<Self, ManifestError>
    where
        P: AsRef<Path>,
    {
        let bytes = match fs::read(path) {
            Ok(result) => result,
            Err(error) => return Err(ManifestError::ReadFileError(error)),
        };
        Self::try_from(&bytes[..])
    }
}

impl TryFrom<Vec<u8>> for RiotManifest {
    type Error = ManifestError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(&bytes[..])
    }
}

impl TryFrom<&[u8]> for RiotManifest {
    type Error = ManifestError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let header = Header::try_from(bytes)?;
        let mut cursor = Cursor::new(bytes);

        if let Err(error) = cursor.seek(SeekFrom::Start(header.offset.into())) {
            return Err(ManifestError::SeekError(error));
        };

        debug!("Attempting to convert \"compressed_size\" into \"usize\".");
        let compressed_size: usize = header.compressed_size.try_into()?;
        debug!("Successfully converted \"compressed_size\" into \"usize\".");

        let mut buf = vec![0u8; compressed_size];
        cursor.read_exact(&mut buf)?;

        debug!("Attempting to convert \"uncompressed_size\" into \"usize\".");
        let uncompressed_size: usize = header.uncompressed_size.try_into()?;
        debug!("Successfully converted \"uncompressed_size\" into \"usize\".");

        let decompressed = match zstd::bulk::decompress(&buf, uncompressed_size) {
            Ok(result) => result,
            Err(error) => return Err(ManifestError::ZstdDecompressError(error)),
        };

        let data = ManifestData::parse(decompressed)?;

        Ok(Self { header, data })
    }
}
