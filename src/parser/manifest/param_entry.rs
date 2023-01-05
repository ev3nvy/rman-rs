use std::io::Error;

use crate::generated::rman::Param;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ParamEntry {
    pub unk0: u16,
    pub hash_type: u8,
    pub unk2: u32,
    pub unk3: u32,
    pub max_uncompressed: u32,
}

impl TryFrom<Param<'_>> for ParamEntry {
    type Error = Error;

    fn try_from(param: Param) -> Result<Self, Self::Error> {
        let unk0 = param.unk0();
        let hash_type = param.hash_type();
        let unk2 = param.unk2();
        let unk3 = param.unk3();
        let max_uncompressed = param.max_uncompressed();

        Ok(Self {
            unk0,
            hash_type,
            unk2,
            unk3,
            max_uncompressed,
        })
    }
}
