use crate::generated::rman::Key;

/// Single key entry object.
///
/// This is identical to the schema in [rman-schema][rman-schema] and exists to provide a
/// persistent structure for the `KeyEntry`.
///
/// [rman-schema]: https://github.com/ev3nvy/rman-schema
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KeyEntry {
    /// Field with an unknown function and type (it might also be an [`i16`]).
    pub unk0: u16,
    /// Field with an unknown function and type (it might also be an [`i32`]).
    pub unk1: u32,
}

impl From<Key<'_>> for KeyEntry {
    fn from(chunk: Key) -> Self {
        let unk0 = chunk.unk0();
        let unk1 = chunk.unk1();

        Self { unk0, unk1 }
    }
}
