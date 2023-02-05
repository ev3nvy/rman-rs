use crate::generated::rman::Directory;

/// Single directory entry object.
///
/// This is identical to the schema in [rman-schema][rman-schema] and exists to provide a
/// persistent structure for the DirectoryEntry.
///
/// [rman-schema]: https://github.com/ev3nvy/rman-schema
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    /// Id of the directory entry.
    pub id: i64,
    /// Id of the parent directory entry.
    ///
    /// NOTE: root directory (which is tipically the first DirectoryEntry in the vector) typically
    /// has an `id` of 0, yet still has a `parent_id` of 0.
    pub parent_id: i64,
    /// Name of the directory entry.
    pub name: String,
}

impl From<Directory<'_>> for DirectoryEntry {
    fn from(directory: Directory) -> Self {
        let id = directory.id();
        let parent_id = directory.parent_id();
        let name = directory.name().unwrap_or_default().to_owned();

        Self {
            id,
            parent_id,
            name,
        }
    }
}
