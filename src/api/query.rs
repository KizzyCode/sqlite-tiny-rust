//! An SQLite query

use crate::api::answer::Answer;
use crate::api::ffiext::{self, PointerMut};
use crate::api::types::SqliteType;
use crate::error::Error;
use crate::{err, ffi, Sqlite};

/// An SQLite query
#[derive(Debug)]
pub struct Query<'db> {
    /// The database
    pub(in crate::api) sqlite: &'db Sqlite,
    /// The statement
    pub(in crate::api) raw: PointerMut<ffi::sqlite3_stmt>,
}
impl<'db> Query<'db> {
    /// Binds a value
    ///
    /// # Important
    /// Sadly, unless manually specified with `?NNN`, default column indices for binding start with `1` 😭
    pub fn bind<T>(self, column: std::ffi::c_int, value: T) -> Result<Self, Error>
    where
        SqliteType: TryFrom<T>,
        <SqliteType as TryFrom<T>>::Error: std::error::Error + Send + 'static,
    {
        // Create intermediate value and bind it
        let value =
            SqliteType::try_from(value).map_err(|e| err!(with: e, "Failed to convert value into SQLite type"))?;
        match value {
            SqliteType::Null => self.bind_null(column)?,
            SqliteType::Integer(value) => self.bind_integer(column, value)?,
            SqliteType::Real(value) => self.bind_real(column, value)?,
            SqliteType::Text(value) => self.bind_text(column, value)?,
            SqliteType::Blob(value) => self.bind_blob(column, value)?,
        }
        Ok(self)
    }
    /// Binds a NULL value
    fn bind_null(&self, column: std::ffi::c_int) -> Result<(), Error> {
        let retval = unsafe { ffi::sqlite3_bind_null(self.raw.as_ptr(), column) };
        unsafe { ffiext::sqlite3_check_result(retval, self.sqlite.raw.as_ptr()) }
    }
    /// Binds an INTEGER value
    fn bind_integer(&self, column: std::ffi::c_int, value: i64) -> Result<(), Error> {
        let retval = unsafe { ffi::sqlite3_bind_int64(self.raw.as_ptr(), column, value) };
        unsafe { ffiext::sqlite3_check_result(retval, self.sqlite.raw.as_ptr()) }
    }
    /// Binds a REAL value
    fn bind_real(&self, column: std::ffi::c_int, value: f64) -> Result<(), Error> {
        let retval = unsafe { ffi::sqlite3_bind_double(self.raw.as_ptr(), column, value) };
        unsafe { ffiext::sqlite3_check_result(retval, self.sqlite.raw.as_ptr()) }
    }
    /// Binds a TEXT value
    fn bind_text(&self, column: std::ffi::c_int, value: String) -> Result<(), Error> {
        let retval = unsafe {
            // Bind the text value and instruct SQLite to immediately copy the value
            ffi::sqlite3_bind_text64(
                self.raw.as_ptr(),
                column,
                value.as_ptr() as _,
                value.len() as _,
                ffi::sqlite3_transient(),
                ffi::SQLITE_UTF8 as _,
            )
        };
        unsafe { ffiext::sqlite3_check_result(retval, self.sqlite.raw.as_ptr()) }
    }
    /// Binds a BLOB value
    fn bind_blob(&self, column: std::ffi::c_int, value: Vec<u8>) -> Result<(), Error> {
        let retval = unsafe {
            // Bind the blob value and instruct SQLite to immediately copy the value
            ffi::sqlite3_bind_blob64(
                self.raw.as_ptr(),
                column,
                value.as_ptr() as _,
                value.len() as _,
                ffi::sqlite3_transient(),
            )
        };
        unsafe { ffiext::sqlite3_check_result(retval, self.sqlite.raw.as_ptr()) }
    }

    /// Executes the query and gets the next result row if any
    pub fn execute(self) -> Result<Answer<'db>, Error> {
        // Create the result object and do a step to make sure the query is actually executed
        let mut result = Answer { sqlite: self.sqlite, raw: self.raw, has_row: false };
        result.step()?;

        // Initialize the result struct
        Ok(result)
    }
}
