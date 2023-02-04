use std::collections::HashMap;

use crate::entries::{BundleEntry, DirectoryEntry, FileEntry, KeyEntry, LanguageEntry, ParamEntry};
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
    /// Vector of [language entries][crate::entries::LanguageEntry].
    pub language_entries: Vec<LanguageEntry>,
    /// Vector of [param entries][crate::entries::ParamEntry].
    pub param_entries: Vec<ParamEntry>,
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
        let language_entries: Vec<_> = map_vector!(manifest, languages, LanguageEntry);
        let param_entries = map_vector!(manifest, params, ParamEntry);

        let mapped_languages = Self::map_languages(&language_entries);
        let mapped_directories = Self::map_directories(&directory_entries);
        let mapped_chunks = Self::map_chunks(&bundle_entries);

        let files = file_entries
            .iter()
            .map(|f| File::parse(f, &mapped_languages, &mapped_directories, &mapped_chunks))
            .collect::<Result<Vec<File>>>()?;

        Ok(Self {
            bundle_entries,
            directory_entries,
            file_entries,
            key_entries,
            language_entries,
            param_entries,
            files,
        })
    }

    fn map_languages(language_entries: &[LanguageEntry]) -> HashMap<u8, String> {
        language_entries
            .iter()
            .map(|l| (l.id, l.name.to_owned()))
            .collect()
    }

    fn map_directories(directory_entries: &[DirectoryEntry]) -> HashMap<u64, (String, u64)> {
        directory_entries
            .iter()
            .map(|d| (d.id, (d.name.to_owned(), d.parent_id)))
            .collect()
    }

    fn map_chunks(bundle_entries: &[BundleEntry]) -> HashMap<u64, (u64, u64, u32, u32)> {
        bundle_entries
            .iter()
            .flat_map(|b| {
                b.chunks.iter().scan(0, move |offset, c| {
                    *offset += u64::from(c.compressed_size);
                    Some((
                        c.id,
                        (
                            b.id,
                            *offset - u64::from(c.compressed_size),
                            c.uncompressed_size,
                            c.compressed_size,
                        ),
                    ))
                })
            })
            .collect()
    }
}
