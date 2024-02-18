//! An SQLite result row

use crate::{
    api::{statement::Statement, types::SqliteType},
    error,
    error::Error,
    ffi,
};
use std::ffi::CStr;

/// A statement result row
#[derive(Debug)]
pub struct Row<'a, 'b> {
    /// The underlying statement
    ///
    /// # Note
    /// We reference the statement as `&mut` here to ensure it is not modified while we access the current row
    pub(in crate::api) statement: &'a mut Statement<'b>,
}
impl Row<'_, '_> {
    /// Reads the value for the requested column
    ///
    /// # Note
    /// Column indices for reading start with `0`
    pub fn read<T>(&self, column: std::ffi::c_int) -> Result<T, Error>
    where
        SqliteType: TryInto<T>,
        <SqliteType as TryInto<T>>::Error: std::error::Error + Send + 'static,
    {
        // Get the type and read the value as said type
        let type_ = unsafe { ffi::sqlite3_column_type(self.statement.raw, column) };
        let value = match type_ {
            ffi::SQLITE_NULL => SqliteType::Null,
            ffi::SQLITE_INTEGER => self.read_integer(column)?,
            ffi::SQLITE_FLOAT => self.read_real(column)?,
            ffi::SQLITE_TEXT => self.read_text(column)?,
            ffi::SQLITE_BLOB => self.read_blob(column)?,
            _ => return Err(error!("Unknown SQLite column type: {type_}")),
        };

        // Convert value into requested type
        value.try_into().map_err(|e| error!(with: e, "Failed to load from SQLite type"))
    }
    /// Reads an INTEGER value from the given column
    fn read_integer(&self, column: std::ffi::c_int) -> Result<SqliteType, Error> {
        let value = unsafe { ffi::sqlite3_column_int64(self.statement.raw, column) };
        Ok(SqliteType::Integer(value))
    }
    /// Reads a REAL value from the given column
    fn read_real(&self, column: std::ffi::c_int) -> Result<SqliteType, Error> {
        let value = unsafe { ffi::sqlite3_column_double(self.statement.raw, column) };
        Ok(SqliteType::Real(value))
    }
    /// Reads a TEXT value from the given column
    fn read_text(&self, column: std::ffi::c_int) -> Result<SqliteType, Error> {
        // Get text value
        let chars = unsafe { ffi::sqlite3_column_text(self.statement.raw, column) };
        let text = unsafe { CStr::from_ptr(chars as _) };

        // Get rust string
        let text = text.to_str().map_err(|e| error!(with: e, "SQLite string is not valid UTF-8"))?;
        Ok(SqliteType::Text(text.to_string()))
    }
    /// Reads a BLOB value from the given column
    fn read_blob(&self, column: std::ffi::c_int) -> Result<SqliteType, Error> {
        // Get blob value
        let data = unsafe { ffi::sqlite3_column_blob(self.statement.raw, column) };
        let false = data.is_null() else {
            // SQLite has a "special" way of handling empty blobs
            return Ok(SqliteType::Blob(Vec::new()));
        };

        // Get blob length and copy bytes
        let len = unsafe { ffi::sqlite3_column_bytes(self.statement.raw, column) };
        let bytes = unsafe { std::slice::from_raw_parts(data as *const u8, len as usize) };
        Ok(SqliteType::Blob(bytes.to_vec()))
    }
}
