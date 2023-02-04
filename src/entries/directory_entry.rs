use crate::generated::rman::Directory;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    pub id: u64,
    pub parent_id: u64,
    pub name: String,
}

impl From<Directory<'_>> for DirectoryEntry {
    fn from(directory: Directory) -> Self {
        let id = directory.id();
        let parent_id = directory.parent_id();
        let name = directory.name().unwrap_or_default().to_string();

        Self {
            id,
            parent_id,
            name,
        }
    }
}
