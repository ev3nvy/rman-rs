use std::io::Error;

use crate::generated::rman::Bundle;

use super::chunk_entry::ChunkEntry;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BundleEntry {
    pub id: u64,
    pub chunks: Vec<ChunkEntry>,
}

impl TryFrom<Bundle<'_>> for BundleEntry {
    type Error = Error;

    fn try_from(bundle: Bundle) -> Result<Self, Self::Error> {
        let id = bundle.id();
        let chunks = bundle.chunks().unwrap_or_default();

        let chunks = chunks
            .iter()
            .map(|c| ChunkEntry::try_from(c).unwrap())
            .collect();

        Ok(Self { id, chunks })
    }
}
