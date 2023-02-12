//! Collection of entries from the parsed flatbuffer schema.
//!
//! NOTE: these structs are only useful for viewing the internal structure of the parsed
//! flatbuffer schema. If you just wish to see the containing files and download them, see
//! [File][crate::File].

mod bundle_entry;
mod chunk_entry;
mod directory_entry;
mod file_entry;
mod key_entry;
mod language_entry;
mod param_entry;

pub use self::bundle_entry::BundleEntry;
pub use self::chunk_entry::ChunkEntry;
pub use self::directory_entry::DirectoryEntry;
pub use self::file_entry::FileEntry;
pub use self::key_entry::KeyEntry;
pub use self::language_entry::LanguageEntry;
pub use self::param_entry::ParamEntry;
