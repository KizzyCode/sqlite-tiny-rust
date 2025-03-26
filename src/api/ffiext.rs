//! FFI types and helpers

use crate::ffi;
use std::ffi::c_int;

/// Gets the last error from the database as [`crate::error::Error`]
///
/// # Safety
/// This function operates on a raw SQLite handle. If `database` is not `NULL` but invalid or points to an invalid
/// handle, the behaviour is undefined.
#[doc(hidden)]
pub unsafe fn sqlite3_last_error(retval: i32, database: *mut ffi::sqlite3) -> crate::error::Error {
    use std::borrow::Cow;
    use std::ffi::CStr;

    // Get the error string
    let error = ffi::sqlite3_errstr(retval);
    let mut message = match error.is_null() {
        true => Cow::Borrowed("Unknown"),
        false => CStr::from_ptr(error).to_string_lossy(),
    };

    // Append database specific error
    if !database.is_null() {
        // Get error from database
        let error = ffi::sqlite3_errmsg(database);
        let message_ = CStr::from_ptr(error).to_string_lossy();
        message = Cow::Owned(format!("{message} ({message_})"));
    }
    crate::err!("SQLite error: {message}")
}

/// Helper to translate a result code into a `Result`
///
/// # Safety
/// This function operates on a raw SQLite handle. If `database` is not `NULL` but invalid or points to an invalid
/// handle, the behaviour is undefined.
#[doc(hidden)]
#[inline]
pub unsafe fn sqlite3_check_result(retval: i32, database: *mut ffi::sqlite3) -> Result<(), crate::error::Error> {
    match retval {
        ffi::SQLITE_OK => Ok(()),
        _ => Err(sqlite3_last_error(retval, database)),
    }
}

/// An "owned", mutable pointer
#[derive(Debug)]
pub struct PointerMut<T> {
    /// The underlying raw pointer
    ptr: *mut T,
    /// An optional callback that is called on drop
    on_drop: unsafe extern "C" fn(*mut T) -> c_int,
}
impl<T> PointerMut<T> {
    /// Creates a new owned pointer
    ///
    /// # Panics
    /// This function panics if the given pointer is `NULL`.
    pub fn new(ptr: *mut T, on_drop: unsafe extern "C" fn(*mut T) -> c_int) -> Self {
        assert!(!ptr.is_null(), "cannot create an owned NULL pointer");
        Self { ptr, on_drop }
    }

    /// Returns the underlying pointer
    pub const fn as_ptr(&self) -> *mut T {
        self.ptr
    }
}
impl<T> Drop for PointerMut<T> {
    fn drop(&mut self) {
        // Call the on-drop callback
        unsafe { (self.on_drop)(self.ptr) };
    }
}

/// A pointer with flexible ownership
#[derive(Debug)]
pub enum PointerMutFlex<'a, T> {
    /// A borrowed pointer
    Borrowed(&'a mut PointerMut<T>),
    /// An owned pointer
    Owned(PointerMut<T>),
}
impl<T> PointerMutFlex<'_, T> {
    /// Returns the underlying pointer
    pub const fn as_ptr(&self) -> *mut T {
        match self {
            PointerMutFlex::Borrowed(pointer_ref) => pointer_ref.as_ptr(),
            PointerMutFlex::Owned(pointer_mut) => pointer_mut.as_ptr(),
        }
    }
}
