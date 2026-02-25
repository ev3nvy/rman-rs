#![allow(missing_docs)]

#[allow(warnings)]
#[allow(missing_debug_implementations, missing_copy_implementations)]
#[allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#[rustfmt::skip]
mod schema_v2_0_generated;
#[allow(warnings)]
#[allow(missing_debug_implementations, missing_copy_implementations)]
#[allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#[rustfmt::skip]
mod schema_v2_1_generated;

pub use schema_v2_0_generated::rman as rman_v2_0;
pub use schema_v2_1_generated::rman as rman_v2_1;
