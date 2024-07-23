//! FFI bindings to the shipped sqlite variant

#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

include!("bindgen.rs");

// Glue bindings
extern "C" {
    /// Internal helper to get the pointer constant to define "transient" ownership (i.e. order SQLite to copy the value
    /// immediately)
    //sqlite3_destructor_type sqlite3_transient()
    #[doc(hidden)]
    pub fn sqlite3_transient() -> sqlite3_destructor_type;
}

/// Helper to translate a result code into a `Result`
///
/// # Safety
/// This function operates on a raw SQLite handle. If `database` is not `NULL` but invalid or points to an invalid handle,
/// the behaviour is undefined.
#[doc(hidden)]
pub unsafe fn sqlite3_check_result(retval: i32, database: *mut sqlite3) -> Result<(), crate::error::Error> {
    use std::{borrow::Cow, ffi::CStr};

    // Early-return on success
    if retval == SQLITE_OK {
        return Ok(());
    }

    // Get the error string
    let error = sqlite3_errstr(retval);
    let mut message = match error.is_null() {
        true => Cow::Borrowed("Unknown"),
        false => CStr::from_ptr(error).to_string_lossy(),
    };

    // Append database specific error
    if !database.is_null() {
        // Get error from database
        let error = sqlite3_errmsg(database);
        let message_ = CStr::from_ptr(error).to_string_lossy();
        message = Cow::Owned(format!("{message} ({message_})"));
    }
    Err(crate::err!("SQLite error: {message}"))
}

/// Asserts that sqlite is compiled threadsafe
#[test]
fn assert_threadsafe() {
    let threadsafe = unsafe { sqlite3_threadsafe() };
    assert_ne!(threadsafe, 0, "sqlite is not compiled threadsafe?!")
}
