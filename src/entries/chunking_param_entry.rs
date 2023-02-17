use crate::generated::rman::ChunkingParam;

/// Single chunking param entry object.
///
/// This is identical to the schema in [rman-schema][rman-schema] and exists to provide a
/// persistent structure for the `ChunkingParamEntry`.
///
/// [rman-schema]: https://github.com/ev3nvy/rman-schema
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ChunkingParamEntry {
    /// Field with an unknown function and type (it might also be an [`i16`]).
    pub unk0: u16,
    /// Determines the hash type used when generating chunks.
    ///
    /// - 0 - Invalid/None
    /// - 1 - SHA256
    /// - 2 - SHA512
    /// - 3 - RIOT_HKDF
    ///
    /// These values are copied straight from
    /// [moonshadow565's implementation][moonshadow565-rman-rchunk].
    /// More about hashing on their [official blog][manifest].
    ///
    /// [moonshadow565-rman-rchunk]: https://github.com/moonshadow565/rman/blob/master/lib/rlib/rchunk.hpp
    /// [manifest]: https://technology.riotgames.com/news/supercharging-data-delivery-new-league-patcher
    pub chunking_version: u8,
    /// Minimum chunk size.
    ///
    /// For more info on what this does, see [Riot's blog][manifest] or [FastCDC pdf][fast-cdc].
    ///
    /// [manifest]: https://technology.riotgames.com/news/supercharging-data-delivery-new-league-patcher
    /// [fast-cdc]: https://www.usenix.org/system/files/conference/atc16/atc16-paper-xia.pdf
    pub min_chunk_size: u32,
    /// Chunk size.
    ///
    /// For more info on what this does, see [Riot's blog][manifest] or [FastCDC pdf][fast-cdc].
    ///
    /// [manifest]: https://technology.riotgames.com/news/supercharging-data-delivery-new-league-patcher
    /// [fast-cdc]: https://www.usenix.org/system/files/conference/atc16/atc16-paper-xia.pdf
    pub chunk_size: u32,
    /// Maximum chunk size.
    ///
    /// For more info on what this does, see [Riot's blog][manifest] or [FastCDC pdf][fast-cdc].
    ///
    /// [manifest]: https://technology.riotgames.com/news/supercharging-data-delivery-new-league-patcher
    /// [fast-cdc]: https://www.usenix.org/system/files/conference/atc16/atc16-paper-xia.pdf
    pub max_chunk_size: u32,
}

impl From<ChunkingParam<'_>> for ChunkingParamEntry {
    fn from(chunking_param: ChunkingParam) -> Self {
        let unk0 = chunking_param.unk0();
        let chunking_version = chunking_param.chunking_version();
        let min_chunk_size = chunking_param.min_chunk_size();
        let chunk_size = chunking_param.chunk_size();
        let max_chunk_size = chunking_param.max_chunk_size();

        Self {
            unk0,
            chunking_version,
            min_chunk_size,
            chunk_size,
            max_chunk_size,
        }
    }
}
