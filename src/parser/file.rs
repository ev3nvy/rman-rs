use std::fs;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;

use log::debug;

use crate::error::ManifestError;

use super::{Header, Manifest};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RiotManifest {
    file_header: Header,
    manifest: Manifest,
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
        let file_header = Header::try_from(bytes)?;
        let mut cursor = Cursor::new(bytes);

        if let Err(error) = cursor.seek(SeekFrom::Start(file_header.offset.into())) {
            return Err(ManifestError::SeekError(error));
        };

        debug!("Attempting to convert \"compressed_size\" into \"usize\".");
        let compressed_size: usize = file_header.compressed_size.try_into()?;
        debug!("Successfully converted \"compressed_size\" into \"usize\".");

        let mut buf = vec![0u8; compressed_size];
        cursor.read_exact(&mut buf)?;

        debug!("Attempting to convert \"uncompressed_size\" into \"usize\".");
        let uncompressed_size: usize = file_header.uncompressed_size.try_into()?;
        debug!("Successfully converted \"uncompressed_size\" into \"usize\".");

        let decompressed = match zstd::bulk::decompress(&buf, uncompressed_size) {
            Ok(result) => result,
            Err(error) => return Err(ManifestError::ZstdDecompressError(error)),
        };

        let manifest = Manifest::try_from(decompressed)?;

        Ok(Self {
            file_header,
            manifest,
        })
    }
}

impl RiotManifest {
    pub fn manifest_header(&self) -> &Header {
        &self.file_header
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }
}
