//! An SQLite query result

use crate::api::ffiext::{self, PointerMut, PointerMutFlex};
use crate::api::row::Row;
use crate::error::Error;
use crate::{err, ffi, Sqlite};

/// A query result
#[derive(Debug)]
pub struct Answer<'db> {
    /// The database
    pub(in crate::api) sqlite: &'db Sqlite,
    /// The statement
    pub(in crate::api) raw: PointerMut<ffi::sqlite3_stmt>,
    /// If we already have a fetched row pending
    pub(in crate::api) has_row: bool,
}
impl Answer<'_> {
    /// Gets the current pending result row or returns an error if there is no row
    pub fn row(mut self) -> Result<Row<'static>, Error> {
        // Try to fetch the next row if necessary
        if !self.has_row {
            // Do a step to get the next row if any
            self.step()?;
        }

        // Check if we have a row now
        if !self.has_row {
            // We still don't have a row
            return Err(err!("No result row available"));
        }

        // Return an borrowed row
        let row = Row { raw: PointerMutFlex::Owned(self.raw) };
        Ok(row)
    }

    /// Returns the next row like a fallible iterator
    pub fn next_row(&mut self) -> Result<Option<Row>, Error> {
        // Try to fetch the next row if necessary
        if !self.has_row {
            // Do a step to get the next row if any
            self.step()?;
        }

        // Check if we have a row now
        if !self.has_row {
            // We still don't have a row
            return Ok(None);
        }

        // Return an borrowed row
        let row = Row { raw: PointerMutFlex::Borrowed(&mut self.raw) };
        self.has_row = false;
        Ok(Some(row))
    }

    /// Advances the underlying statement towards the first or subsequent row
    pub(in crate::api) fn step(&mut self) -> Result<(), Error> {
        // Do a step
        let retval = unsafe { ffi::sqlite3_step(self.raw.as_ptr()) };
        let true = matches!(retval, ffi::SQLITE_ROW | ffi::SQLITE_DONE) else {
            // Failed while trying to get the next row
            return Err(unsafe { ffiext::sqlite3_last_error(retval, self.sqlite.raw.as_ptr()) });
        };

        // Mark if we have a pending row
        self.has_row = retval == ffi::SQLITE_ROW;
        Ok(())
    }
}
