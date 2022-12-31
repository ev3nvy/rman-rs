use std::collections::HashMap;

use crate::error::Error;
use crate::generated::rman::File;

#[derive(Debug, Default)]
pub struct FileEntry {
    pub id: u64,
    pub name: String,
    pub permissions: u8,
    pub size: u32,
    pub path: String,
    pub link: String,
    pub languages: Vec<String>,
    #[allow(dead_code)]
    chunks: Vec<(u64, u64, u32, u32)>,
}

impl FileEntry {
    pub fn try_parse(
        file: &File,
        language_entries: &HashMap<u8, String>,
        directories: &HashMap<u64, (String, u64)>,
        chunk_entries: &HashMap<u64, (u64, u64, u32, u32)>,
    ) -> Result<Self, Error> {
        let id = file.id();
        let name = file.name().unwrap_or_default().to_string();
        let permissions = file.permissions();
        let size = file.size_();
        let link = file.link().unwrap_or_default().to_string();
        let language_mask = file.language_mask();
        let chunk_ids = file.chunk_ids().unwrap();

        let mut directory_id = file.directory_id();
        let mut path = String::new();

        while directory_id != 0 {
            let (dir_name, parent_id) = directories.get(&directory_id).unwrap();
            path = format!("{}/{}", dir_name, path);
            directory_id = *parent_id;
        }

        path.push_str(&name);

        let mut languages = Vec::new();

        for i in 0..64 {
            if (language_mask & (1u64 << i)) == 0 {
                continue;
            }

            if let Some(lang_name) = language_entries.get(&(i + 1)) {
                languages.push(lang_name.to_string());
            }
        }

        let mut chunks = Vec::new();

        for chunk_id in chunk_ids {
            let chunk = chunk_entries.get(&chunk_id).unwrap();
            chunks.push(chunk.to_owned());
        }

        let file = Self {
            id,
            name,
            permissions,
            size,
            path,
            link,
            languages,
            chunks,
        };
        Ok(file)
    }
}
