//! An SQLite query

use crate::{
    api::{row::Row, types::SqliteType},
    err,
    error::Error,
    ffi, Sqlite,
};
use std::ffi::c_int;

/// An SQLite query
#[derive(Debug)]
pub struct Query<'a> {
    /// The database
    pub(in crate::api) sqlite: &'a Sqlite,
    /// The statement
    pub(in crate::api) raw: *mut ffi::sqlite3_stmt,
}
impl<'a> Query<'a> {
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
        let retval = unsafe { ffi::sqlite3_bind_null(self.raw, column) };
        unsafe { ffi::sqlite3_check_result(retval, self.sqlite.raw) }
    }
    /// Binds an INTEGER value
    fn bind_integer(&self, column: std::ffi::c_int, value: i64) -> Result<(), Error> {
        let retval = unsafe { ffi::sqlite3_bind_int64(self.raw, column, value) };
        unsafe { ffi::sqlite3_check_result(retval, self.sqlite.raw) }
    }
    /// Binds a REAL value
    fn bind_real(&self, column: std::ffi::c_int, value: f64) -> Result<(), Error> {
        let retval = unsafe { ffi::sqlite3_bind_double(self.raw, column, value) };
        unsafe { ffi::sqlite3_check_result(retval, self.sqlite.raw) }
    }
    /// Binds a TEXT value
    fn bind_text(&self, column: std::ffi::c_int, value: String) -> Result<(), Error> {
        let retval = unsafe {
            // Bind the text value and instruct SQLite to immediately copy the value
            ffi::sqlite3_bind_text64(
                self.raw,
                column,
                value.as_ptr() as _,
                value.len() as _,
                ffi::sqlite3_transient(),
                ffi::SQLITE_UTF8 as _,
            )
        };
        unsafe { ffi::sqlite3_check_result(retval, self.sqlite.raw) }
    }
    /// Binds a BLOB value
    fn bind_blob(&self, column: std::ffi::c_int, value: Vec<u8>) -> Result<(), Error> {
        let retval = unsafe {
            // Bind the blob value and instruct SQLite to immediately copy the value
            ffi::sqlite3_bind_blob64(self.raw, column, value.as_ptr() as _, value.len() as _, ffi::sqlite3_transient())
        };
        unsafe { ffi::sqlite3_check_result(retval, self.sqlite.raw) }
    }

    /// Executes the query and gets the next result row if any
    #[allow(clippy::missing_panics_doc)]
    pub fn execute(self) -> Result<QueryResult<'a>, Error> {
        // Create a row view over the current query and do the first `step` to execute it
        let row = Row { query: self };
        let row_state = row.step()?;

        // Initialize the result struct
        Ok(QueryResult { row, row_preflighted: true, row_state })
    }
}

/// A query result
#[derive(Debug)]
pub struct QueryResult<'a> {
    /// A row view over the underlying statement
    pub(in crate::api) row: Row<'a>,
    /// Whether the next row has already been preflighted or needs to be fetched
    pub(in crate::api) row_preflighted: bool,
    /// A slot to hold the result state of the associated row view
    pub(in crate::api) row_state: c_int,
}
impl<'a> QueryResult<'a> {
    /// Gets the current pending result row or returns an error if there is no row
    pub fn row(self) -> Result<Row<'a>, Error> {
        match self.row_state {
            ffi::SQLITE_ROW => Ok(self.row),
            _ => Err(err!("No result row available")),
        }
    }

    /// Returns the next row like a fallible iterator
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<Option<&Row<'a>>, Error> {
        // Fetch the next row if necessary
        match self.row_preflighted {
            true => self.row_preflighted = false,
            false => self.row_state = self.row.step()?,
        }

        // Validate the current state
        match self.row_state {
            ffi::SQLITE_ROW => Ok(Some(&self.row)),
            _ => Ok(None),
        }
    }
}
