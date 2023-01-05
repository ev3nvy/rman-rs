use std::io::Error;

use crate::generated::rman::Chunk;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ChunkEntry {
    pub id: u64,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
}

impl TryFrom<Chunk<'_>> for ChunkEntry {
    type Error = Error;

    fn try_from(chunk: Chunk) -> Result<Self, Self::Error> {
        let id = chunk.id();
        let compressed_size = chunk.compressed_size();
        let uncompressed_size = chunk.uncompressed_size();

        Ok(Self {
            id,
            compressed_size,
            uncompressed_size,
        })
    }
}
