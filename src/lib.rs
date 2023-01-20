pub mod error;
mod generated;
mod parser;

pub use parser::{File, FileHeader, Manifest, ManifestFile};

pub mod entries {
    pub use crate::parser::manifest::{
        BundleEntry, DirectoryEntry, FileEntry, KeyEntry, LanguageEntry, ParamEntry,
    };
}
