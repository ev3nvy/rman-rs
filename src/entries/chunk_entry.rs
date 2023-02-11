use crate::generated::rman::Chunk;

/// Single chunk entry object.
///
/// This is identical to the schema in [rman-schema][rman-schema] and exists to provide a
/// persistent structure for the `ChunkEntry`.
///
/// [rman-schema]: https://github.com/ev3nvy/rman-schema
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ChunkEntry {
    /// Id of the chunk entry.
    pub id: i64,
    /// Chunk size before decompression.
    ///
    /// Mainly used when downloading files.
    pub compressed_size: u32,
    /// Chunk size after decompression.
    ///
    /// Mainly used when downloading files.
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
