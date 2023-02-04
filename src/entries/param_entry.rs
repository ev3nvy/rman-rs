use crate::generated::rman::Param;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ParamEntry {
    pub unk0: u16,
    pub chunking_version: u8,
    pub min_chunk_size: u32,
    pub chunk_size: u32,
    pub max_chunk_size: u32,
}

impl From<Param<'_>> for ParamEntry {
    fn from(param: Param) -> Self {
        let unk0 = param.unk0();
        let chunking_version = param.chunking_version();
        let min_chunk_size = param.min_chunk_size();
        let chunk_size = param.chunk_size();
        let max_chunk_size = param.max_chunk_size();

        Self {
            unk0,
            chunking_version,
            min_chunk_size,
            chunk_size,
            max_chunk_size,
        }
    }
}
