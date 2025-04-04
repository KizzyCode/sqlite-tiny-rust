//! FFI bindings to the shipped sqlite variant

#![allow(unused, reason = "Includes autogenerated bindings")]
#![allow(non_snake_case, reason = "Includes autogenerated bindings")]
#![allow(non_camel_case_types, reason = "Includes autogenerated bindings")]

include!("bindgen.rs");

// Glue bindings
unsafe extern "C" {
    /// Internal helper to get the pointer constant to define "transient" ownership (i.e. order SQLite to copy the value
    /// immediately)
    //sqlite3_destructor_type sqlite3_transient()
    pub unsafe fn sqlite3_transient() -> sqlite3_destructor_type;
}

/// Asserts that sqlite is compiled threadsafe
#[test]
fn assert_threadsafe() {
    let threadsafe = unsafe { sqlite3_threadsafe() };
    assert_ne!(threadsafe, 0, "sqlite is not compiled threadsafe?!")
}
