use std::io::Error;

use crate::generated::rman::Language;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LanguageEntry {
    pub id: u8,
    pub name: String,
}

impl TryFrom<Language<'_>> for LanguageEntry {
    type Error = Error;

    fn try_from(language: Language) -> Result<Self, Self::Error> {
        let id = language.id();
        let name = language.name().unwrap_or_default().to_string();

        Ok(Self { id, name })
    }
}
