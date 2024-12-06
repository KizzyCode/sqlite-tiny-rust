//! Bridge types to help with C and SQLite data types

use crate::{err, error::Error};
use core::ffi::c_int;

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
macro_rules! impl_sqlitetype_conversion {
    (from: $variant:path => $type:ty) => {
        impl TryInto<$type> for SqliteType {
            type Error = Error;

            fn try_into(self) -> Result<$type, Self::Error> {
                match self {
                    $variant(value) => <$type>::try_from(value)
                        .map_err(|e| err!(with: e, "Failed to from SQLite type")),
                    _ => Err(err!("Failed to convert from SQLite type"))
                }
            }
        }
        impl TryInto<Option<$type>> for SqliteType {
            type Error = Error;

            fn try_into(self) -> Result<Option<$type>, Self::Error> {
                match self {
                    Self::Null => Ok(None),
                    $variant(value) => <$type>::try_from(value)
                        .map(Some).map_err(|e| err!(with: e, "Failed to from SQLite type")),
                    _ => Err(err!("Failed to convert from SQLite type"))
                }
            }
        }
    };
    (into: $type:ty => $intermediate:ty => $variant:path) => {
        impl TryFrom<$type> for SqliteType {
            type Error = Error;

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                <$intermediate>::try_from(value)
                    .map($variant).map_err(|e| err!(with: e, "Failed to convert into SQLite type"))
            }
        }
        impl TryFrom<Option<$type>> for SqliteType {
            type Error = Error;

            fn try_from(value: Option<$type>) -> Result<Self, Self::Error> {
                match value {
                    None => Ok(Self::Null),
                    Some(value) => <$intermediate>::try_from(value)
                        .map($variant).map_err(|e| err!(with: e, "Failed to convert into SQLite type")),
                }
            }
        }
    };
    ($type:ty => $intermediate:ty => $variant:path) => {
        impl_sqlitetype_conversion!(from: $variant => $type);
        impl_sqlitetype_conversion!(into: $type => $intermediate => $variant);
    };
}
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
impl_sqlitetype_conversion!(f64 => f64 => SqliteType::Real);
impl_sqlitetype_conversion!(into: f32 => f64 => SqliteType::Real);
impl_sqlitetype_conversion!(String => String => SqliteType::Text);
impl_sqlitetype_conversion!(into: &str => String => SqliteType::Text);
impl_sqlitetype_conversion!(Vec<u8> => Vec<u8> => SqliteType::Blob);
impl_sqlitetype_conversion!(into: &[u8] => Vec<u8> => SqliteType::Blob);

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
