pub mod header;
pub mod manifest;

use header::Header;
use manifest::ManifestData;

use std::fs;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use log::debug;

use crate::error::ManifestError;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RiotManifest {
    pub header: Header,
    pub data: ManifestData,
}

impl RiotManifest {
    pub fn from_path<P>(path: P) -> Result<Self, ManifestError>
    where
        P: AsRef<Path>,
    {
        let file = match fs::File::open(path) {
            Ok(result) => result,
            Err(error) => return Err(ManifestError::ReadFileError(error)),
        };
        let mut reader = BufReader::new(file);
        Self::from_reader(&mut reader)
    }

    pub fn from_reader<R>(reader: &mut R) -> Result<Self, ManifestError>
    where
        R: Read + Seek,
    {
        let header = Header::from_reader(reader)?;

        if let Err(error) = reader.seek(SeekFrom::Start(header.offset.into())) {
            return Err(ManifestError::SeekError(error));
        };

        debug!("Attempting to convert \"compressed_size\" into \"usize\".");
        let compressed_size: usize = header.compressed_size.try_into()?;
        debug!("Successfully converted \"compressed_size\" into \"usize\".");

        let mut buf = vec![0u8; compressed_size];
        reader.read_exact(&mut buf)?;

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
