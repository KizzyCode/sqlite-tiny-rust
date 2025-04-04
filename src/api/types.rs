//! Bridge types to help with SQLite data types

use crate::err;
use crate::error::Error;
use std::sync::Arc;

/// An SQLite convertible type
#[derive(Debug, Clone, PartialEq)]
pub enum SqliteType {
    /// NULL
    Null,
    /// INTEGER
    Integer(i64),
    /// REAL
    Real(f64),
    /// TEXT
    Text(String),
    /// BLOB
    Blob(Vec<u8>),
}

// Byte array conversions
impl<const LEN: usize> TryInto<[u8; LEN]> for SqliteType {
    type Error = Error;

    #[rustfmt::skip]
    fn try_into(self) -> Result<[u8; LEN], Self::Error> {
        match self {
            SqliteType::Blob(value) => <[u8; LEN]>::try_from(value)
                .map_err(|_| err!("Failed to convert from SQLite type")),
            _ => Err(err!("Failed to convert from SQLite type")),
        }
    }
}
impl<const LEN: usize> TryInto<Option<[u8; LEN]>> for SqliteType {
    type Error = Error;

    #[rustfmt::skip]
    fn try_into(self) -> Result<Option<[u8; LEN]>, Self::Error> {
        match self {
            Self::Null => Ok(None),
            SqliteType::Blob(value) => <[u8; LEN]>::try_from(value).map(Some)
                .map_err(|_| err!("Failed to convert from SQLite type")),
            _ => Err(err!("Failed to convert from SQLite type")),
        }
    }
}
impl<const LEN: usize> TryFrom<[u8; LEN]> for SqliteType {
    type Error = Error;

    fn try_from(value: [u8; LEN]) -> Result<Self, Self::Error> {
        Ok(SqliteType::Blob(value.into()))
    }
}
impl<const LEN: usize> TryFrom<Option<[u8; LEN]>> for SqliteType {
    type Error = Error;

    fn try_from(value: Option<[u8; LEN]>) -> Result<Self, Self::Error> {
        match value {
            None => Ok(Self::Null),
            Some(value) => Ok(SqliteType::Blob(value.into())),
        }
    }
}

// Generic conversions
macro_rules! impl_sqlitetype_conversion {
    (from: $variant:path => $type:ty) => {
        impl TryInto<$type> for SqliteType {
            type Error = Error;

            fn try_into(self) -> Result<$type, Self::Error> {
                match self {
                    $variant(value) => <$type>::try_from(value)
                        .map_err(|e| err!(with: e, "Failed to convert from SQLite type")),
                    _ => Err(err!("Failed to convert from SQLite type"))
                }
            }
        }
        impl TryInto<Option<$type>> for SqliteType {
            type Error = Error;

            fn try_into(self) -> Result<Option<$type>, Self::Error> {
                match self {
                    Self::Null => Ok(None),
                    $variant(value) => <$type>::try_from(value).map(Some)
                        .map_err(|e| err!(with: e, "Failed to convert from SQLite type")),
                    _ => Err(err!("Failed to convert from SQLite type"))
                }
            }
        }
    };
    (into: $type:ty => $intermediate:ty => $variant:path) => {
        impl TryFrom<$type> for SqliteType {
            type Error = Error;

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                <$intermediate>::try_from(value).map($variant)
                    .map_err(|e| err!(with: e, "Failed to convert into SQLite type"))
            }
        }
        impl TryFrom<Option<$type>> for SqliteType {
            type Error = Error;

            fn try_from(value: Option<$type>) -> Result<Self, Self::Error> {
                match value {
                    None => Ok(Self::Null),
                    Some(value) => <$intermediate>::try_from(value).map($variant)
                        .map_err(|e| err!(with: e, "Failed to convert into SQLite type")),
                }
            }
        }
    };
    ($type:ty => $intermediate:ty => $variant:path) => {
        impl_sqlitetype_conversion!(from: $variant => $type);
        impl_sqlitetype_conversion!(into: $type => $intermediate => $variant);
    };
}
// Integers
impl_sqlitetype_conversion!(usize => i64 => SqliteType::Integer);
impl_sqlitetype_conversion!(isize => i64 => SqliteType::Integer);
impl_sqlitetype_conversion!(u128 => i64 => SqliteType::Integer);
impl_sqlitetype_conversion!(i128 => i64 => SqliteType::Integer);
impl_sqlitetype_conversion!(u64 => i64 => SqliteType::Integer);
impl_sqlitetype_conversion!(i64 => i64 => SqliteType::Integer);
impl_sqlitetype_conversion!(u32 => i64 => SqliteType::Integer);
impl_sqlitetype_conversion!(i32 => i64 => SqliteType::Integer);
impl_sqlitetype_conversion!(u16 => i64 => SqliteType::Integer);
impl_sqlitetype_conversion!(i16 => i64 => SqliteType::Integer);
impl_sqlitetype_conversion!(u8 => i64 => SqliteType::Integer);
impl_sqlitetype_conversion!(i8 => i64 => SqliteType::Integer);
// Floats
impl_sqlitetype_conversion!(f64 => f64 => SqliteType::Real);
impl_sqlitetype_conversion!(into: f32 => f64 => SqliteType::Real);
// Strings
impl_sqlitetype_conversion!(String => String => SqliteType::Text);
impl_sqlitetype_conversion!(into: &str => String => SqliteType::Text);
impl_sqlitetype_conversion!(from: SqliteType::Text => Arc<String>);
// Bytes
impl_sqlitetype_conversion!(Vec<u8> => Vec<u8> => SqliteType::Blob);
impl_sqlitetype_conversion!(into: &[u8] => Vec<u8> => SqliteType::Blob);
impl_sqlitetype_conversion!(from: SqliteType::Blob => Arc<Vec<u8>>);
