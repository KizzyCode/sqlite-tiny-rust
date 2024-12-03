//! Implements the crate's error type

use std::{
    backtrace::{Backtrace, BacktraceStatus},
    fmt::{self, Display, Formatter},
};

/// Creates a new error
#[macro_export]
macro_rules! err {
    (with: $error:expr, $($arg:tt)*) => {{
        let error = format!($($arg)*);
        let source = Box::new($error);
        $crate::error::Error::new(error, Some(source))
    }};
    ($($arg:tt)*) => {{
        let error = format!($($arg)*);
        $crate::error::Error::new(error, None)
    }};
}

/// The crates error type
#[derive(Debug)]
pub struct Error {
    /// The error description
    pub error: String,
    /// The underlying error
    pub source: Option<Box<dyn std::error::Error + Send>>,
    /// The backtrace
    pub backtrace: Backtrace,
}
impl Error {
    /// Creates a new error and captures a backtrace
    pub fn new(error: String, source: Option<Box<dyn std::error::Error + Send>>) -> Self {
        let backtrace = Backtrace::capture();
        Self { error, source, backtrace }
    }

    /// Whether the error has captured a backtrace or not
    pub fn has_backtrace(&self) -> bool {
        self.backtrace.status() == BacktraceStatus::Captured
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Print the error
        writeln!(f, "{}", self.error)?;

        // Print the source
        if let Some(source) = &self.source {
            writeln!(f, " caused by: {source}")?;
        }
        Ok(())
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Do some type gymnastics to get the `Send` out of the typesystem, because it breaks a direct conversion
        #[allow(clippy::borrowed_box, reason = "Type gymnastics to remove the `Send`")]
        let source: &Box<dyn std::error::Error + Send> = self.source.as_ref()?;
        let source: &dyn std::error::Error = source.as_ref();
        Some(source)
    }
}
