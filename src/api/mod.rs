//! A tiny, Rust-native API to the embedded SQLite library
#![cfg(feature = "api")]

pub mod answer;
pub mod ffiext;
pub mod query;
pub mod row;
pub mod sqlite;
pub mod types;
