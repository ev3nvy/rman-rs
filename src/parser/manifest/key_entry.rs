use std::io::Error;

use crate::generated::rman::Key;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KeyEntry {
    pub unk0: u16,
    pub unk1: u32,
}

impl TryFrom<Key<'_>> for KeyEntry {
    type Error = Error;

    fn try_from(chunk: Key) -> Result<Self, Self::Error> {
        let unk0 = chunk.unk0();
        let unk1 = chunk.unk1();

        Ok(Self { unk0, unk1 })
    }
}
