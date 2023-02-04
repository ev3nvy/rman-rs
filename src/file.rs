use std::collections::HashMap;

use crate::entries::FileEntry;
use crate::{ManifestError, Result};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct File {
    pub id: u64,
    pub name: String,
    pub permissions: u8,
    pub size: u32,
    pub path: String,
    pub symlink: String,
    pub languages: Vec<String>,
    #[allow(dead_code)]
    chunks: Vec<(u64, u64, u32, u32)>,
}

impl File {
    pub fn parse(
        file: &FileEntry,
        language_entries: &HashMap<u8, String>,
        directories: &HashMap<u64, (String, u64)>,
        chunk_entries: &HashMap<u64, (u64, u64, u32, u32)>,
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
    pub fn download(&self) {
        unimplemented!("downloading not yet implemented");
    }
}
