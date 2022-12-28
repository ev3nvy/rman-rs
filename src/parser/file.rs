use std::fs;
use std::io::{Seek, SeekFrom, Read};
use std::path::{Path, PathBuf};

use crate::error::{CursorError, Error};
use crate::structs::Cursor;

use super::{FileHeader, Manifest};

#[derive(Debug)]
pub struct ManifestFile {
    pub file_header: FileHeader,
    pub manifest: Manifest,
}

impl ManifestFile {
    pub fn new(file_header: FileHeader, manifest: Manifest) -> Self {
        Self {
            file_header,
            manifest,
        }
    }

    pub fn try_from_path<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>
    {
        let bytes = match fs::read(path) {
            Ok(result) => result,
            Err(error) => return Err(Error::ReadFileError(error.into())),
        };
        Self::try_from(&bytes[..])
    }
}

impl TryFrom<&[u8]> for ManifestFile {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let file_header = FileHeader::try_from(bytes)?;
        let mut cursor = Cursor::from(bytes);

        if let Err(error) = cursor.cursor.seek(SeekFrom::Start(file_header.offset.into())) {
            let cursor_error = CursorError::SeekError(error.into());
            return Err(Error::CursorError(cursor_error));
        }

        let compressed_size: usize = match file_header.compressed_size.try_into() {
            Ok(result) => result,
            Err(error) => {
                let error = Error::ConversionFailure(String::from("u32"), String::from("usize"), error.into());
                return Err(error);
            },
        };
        let mut buf = vec![0u8; compressed_size];

        if let Err(error) = cursor.cursor.read_exact(&mut buf) {
            let cursor_error = CursorError::ReadManyError(error.into());
            return Err(Error::CursorError(cursor_error));
        }

        let uncompressed_size: usize = match file_header.uncompressed_size.try_into() {
            Ok(result) => result,
            Err(error) => {
                let error = Error::ConversionFailure(String::from("u32"), String::from("usize"), error.into());
                return Err(error);
            },
        };
        let decompressed = match zstd::bulk::decompress(&mut buf, uncompressed_size) {
            Ok(result) => result,
            Err(error) => return Err(Error::ZstdDecompressError(error.into())),
        };

        let manifest = Manifest::try_from(&decompressed[..])?;

        Ok(Self {
            file_header,
            manifest,
        })
    }
}

impl TryFrom<Vec<u8>> for ManifestFile {
    type Error = Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(&bytes[..])
    }
}

impl TryFrom<String> for ManifestFile {
    type Error = Error;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        Self::try_from_path(path)
    }
}

impl TryFrom<PathBuf> for ManifestFile {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from_path(path)
    }
}
