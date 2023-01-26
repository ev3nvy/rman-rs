// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]

pub mod entries;
pub mod error;
mod generated;
mod parser;

pub use parser::{File, FileHeader, Manifest, ManifestFile};
