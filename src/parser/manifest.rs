pub mod file_entry;

use std::collections::HashMap;

pub use file_entry::FileEntry;

use crate::error::Error;
use crate::generated::rman::{root_as_manifest, Manifest as ManifestFlatbuffer};

#[derive(Debug, Default)]
pub struct Manifest {
    pub files: Vec<FileEntry>,
}

impl TryFrom<&[u8]> for Manifest {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let manifest = root_as_manifest(&bytes).unwrap();

        let languages = Self::try_map_languages(&manifest);
        let directories = Self::try_map_directories(&manifest);
        let chunks = Self::try_map_chunks(&manifest);

        let file_entries = manifest
            .files()
            .unwrap_or(flatbuffers::Vector::default());

        let mut files = Vec::with_capacity(file_entries.len());

        for file in file_entries {
            let file = FileEntry::try_parse(&file, &languages, &directories, &chunks);
            files.push(file?);
        }

        Ok(Self { files })
    }
}

impl Manifest {
    fn try_map_languages(manifest: &ManifestFlatbuffer) -> HashMap<u8, String> {
        let language_entries = manifest
            .languages()
            .unwrap_or(flatbuffers::Vector::default());

        let mut languages = HashMap::with_capacity(language_entries.len());

        for language in language_entries {
            let id = language.id();
            let name = language.name().unwrap_or_default();

            languages.insert(id, name.to_string());
        }

        languages
    }

    fn try_map_directories(manifest: &ManifestFlatbuffer) -> HashMap<u64, (String, u64)> {
        let directory_entries = manifest
            .directories()
            .unwrap_or(flatbuffers::Vector::default());

        let mut directories = HashMap::with_capacity(directory_entries.len());

        for directory in directory_entries {
            let id = directory.id();
            let parent_id = directory.parent_id();
            let name = directory.name().unwrap_or_default();

            directories.insert(id, (name.to_string(), parent_id));
        }

        directories
    }

    fn try_map_chunks(manifest: &ManifestFlatbuffer) -> HashMap<u64, (u64, u64, u32, u32)> {
        let bundle_entries = manifest
            .bundles()
            .unwrap_or(flatbuffers::Vector::default());

        let mut chunks = HashMap::new();

        for bundle in bundle_entries {
            let chunk_entries = bundle
                .chunks()
                .unwrap_or(flatbuffers::Vector::default());

            let mut offset: u64 = 0;

            for chunk in chunk_entries {
                let id = chunk.id();
                let parent_id = bundle.id();
                let uncompressed_size = chunk.uncompressed_size();
                let compressed_size = chunk.compressed_size();

                chunks.insert(id, (parent_id, offset, uncompressed_size, compressed_size));
                offset += u64::from(compressed_size);
            }
        }

        chunks
    }
}

