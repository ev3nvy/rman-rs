#![deny(missing_docs)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![allow(clippy::module_name_repetitions, clippy::similar_names)]

//! # rman
//!
//! The `rman` crate provides a convenient api for parsing and downloading
//! [manifest files][manifest]. This format is specific to games made by [Riot Games][riot-games].
//!
//! Crates name is derived from the 4 magic bytes at the beginning of the `.manifest` file,
//! which correspond to `R`, `M`, `A`, `N` ascii characters. `RMAN` probably stands for
//! Riot Manifest (or a variation of the two words).
//!
//! # Usage
//!
//! This crate is on [crates.io][rman-crates-io]. To use it, just add the dependency `rman` to the
//! `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! rman = "0.1"
//! ```
//!
//! # Example: parsing a manifest file from path
//!
//! Easiest way to parse a manifest file is to just provide a path to it's location.
//!
//! [`from_path`][crate::RiotManifest::from_path] calls [`File::open`][std::fs::File::open]
//! internally, so everything from [`&str`][str] to [`PathBuf`][std::path::PathBuf] is a valid
//! argument.
//!
//! ```rust
//! # use rman::Result;
//! use rman::RiotManifest;
//!
//! # fn main() -> Result<()> {
//! let path = "file.manifest";
//!   # let path = concat!(env!("OUT_DIR"), "/valid.manifest");
//!
//! let manifest = RiotManifest::from_path(path)?;
//!
//! assert_eq!(manifest.data.files.len(), 1);
//!   # Ok(())
//! # }
//! ```
//!
//! # Example: parsing a manifest file from reader
//!
//! If you read the file from another source, or you already have the file in a reader,
//! you can instead call [`from_reader`](crate::RiotManifest::from_reader) to parse the file.
//!
//! Internally, this is what [`from_path`](crate::RiotManifest::from_path) calls, so the response
//! of the two functions should be identical.
//!
//! ```rust
//! use std::fs;
//! use std::io::BufReader;
//!
//! # use rman::Result;
//! use rman::RiotManifest;
//!
//! # fn main() -> Result<()> {
//! let path = "file.manifest";
//!   # let path = concat!(env!("OUT_DIR"), "/valid.manifest");
//! let file = fs::File::open(path)?;
//! let mut reader = BufReader::new(file);
//!
//! let manifest = RiotManifest::from_reader(&mut reader)?;
//!
//! assert_eq!(manifest.data.files.len(), 1);
//!   # Ok(())
//! # }
//! ```
//!
//! # Example: downloading a file
//!
//! To download a specific file from a parsed manifest, you can invoke the
//! [download function](crate::File::download) on that [file][crate::File].
//!
//! ```rust
//! use std::fs;
//!
//! # use httptest::{matchers::*, responders::*, Expectation, Server};
//! use rman::{Result, RiotManifest};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     # let bundle = fs::read(concat!(env!("OUT_DIR"), "/valid.bundle")).unwrap();
//!     # let server = Server::run();
//!     # server.expect(
//!         # Expectation::matching(request::method_path(
//!             # "GET",
//!             # "/bundles/0000000000000000.bundle",
//!         # ))
//!         # .respond_with(
//!             # status_code(200)
//!                 # .body(bundle)
//!                 # .append_header("Content-Type", "binary/octet-stream")
//!                 # .append_header("Content-Length", 13),
//!         # ),
//!     # );
//!     let path = "file.manifest";
//!     # let path = concat!(env!("OUT_DIR"), "/valid.manifest");
//!     let manifest = RiotManifest::from_path(path)?;
//!
//!     let file_name = "VALORANT.exe";
//!     # let file_name = "file.txt";
//!     let mut file = fs::File::create(file_name)?;
//!     let file_to_download = manifest
//!         .data
//!         .files
//!         .iter()
//!         .find(|f| f.name == file_name)
//!         .expect(format!("file {file_name} does not exist in this manifest").as_str());
//!
//!     let url = "https://valorant.secure.dyn.riotcdn.net/channels/public/bundles";
//!     # let url = server.url("/bundles").to_string();
//!
//!     file_to_download.download(&mut file, url).await?;
//!
//!     assert_eq!(std::fs::read(file_name)?.len(), 4);
//!
//!     # fs::remove_file(file_name)?;
//!     Ok(())
//! }
//! ```
//!
//! # Scope
//!
//! This crate:
//! - reads the `.manifest` file, and verifies it's a valid `rman` file,
//! - decompresses the containing data which was compressed using [zstd][zstd],
//! - parses the decompressed [flatbuffer data][flatbuffers],
//! - stores all of the parsed data on [`ManifestData`][crate::ManifestData],
//! - combines the data into a vector of downloadable [`File`][crate::File]s,
//! - provides a function to [`download`][crate::File::download] specific files.
//!
//! This crate doesn't:
//! - generate a `.manifest` file,
//! - create or parse chunks.
//!
//! # Feature: `default`
//!
//! By default, only [`rustls-tls`](index.html#feature-rustls-tls) is enabled.
//!
//! # Feature: `version_error`
//!
//! If enabled, throws errors on unknown manifest versions, instead of continuing and assuming it
//! works.
//!
//! # Feature: `serde`
//!
//! If enabled, all structs in [`entries`][crate::entries], as well as [`File`][crate::File]
//! will implement [`Serialize`][serde-serialize] and [`Deserialize`][serde-deserialize].
//!
//! # Feature: `native-tls`
//!
//! If enabled, the feature with the same name is enabled for [`reqwest`].
//!
//! # Feature: `rustls-tls`
//!
//! If enabled, the feature with the same name is enabled for [`reqwest`].
//!
//! [flatbuffers]: https://github.com/google/flatbuffers
//! [manifest]: https://technology.riotgames.com/news/supercharging-data-delivery-new-league-patcher
//! [riot-games]: https://www.riotgames.com
//! [rman-crates-io]: https://crates.io/crates/rman
//! [serde-serialize]: https://docs.rs/serde/latest/serde/trait.Serialize.html
//! [serde-deserialize]: https://docs.rs/serde/latest/serde/trait.Deserialize.html
//! [zstd]: https://github.com/facebook/zstd

pub mod entries;
mod error;
mod file;
mod generated;
mod parser;

pub use crate::error::{ManifestError, Result};
pub use crate::file::File;
pub use crate::parser::header::Header;
pub use crate::parser::manifest::ManifestData;
pub use crate::parser::RiotManifest;
