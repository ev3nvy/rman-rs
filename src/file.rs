use std::collections::HashMap;
use std::io::Write;

use log::debug;
use reqwest::header;
use reqwest::Client;
use reqwest::IntoUrl;

use crate::entries::FileEntry;
use crate::{ManifestError, Result};

/// Single file object.
///
/// Represents a file and it's properties that can be downloaded and written to a file system.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct File {
    /// Id of the file.
    pub id: i64,
    /// File name.
    pub name: String,
    /// Permissions for the given file.
    pub permissions: u8,
    /// Size of the file entry in bytes.
    pub size: u32,
    /// Absolute path to the file, where root is one of the
    /// [directory entries][crate::entries::DirectoryEntry].
    pub path: String,
    /// Symbolic link of the file.
    pub symlink: String,
    /// A vector of applicable languages.
    pub languages: Vec<String>,
    #[allow(dead_code)]
    chunks: Vec<(i64, u32, u32, u32)>,
}

impl File {
    /// Parses [`FileEntry`] into a [`File`] object.
    ///
    /// First parameter is a [`FileEntry`] that is parsed into a [`File`], the other three are
    /// [`HashMap`]s used for fast lookups for the required data.
    ///
    /// Here is how they are structured:
    /// - Parameter `language_entries` is a [`HashMap`] where the key is a
    /// [language id](crate::entries::LanguageEntry::id) and the value is a
    /// [language name](crate::entries::LanguageEntry::name).
    ///
    /// - Parameter `directories` is a [`HashMap`] where the key is a
    /// [directory id](crate::entries::DirectoryEntry::id) and the value is a tuple of:
    ///   - [directory name](crate::entries::DirectoryEntry::name)
    ///   - and [parent directory id](crate::entries::DirectoryEntry::parent_id).
    ///
    /// - Parameter `chunk_entries` is a [`HashMap`] where the key is a
    /// [chunk id](crate::entries::ChunkEntry::id) and the value is a tuple of:
    ///   - [bundle id](crate::entries::BundleEntry::id),
    ///   - offset in bundle (to this specific chunk),
    ///   - [uncompressed size](crate::entries::ChunkEntry::uncompressed_size)
    ///   - and [compressed size](crate::entries::ChunkEntry::compressed_size).
    ///
    /// [`File`]: crate::File
    /// [`FileEntry`]: crate::entries::FileEntry
    ///
    /// # Errors
    ///
    /// If a directory with [provided id](crate::entries::FileEntry::directory_id) or
    /// [parent id](crate::entries::DirectoryEntry::parent_id) does not exist within the
    /// `directories` [`HashMap`], or if a chunk with
    /// [chunk id](crate::entries::FileEntry::chunk_ids) does not exist within the `chunk_entries`
    /// [`HashMap`], the error [`FileParseError`][crate::ManifestError::FileParseError] is
    /// returned.
    pub fn parse(
        file: &FileEntry,
        language_entries: &HashMap<u8, String>,
        directories: &HashMap<i64, (String, i64)>,
        chunk_entries: &HashMap<i64, (i64, u32, u32, u32)>,
    ) -> Result<Self> {
        let id = file.id;
        let name = file.name.to_owned();
        let permissions = file.permissions;
        let size = file.size;
        let symlink = file.symlink.to_owned();
        let language_mask = file.language_mask;
        let chunk_ids = file.chunk_ids.to_owned();

        let mut directory_id = file.directory_id;
        let mut path = String::new();

        while directory_id != 0 {
            let Some((dir_name, parent_id)) = directories.get(&directory_id) else {
                let message = format!("Could not find a directory with the following id: \"{directory_id}\".");
                return Err(ManifestError::FileParseError(message));
            };
            path = format!("{dir_name}/{path}");
            directory_id = *parent_id;
        }

        path.push_str(&name);

        let mut languages = Vec::new();

        for i in 0..64 {
            if (language_mask & (1u64 << i)) == 0 {
                continue;
            }

            if let Some(lang_name) = language_entries.get(&(i + 1)) {
                languages.push(lang_name.to_owned());
            }
        }

        let mut chunks = Vec::new();

        for chunk_id in chunk_ids {
            let Some(chunk) = chunk_entries.get(&chunk_id) else {
                let message = format!("Could not find a chunk with the following id: \"{chunk_id}\".");
                return Err(ManifestError::FileParseError(message));
            };
            chunks.push(chunk.to_owned());
        }

        let file = Self {
            id,
            name,
            permissions,
            size,
            path,
            symlink,
            languages,
            chunks,
        };
        Ok(file)
    }
}

impl File {
    /// Function to download the associated file contents.
    ///
    /// This is done by looping through all of the chunks of this file, and for each loop:
    /// - get the [bundle id](crate::entries::BundleEntry::id) it belongs to, and convert it to
    /// hexadecimal value with a fixed size of 16 (if the length is less than 16, zeros are
    /// padded to the left).
    /// - download the chunk from the url using the range header
    /// - [decompress the chunk][zstd::bulk::decompress]
    /// - write chunk.
    ///
    /// # Errors
    ///
    /// If downloading fails, the error [`ReqwestError`][crate::ManifestError::ReqwestError] is
    /// returned.
    ///
    /// If converting [`uncompressed_size`](crate::Header::uncompressed_size) to [`usize`] fails,
    /// the error [`ConversionFailure`][crate::ManifestError::ConversionFailure] is returned.
    ///
    /// If zstd decompression fails, the error
    /// [`ZstdDecompressError`][crate::ManifestError::ZstdDecompressError] is returned.
    ///
    /// If writing to io stream fails, the error [`IoError`][crate::ManifestError::IoError] is
    /// returned.
    ///
    /// # Examples
    ///
    /// See [downloading a file](index.html#example-downloading-a-file).
    pub async fn download<W: Write, U: IntoUrl>(&self, mut writer: W, bundle_url: U) -> Result<()> {
        let client = Client::new();

        for (bundle_id, offset, uncompressed_size, compressed_size) in &self.chunks {
            let from = offset;
            let to = offset + compressed_size - 1;

            let response = client
                .get(format!("{}/{bundle_id:016X}.bundle", bundle_url.as_str()))
                .header(header::RANGE, format!("bytes={from}-{to}"))
                .send()
                .await?;

            debug!("Attempting to convert \"uncompressed_size\" into \"usize\".");
            let uncompressed_size: usize = uncompressed_size.to_owned().try_into()?;
            debug!("Successfully converted \"uncompressed_size\" into \"usize\".");

            let decompressed_chunk =
                match zstd::bulk::decompress(&response.bytes().await?, uncompressed_size) {
                    Ok(result) => result,
                    Err(error) => return Err(ManifestError::ZstdDecompressError(error)),
                };

            writer.write_all(&decompressed_chunk)?;
        }

        Ok(())
    }
}
