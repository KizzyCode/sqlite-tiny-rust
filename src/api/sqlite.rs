//! An SQLite database handle

use super::ffiext;
use crate::api::ffiext::PointerMut;
use crate::api::query::Query;
use crate::error::Error;
use crate::{err, ffi};
use std::ffi::CString;
use std::ptr;

/// An SQLite database handle
#[derive(Debug)]
pub struct Sqlite {
    /// The database handle
    pub(in crate::api) raw: PointerMut<ffi::sqlite3>,
}
impl Sqlite {
    /// Opens or creates an SQLite 3 database for reading and writing
    pub fn new(path: &str) -> Result<Self, Error> {
        Self::raw(path, ffi::SQLITE_OPEN_READWRITE | ffi::SQLITE_OPEN_CREATE)
    }
    /// Opens an SQLite database from an URL (see <https://www.sqlite.org/uri.html>)
    pub fn uri(uri: &str) -> Result<Self, Error> {
        Self::raw(uri, ffi::SQLITE_OPEN_READWRITE | ffi::SQLITE_OPEN_URI)
    }
    /// Opens an SQLite database at the given location with the given flags
    ///
    /// # Important
    /// To ensure the safety guarantees of the Rust API are not violated, we always add `SQLITE_OPEN_FULLMUTEX` to the
    /// provided flags.
    pub fn raw(location: &str, flags: std::ffi::c_int) -> Result<Self, Error> {
        // Prepare path and database pointer
        let path = CString::new(location).map_err(|e| err!(with: e, "Invalid database location"))?;
        let mut database = ptr::null_mut();

        // Open the database
        let flags = flags | ffi::SQLITE_OPEN_FULLMUTEX;
        let retval = unsafe { ffi::sqlite3_open_v2(path.as_ptr(), &mut database, flags, ptr::null()) };
        unsafe { ffiext::sqlite3_check_result(retval, ptr::null_mut()) }?;

        // Init self
        let database = PointerMut::new(database, ffi::sqlite3_close_v2);
        Ok(Self { raw: database })
    }

    /// Creates a new query from a **single** SQL statement
    pub fn query(&self, query: &str) -> Result<Query, Error> {
        // Prepare query and statement pointer
        let query = CString::new(query).map_err(|e| err!(with: e, "Invalid database query"))?;
        let mut statement = ptr::null_mut();

        // Prepare statement and check result code
        let retval =
            unsafe { ffi::sqlite3_prepare_v2(self.raw.as_ptr(), query.as_ptr(), -1, &mut statement, ptr::null_mut()) };
        unsafe { ffiext::sqlite3_check_result(retval, self.raw.as_ptr()) }?;

        // Init query
        let statement = PointerMut::new(statement, ffi::sqlite3_finalize);
        Ok(Query { sqlite: self, raw: statement })
    }

    /// Executes one or more SQL queries
    pub fn execute(&self, query: &str) -> Result<(), Error> {
        // Prepare query and statement pointer
        let query = CString::new(query).map_err(|e| err!(with: e, "Invalid database query"))?;
        let retval =
            unsafe { ffi::sqlite3_exec(self.raw.as_ptr(), query.as_ptr(), None, ptr::null_mut(), ptr::null_mut()) };
        unsafe { ffiext::sqlite3_check_result(retval, self.raw.as_ptr()) }?;

        // Apparently, the query was successful
        Ok(())
    }
}
unsafe impl Send for Sqlite {
    // This struct is safely send because:
    //  - the underlying database is sync because SQLite guarantees so if the database is opened with
    //    `SQLITE_OPEN_FULLMUTEX`
    //  - the pointer itself is never mutated, and dropping is safe because this struct is no-`Copy`/no-`Clone`, so it
    //    can only be dropped once and access in the destructor is exclusive
}
unsafe impl Sync for Sqlite {
    // This struct is safely sync because:
    //  - the underlying database is sync because SQLite guarantees so if the database is opened with
    //    `SQLITE_OPEN_FULLMUTEX`
    //  - the pointer itself is never mutated, and dropping is safe because this struct is no-`Copy`/no-`Clone`, so it
    //    can only be dropped once and access in the destructor is exclusive
}
