use crate::generated::rman::Tag;

/// Single tag entry object.
///
/// This is identical to the schema in [rman-schema][rman-schema] and exists to provide a
/// persistent structure for the `TagEntry`.
///
/// [rman-schema]: https://github.com/ev3nvy/rman-schema
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TagEntry {
    /// Id of the tag entry.
    pub id: u8,
    /// Name of the tag entry.
    ///
    /// A non-exhaustive list of values:
    /// - language in the language-region variant of the [RFC 5646 standard][rfc-5646] but with
    ///   underscores instead of hyphens,
    /// - `krrating`,
    /// - `mature`,
    /// - `twmlogo`,
    /// - `vnglogo`,
    /// - `all_loc`.
    ///
    /// [rfc-5646]: https://www.rfc-editor.org/rfc/rfc5646.html
    pub name: String,
}

impl From<Tag<'_>> for TagEntry {
    fn from(tag: Tag) -> Self {
        let id = tag.id();
        let name = tag.name().unwrap_or_default().to_owned();

        Self { id, name }
    }
}
