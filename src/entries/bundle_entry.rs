use crate::generated::rman::Bundle;

use super::chunk_entry::ChunkEntry;

/// Single bundle entry object.
///
/// This is identical to the schema in [rman-schema][rman-schema] and exists to provide a
/// persistent structure for the BundleEntry.
///
/// [rman-schema]: https://github.com/ev3nvy/rman-schema
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BundleEntry {
    /// Id of the bundle entry.
    pub id: u64,
    /// A vector of chunk entries.
    pub chunks: Vec<ChunkEntry>,
}

impl From<Bundle<'_>> for BundleEntry {
    fn from(bundle: Bundle) -> Self {
        let id = bundle.id();
        let chunks = bundle.chunks().unwrap_or_default();

        let chunks = chunks.iter().map(ChunkEntry::from).collect();

        Self { id, chunks }
    }
}
