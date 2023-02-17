use std::collections::HashMap;

use crate::entries::{
    BundleEntry, ChunkingParamEntry, DirectoryEntry, FileEntry, KeyEntry, TagEntry,
};
use crate::generated::rman::root_as_manifest;
use crate::File;
use crate::Result;

/// Stores all of the flatbuffer data, as well as the parsed files.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ManifestData {
    /// Vector of [bundle entries][crate::entries::BundleEntry].
    pub bundle_entries: Vec<BundleEntry>,
    /// Vector of [directory entries][crate::entries::DirectoryEntry].
    pub directory_entries: Vec<DirectoryEntry>,
    /// Vector of [file entries][crate::entries::FileEntry].
    pub file_entries: Vec<FileEntry>,
    /// Vector of [key entries][crate::entries::KeyEntry].
    pub key_entries: Vec<KeyEntry>,
    /// Vector of [tag entries][crate::entries::TagEntry].
    pub tag_entries: Vec<TagEntry>,
    /// Vector of [chunking param entries][crate::entries::ChunkingParamEntry].
    pub chunking_param_entries: Vec<ChunkingParamEntry>,
    /// Vector of [files][crate::File].
    pub files: Vec<File>,
}

macro_rules! map_vector {
    ($manifest: ident, $name: ident, $entry: ident) => {
        $manifest
            .$name()
            .unwrap_or_default()
            .iter()
            .map(|i| $entry::from(i))
            .collect()
    };
}

impl ManifestData {
    /// Main flatbuffer parser method.
    ///
    /// This method tries to parse the entire flatbuffer binary.
    ///
    /// Yes, I know that a huge advantage of flatbuffers is the fact that they don't need to be
    /// parsed. However parsing the data in our case allows us to more easly explore it's
    /// contents and provides permanent objects so that the buffer can be discarded. Not to
    /// mention the fact that it's still plenty fast. :^)
    ///
    /// # Errors
    ///
    /// If verifying the flatbuffer fails, the error
    /// [`FlatbufferError`][crate::ManifestError::FlatbufferError] is returned.
    ///
    /// If parsing the [`File`][crate::File] fails, it propagates an error from
    /// [`File::parse`][crate::File::parse].
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let manifest = root_as_manifest(bytes)?;

        let bundle_entries: Vec<_> = map_vector!(manifest, bundles, BundleEntry);
        let directory_entries: Vec<_> = map_vector!(manifest, directories, DirectoryEntry);
        let file_entries: Vec<_> = map_vector!(manifest, files, FileEntry);
        let key_entries = map_vector!(manifest, keys, KeyEntry);
        let tag_entries: Vec<_> = map_vector!(manifest, tags, TagEntry);
        let chunking_param_entries = map_vector!(manifest, chunking_params, ChunkingParamEntry);

        let mapped_tags = Self::map_tags(&tag_entries);
        let mapped_directories = Self::map_directories(&directory_entries);
        let mapped_chunks = Self::map_chunks(&bundle_entries);

        let files = file_entries
            .iter()
            .map(|f| File::parse(f, &mapped_tags, &mapped_directories, &mapped_chunks))
            .collect::<Result<Vec<File>>>()?;

        Ok(Self {
            bundle_entries,
            directory_entries,
            file_entries,
            key_entries,
            tag_entries,
            chunking_param_entries,
            files,
        })
    }

    fn map_tags(tag_entries: &[TagEntry]) -> HashMap<u8, String> {
        tag_entries.iter().map(|l| (l.id, l.name.clone())).collect()
    }

    fn map_directories(directory_entries: &[DirectoryEntry]) -> HashMap<i64, (String, i64)> {
        directory_entries
            .iter()
            .map(|d| (d.id, (d.name.clone(), d.parent_id)))
            .collect()
    }

    fn map_chunks(bundle_entries: &[BundleEntry]) -> HashMap<i64, (i64, u32, u32, u32)> {
        bundle_entries
            .iter()
            .flat_map(|b| {
                b.chunks.iter().scan(0, move |offset, c| {
                    *offset += c.compressed_size;
                    Some((
                        c.id,
                        (
                            b.id,
                            *offset - c.compressed_size,
                            c.uncompressed_size,
                            c.compressed_size,
                        ),
                    ))
                })
            })
            .collect()
    }
}
