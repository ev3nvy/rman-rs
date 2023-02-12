pub mod header;
pub mod manifest;

use header::Header;
use manifest::ManifestData;

use std::fs;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use log::debug;

use crate::{ManifestError, Result};

/// Main parser object.
///
/// Depending on the function you call, it either parses a manifest
/// [from reader][crate::RiotManifest::from_reader] or [a file][crate::RiotManifest::from_path].
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RiotManifest {
    /// Parsed file header data.
    ///
    /// Stores information like [magic bytes](crate::Header::magic),
    /// [version](crate::Header::major), [flags](crate::Header::flags),
    /// [size](crate::Header::compressed_size), [offset](crate::Header::offset), etc.
    pub header: Header,
    /// Parsed flatbuffer data.
    ///
    /// Stores all of the [flatbuffer entries][crate::entries], as well as the [parsed files][crate::File].
    pub data: ManifestData,
}

impl RiotManifest {
    /// Loads data from a file and parses it.
    ///
    /// This is just a convenience method that [opens a file][std::fs::File::open],
    /// [buffers it][std::io::BufReader] and calls [`RiotManifest::from_reader`].
    ///
    /// # Errors
    ///
    /// If reading a file fails, the error [`IoError`][crate::ManifestError::IoError] is
    /// returned.
    ///
    /// If parsing fails, it propagates an error from [`RiotManifest::from_reader`].
    ///
    /// # Examples
    ///
    /// See
    /// [parsing a manifest file from path](index.html#example-parsing-a-manifest-file-from-path).
    ///
    /// [`RiotManifest::from_reader`]: crate::RiotManifest::from_reader
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = fs::File::open(path)?;
        let mut reader = BufReader::new(file);
        Self::from_reader(&mut reader)
    }

    /// Main parser method.
    ///
    /// Brief overview on how parsing the manifest is done:
    /// - attempts to [parse the header][crate::Header::from_reader]
    /// - [seeks][std::io::Seek] to the [offset](crate::Header::offset)
    /// - reads [x amount](crate::Header::compressed_size) of bytes to buffer
    /// - [decompresses][zstd::bulk::decompress] read bytes
    /// - decompressed data is a [flatbuffer binary], that is then
    /// [parsed][crate::ManifestData::parse].
    ///
    /// # Errors
    ///
    /// If parsing the header fails, it propagates an error from
    /// [`Header::from_reader`][crate::Header::from_reader].
    ///
    /// If seeking to offset fails, the error [`SeekError`][crate::ManifestError::SeekError] is
    /// returned.
    ///
    /// If converting [`compressed_size`](crate::Header::compressed_size) or
    /// [`uncompressed_size`](crate::Header::uncompressed_size) to [`usize`] fails, the error
    /// [`ConversionFailure`][crate::ManifestError::ConversionFailure] is returned.
    ///
    /// If reading compressed flatbuffer data fails, the error
    /// [`IoError`][crate::ManifestError::IoError] is returned.
    ///
    /// If zstd decompression fails, the error
    /// [`ZstdDecompressError`][crate::ManifestError::ZstdDecompressError] is returned.
    ///
    /// If parsing flatbuffer binary fails, it propagates an error from
    /// [`ManifestData::parse`][crate::ManifestData::parse].
    ///
    /// [flatbuffer binary]: https://github.com/ev3nvy/rman-schema
    pub fn from_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        let header = Header::from_reader(&mut reader)?;

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

        let data = ManifestData::parse(&decompressed)?;

        Ok(Self { header, data })
    }
}
