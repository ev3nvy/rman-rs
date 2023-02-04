use crate::generated::rman::Language;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LanguageEntry {
    pub id: u8,
    pub name: String,
}

impl From<Language<'_>> for LanguageEntry {
    fn from(language: Language) -> Self {
        let id = language.id();
        let name = language.name().unwrap_or_default().to_string();

        Self { id, name }
    }
}
