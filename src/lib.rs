#![doc = include_str!("../README.md")]
// Clippy lints
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::expect_used)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::panic)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unreachable)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::cognitive_complexity)]

pub mod api;
pub mod error;
pub mod ffi;

#[cfg(feature = "api")]
pub use api::sqlite::Sqlite;

/// Returns the semver tuple for the distributed sqlite version as `(major, minor, patch)`-tuple
pub fn version() -> (i32, i32, i32) {
    // Get version from lib
    let version = unsafe { ffi::sqlite3_libversion_number() };

    // Split version integer
    let major = (version / 1_000_000) % 1000;
    let minor = (version / 1_000) % 1000;
    let patch = version % 1000;
    (major, minor, patch)
}
