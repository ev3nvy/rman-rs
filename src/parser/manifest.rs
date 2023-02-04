use std::collections::HashMap;

use crate::entries::{BundleEntry, DirectoryEntry, FileEntry, KeyEntry, LanguageEntry, ParamEntry};
use crate::generated::rman::root_as_manifest;
use crate::File;
use crate::Result;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ManifestData {
    pub bundle_entries: Vec<BundleEntry>,
    pub directory_entries: Vec<DirectoryEntry>,
    pub file_entries: Vec<FileEntry>,
    pub key_entries: Vec<KeyEntry>,
    pub language_entries: Vec<LanguageEntry>,
    pub param_entries: Vec<ParamEntry>,
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
    pub fn parse(bytes: Vec<u8>) -> Result<Self> {
        let manifest = root_as_manifest(&bytes).unwrap();

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
            .map(|l| (l.id, l.name.to_string()))
            .collect()
    }

    fn map_directories(directory_entries: &[DirectoryEntry]) -> HashMap<u64, (String, u64)> {
        directory_entries
            .iter()
            .map(|d| (d.id, (d.name.to_string(), d.parent_id)))
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
