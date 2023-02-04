use crate::generated::rman::Chunk;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ChunkEntry {
    pub id: u64,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
}

impl From<Chunk<'_>> for ChunkEntry {
    fn from(chunk: Chunk) -> Self {
        let id = chunk.id();
        let compressed_size = chunk.compressed_size();
        let uncompressed_size = chunk.uncompressed_size();

        Self {
            id,
            compressed_size,
            uncompressed_size,
        }
    }
}
