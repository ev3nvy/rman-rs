mod bundle_entry;
mod chunk_entry;
mod directory_entry;
mod file;
mod file_entry;
mod key_entry;
mod language_entry;
mod param_entry;

use std::collections::HashMap;

use crate::error::Error;
use crate::generated::rman::root_as_manifest;

pub use self::bundle_entry::BundleEntry;
pub use self::directory_entry::DirectoryEntry;
pub use self::file::File;
pub use self::file_entry::FileEntry;
pub use self::key_entry::KeyEntry;
pub use self::language_entry::LanguageEntry;
pub use self::param_entry::ParamEntry;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Manifest {
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
            .map(|i| $entry::try_from(i).unwrap_or_default())
            .collect()
    };
}

impl TryFrom<Vec<u8>> for Manifest {
    type Error = Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let manifest = root_as_manifest(&bytes).unwrap();

        let bundle_entries = map_vector!(manifest, bundles, BundleEntry);
        let directory_entries = map_vector!(manifest, directories, DirectoryEntry);
        let file_entries: Vec<_> = map_vector!(manifest, files, FileEntry);
        let key_entries = map_vector!(manifest, keys, KeyEntry);
        let language_entries = map_vector!(manifest, languages, LanguageEntry);
        let param_entries = map_vector!(manifest, params, ParamEntry);

        let mapped_languages = Self::try_map_languages(&language_entries);
        let mapped_directories = Self::try_map_directories(&directory_entries);
        let mapped_chunks = Self::try_map_chunks(&bundle_entries);

        let files = file_entries
            .iter()
            .map(|f| {
                File::try_parse(f, &mapped_languages, &mapped_directories, &mapped_chunks).unwrap()
            })
            .collect();

        Ok(Self {
            bundle_entries,
            directory_entries,
            file_entries,
            key_entries,
            language_entries,
            param_entries,
            files,
        })
        // Ok(Self { languages, files })
    }
}

impl Manifest {
    fn try_map_languages(language_entries: &Vec<LanguageEntry>) -> HashMap<u8, String> {
        let language_entries: Vec<(u8, String)> = language_entries
            .iter()
            .map(|l| (l.id, l.name.to_string()))
            .collect();

        language_entries.into_iter().collect()
    }

    fn try_map_directories(directory_entries: &Vec<DirectoryEntry>) -> HashMap<u64, (String, u64)> {
        let directory_entries: Vec<(u64, (String, u64))> = directory_entries
            .iter()
            .map(|d| (d.id, (d.name.to_string(), d.parent_id)))
            .collect();

        directory_entries.into_iter().collect()
    }

    fn try_map_chunks(bundle_entries: &Vec<BundleEntry>) -> HashMap<u64, (u64, u64, u32, u32)> {
        let chunk_entries: Vec<(u64, (u64, u64, u32, u32))> = bundle_entries
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
            .collect();

        chunk_entries.into_iter().collect()
    }
}
