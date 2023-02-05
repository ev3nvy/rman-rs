use crate::generated::rman::Language;

/// Single language entry object.
///
/// This is identical to the schema in [rman-schema][rman-schema] and exists to provide a
/// persistent structure for the LanguageEntry.
///
/// [rman-schema]: https://github.com/ev3nvy/rman-schema
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LanguageEntry {
    /// Id of the language entry.
    pub id: u8,
    /// Name of the language in the language-region variant of the [RFC 5646 standard][rfc-5646],
    /// however, underscores are used instead of hyphens.
    ///
    /// There are also non languages present.
    /// A non-exhaustive list of values that are not languagues:
    /// - `krrating`
    /// - `mature`
    /// - `twmlogo`
    /// - `vnglogo`
    /// - `all_loc`
    ///
    /// [rfc-5646]: https://www.rfc-editor.org/rfc/rfc5646.html
    pub name: String,
}

impl From<Language<'_>> for LanguageEntry {
    fn from(language: Language) -> Self {
        let id = language.id();
        let name = language.name().unwrap_or_default().to_owned();

        Self { id, name }
    }
}
