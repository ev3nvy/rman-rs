use crate::generated::rman::Bundle;

use super::chunk_entry::ChunkEntry;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BundleEntry {
    pub id: u64,
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
