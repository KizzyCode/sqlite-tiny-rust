[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/sqlite-tiny-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/sqlite-tiny-rust)
[![docs.rs](https://docs.rs/sqlite-tiny/badge.svg)](https://docs.rs/sqlite-tiny)
[![crates.io](https://img.shields.io/crates/v/sqlite-tiny.svg)](https://crates.io/crates/sqlite-tiny)
[![Download numbers](https://img.shields.io/crates/d/sqlite-tiny.svg)](https://crates.io/crates/sqlite-tiny)
[![dependency status](https://deps.rs/crate/sqlite-tiny/latest/status.svg)](https://deps.rs/crate/sqlite-tiny)


# `sqlite-tiny`
Welcome to `sqlite-tiny` 🎉

This crate is minimalistic SQLite library crate which ships the amalgamation variant and provides a tiny Rust API. If
you just want the embedded SQLite library plus the generated C bindings, you can disable the `api`-feature (enabled by
default).

## Performance Considerations
For the sake of simplicity, this crate operates under the following assumption: `malloc` is cheap. To keep the code
clean and readable, we are quite liberal with allocating memory and copying data to avoid overly complex life-time
juggling.

Some locations where we do this are (non-exhaustive):
- Binding values: Since some values require a temporary intermediate representation before they can be bound, and
  statements should be able to outlive a passed argument, we instruct SQLite to copy the values into an internal buffer
- Reading values: To avoid lifetime troubles, we always copy a value from a row/column out of the SQLite context into 
  Rust-managed memory immediately on access

## Distributed SQLite Version
For simplicity, this crate does not link to external SQLite versions, but exclusively builds and embeds the amalgamation
in the [`dist`-folder](dist/). For more information see [`dist/README.md`](dist/README.md).
