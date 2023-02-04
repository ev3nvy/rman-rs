use crate::generated::rman::Key;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KeyEntry {
    pub unk0: u16,
    pub unk1: u32,
}

impl From<Key<'_>> for KeyEntry {
    fn from(chunk: Key) -> Self {
        let unk0 = chunk.unk0();
        let unk1 = chunk.unk1();

        Self { unk0, unk1 }
    }
}
