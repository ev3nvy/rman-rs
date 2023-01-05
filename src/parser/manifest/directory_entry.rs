use std::io::Error;

use crate::generated::rman::Directory;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    pub id: u64,
    pub parent_id: u64,
    pub name: String,
}

impl TryFrom<Directory<'_>> for DirectoryEntry {
    type Error = Error;

    fn try_from(directory: Directory) -> Result<Self, Self::Error> {
        let id = directory.id();
        let parent_id = directory.parent_id();
        let name = directory.name().unwrap_or_default().to_string();

        Ok(Self {
            id,
            parent_id,
            name,
        })
    }
}
