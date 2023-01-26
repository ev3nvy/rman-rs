// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]

pub mod entries;
pub mod error;
mod file;
mod parser;

mod generated {
    #![allow(clippy::all, clippy::pedantic, clippy::nursery)]
    #![allow(missing_debug_implementations, unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/schema_generated.rs"));
}

pub use crate::file::File;
pub use crate::parser::header::Header;
pub use crate::parser::manifest::ManifestData;
pub use crate::parser::RiotManifest;
