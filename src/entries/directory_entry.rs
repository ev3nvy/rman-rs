use crate::generated::{rman_v2_0, rman_v2_1};

/// Single directory entry object.
///
/// This is identical to the schema in [rman-schema][rman-schema] and exists to provide a
/// persistent structure for the `DirectoryEntry`.
///
/// [rman-schema]: https://github.com/ev3nvy/rman-schema
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    /// Id of the directory entry.
    pub id: i64,
    /// Id of the parent directory entry.
    ///
    /// NOTE: root directory (which is typically the first [`DirectoryEntry`] in the vector) typically
    /// has an `id` of 0, yet still has a `parent_id` of 0.
    pub parent_id: i64,
    /// Name of the directory entry.
    pub name: String,
}

impl From<rman_v2_0::Directory<'_>> for DirectoryEntry {
    fn from(directory: rman_v2_0::Directory) -> Self {
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

impl From<rman_v2_1::Directory<'_>> for DirectoryEntry {
    fn from(directory: rman_v2_1::Directory) -> Self {
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
