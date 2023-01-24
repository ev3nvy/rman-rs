use std::fs;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;

use log::debug;

use crate::error::ManifestError;

use super::{FileHeader, Manifest};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ManifestFile {
    file_header: FileHeader,
    manifest: Manifest,
}

impl ManifestFile {
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

impl TryFrom<Vec<u8>> for ManifestFile {
    type Error = ManifestError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(&bytes[..])
    }
}

impl TryFrom<&[u8]> for ManifestFile {
    type Error = ManifestError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let file_header = FileHeader::try_from(bytes)?;
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

impl ManifestFile {
    pub fn manifest_header(&self) -> &FileHeader {
        &self.file_header
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod helpers {
        pub const VALID_FILE: &str = "assets/valid.manifest";
        pub const VALID_EMPTY_FILE: &str = "assets/valid_empty.manifest";
    }

    #[test]
    pub fn should_parse_from_path_when_valid_manifest() {
        if let Err(error) = ManifestFile::try_from_path(helpers::VALID_FILE) {
            panic!(
                "there was an error when trying to parse the manifest, manifest: {:?}",
                error
            );
        };
    }

    #[test]
    pub fn should_have_correct_values_when_valid_manifest() {
        let manifest = ManifestFile::try_from_path(helpers::VALID_FILE).unwrap();

        // FIXME: don't check for equality on compressed size and uncompressed size
        // compressed and uncompressed size could change in the future
        let header = FileHeader {
            magic: 0x4E414D52,
            major: 2,
            minor: 0,
            flags: 512,
            offset: 28,
            compressed_size: 202,
            manifest_id: 0,
            uncompressed_size: 392,
        };

        assert_eq!(
            manifest.manifest_header(),
            &header,
            "manifest header should be the same"
        );
        assert_eq!(
            manifest.manifest().bundle_entries.len(),
            1,
            "should have 1 bundle entry"
        );
        assert_eq!(
            manifest.manifest().bundle_entries[0].chunks.len(),
            2,
            "bundle entry should have 2 chunks"
        );
        assert_eq!(
            manifest.manifest().directory_entries.len(),
            2,
            "should have 2 directory entries"
        );
        assert_eq!(
            manifest.manifest().file_entries.len(),
            1,
            "should have 1 file entry"
        );
        assert_eq!(
            manifest.manifest().files.len(),
            1,
            "should parse into 1 file"
        );
        assert_eq!(
            manifest.manifest().key_entries.len(),
            1,
            "should have 1 key entry"
        );
        assert_eq!(
            manifest.manifest().language_entries.len(),
            1,
            "should have 1 language entries"
        );
        assert_eq!(
            manifest.manifest().param_entries.len(),
            1,
            "should have 1 param entry"
        );
    }

    #[test]
    pub fn should_parse_from_path_when_valid_empty_manifest() {
        if let Err(error) = ManifestFile::try_from_path(helpers::VALID_EMPTY_FILE) {
            panic!(
                "there was an error when trying to parse the manifest, manifest: {:?}",
                error
            );
        };
    }

    #[test]
    pub fn should_have_correct_values_when_valid_empty_manifest() {
        let manifest = ManifestFile::try_from_path(helpers::VALID_EMPTY_FILE).unwrap();

        // FIXME: don't check for equality on compressed size and uncompressed size
        // compressed and uncompressed size could change in the future
        let header = FileHeader {
            magic: 0x4E414D52,
            major: 2,
            minor: 0,
            flags: 512,
            offset: 28,
            compressed_size: 59,
            manifest_id: 0,
            uncompressed_size: 72,
        };

        assert_eq!(
            manifest.manifest_header(),
            &header,
            "manifest header should be the same"
        );
        assert_eq!(
            manifest.manifest().bundle_entries.len(),
            0,
            "should have 0 bundle entries"
        );
        assert_eq!(
            manifest.manifest().directory_entries.len(),
            0,
            "should have 0 directory entries"
        );
        assert_eq!(
            manifest.manifest().file_entries.len(),
            0,
            "should have 0 file entries"
        );
        assert_eq!(
            manifest.manifest().files.len(),
            0,
            "should parse into 0 files"
        );
        assert_eq!(
            manifest.manifest().key_entries.len(),
            0,
            "should have 0 key entries"
        );
        assert_eq!(
            manifest.manifest().language_entries.len(),
            0,
            "should have 0 language entries"
        );
        assert_eq!(
            manifest.manifest().param_entries.len(),
            0,
            "should have 0 param entries"
        );
    }
}
