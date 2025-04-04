//! An SQLite query result row

use crate::api::ffiext::PointerMutFlex;
use crate::api::types::SqliteType;
use crate::error::Error;
use crate::{err, ffi};
use std::ffi::CStr;

/// An SQLite result row
#[derive(Debug)]
pub struct Row<'stmt> {
    /// The statement
    pub(in crate::api) raw: PointerMutFlex<'stmt, ffi::sqlite3_stmt>,
}
impl Row<'_> {
    /// The amount of fields/columns in the current row
    #[allow(clippy::missing_panics_doc, reason = "Panic should never occur during normal operation")]
    pub fn len(&self) -> usize {
        let columns = unsafe { ffi::sqlite3_data_count(self.raw.as_ptr()) };
        // Note: If the amount of columns is greater than `usize::MAX` or an `core::ffi::c_int` is greater than
        //  `usize::MAX`, something is super weird here and we want to panic
        #[allow(clippy::expect_used, reason = "Panic should never occur during normal operation")]
        usize::try_from(columns).expect("amount of columns is greater than `usize::MAX`")
    }
    /// Whether the current row contains some fields/columns or not
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Reads the value for the requested column from the current row
    ///
    /// # Note
    /// Column indices for reading start with `0`
    pub fn read<T>(&self, column: std::ffi::c_int) -> Result<T, Error>
    where
        SqliteType: TryInto<T>,
        <SqliteType as TryInto<T>>::Error: std::error::Error + Send + 'static,
    {
        // Get the type and read the value as said type
        let type_ = unsafe { ffi::sqlite3_column_type(self.raw.as_ptr(), column) };
        let value = match type_ {
            ffi::SQLITE_NULL => SqliteType::Null,
            ffi::SQLITE_INTEGER => self.read_integer(column)?,
            ffi::SQLITE_FLOAT => self.read_real(column)?,
            ffi::SQLITE_TEXT => self.read_text(column)?,
            ffi::SQLITE_BLOB => self.read_blob(column)?,
            _ => return Err(err!("Unknown SQLite column type: {type_}")),
        };

        // Convert value into requested type
        value.try_into().map_err(|e| err!(with: e, "Failed to load from SQLite type"))
    }
    /// Reads an INTEGER value from the given column
    fn read_integer(&self, column: std::ffi::c_int) -> Result<SqliteType, Error> {
        let value = unsafe { ffi::sqlite3_column_int64(self.raw.as_ptr(), column) };
        Ok(SqliteType::Integer(value))
    }
    /// Reads a REAL value from the given column
    fn read_real(&self, column: std::ffi::c_int) -> Result<SqliteType, Error> {
        let value = unsafe { ffi::sqlite3_column_double(self.raw.as_ptr(), column) };
        Ok(SqliteType::Real(value))
    }
    /// Reads a TEXT value from the given column
    fn read_text(&self, column: std::ffi::c_int) -> Result<SqliteType, Error> {
        // Get text value
        let chars = unsafe { ffi::sqlite3_column_text(self.raw.as_ptr(), column) };
        let text = unsafe { CStr::from_ptr(chars as _) };

        // Get rust string
        let text = text.to_str().map_err(|e| err!(with: e, "SQLite string is not valid UTF-8"))?;
        Ok(SqliteType::Text(text.to_string()))
    }
    /// Reads a BLOB value from the given column
    fn read_blob(&self, column: std::ffi::c_int) -> Result<SqliteType, Error> {
        // Get blob value
        let data = unsafe { ffi::sqlite3_column_blob(self.raw.as_ptr(), column) };
        let false = data.is_null() else {
            // SQLite has a "special" way of handling empty blobs
            return Ok(SqliteType::Blob(Vec::new()));
        };

        // Get blob length and copy bytes
        let len = unsafe { ffi::sqlite3_column_bytes(self.raw.as_ptr(), column) };
        let bytes = unsafe { std::slice::from_raw_parts(data as *const u8, len as usize) };
        Ok(SqliteType::Blob(bytes.to_vec()))
    }
}
