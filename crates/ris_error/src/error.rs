use std::backtrace::Backtrace;
use std::error::Error;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Local;

pub static mut PRINT_WARNING_ON_BACKTRACE: bool = true;

pub type SourceError = Option<Arc<dyn Error + 'static>>;
pub type RisResult<T> = Result<T, RisError>;

#[derive(Clone)]
pub struct RisError {
    pub source: SourceError,
    pub message: Option<String>,
    pub file: String,
    pub line: u32,
    pub backtrace: Arc<Backtrace>,
}

impl std::fmt::Display for RisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(source) = &self.source {
            write!(f, "source: {}", source)?;
        } else {
            write!(f, "source: none")?;
        }

        if let Some(message) = &self.message {
            write!(f, "\n    message: \"{}\"", message)?;
        } else {
            write!(f, "\n    message: none")?;
        }

        write!(f, "\n    at {}:{}", self.file, self.line)
    }
}

impl std::fmt::Debug for RisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nbacktrace:\n{}", self, self.backtrace)
    }
}

#[derive(Debug)]
pub struct OptionError;

impl Error for OptionError {}

impl std::fmt::Display for OptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Option was None")
    }
}

impl<E: Error + 'static> From<E> for RisError {
    fn from(value: E) -> Self {
        Self {
            source: Some(Arc::new(value)),
            message: None,
            file: String::from(file!()),
            line: line!(),
            backtrace: crate::get_backtrace!(),
        }
    }
}

pub trait Extensions<T> {
    fn unroll(self) -> Result<T, RisError>;
}

impl<T> Extensions<T> for Option<T> {
    fn unroll(self) -> Result<T, RisError> {
        match self {
            Some(value) => Ok(value),
            None => Err(RisError::from(OptionError)),
        }
    }
}

impl<T, E: std::fmt::Display> Extensions<T> for Result<T, E> {
    fn unroll(self) -> Result<T, RisError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => crate::new_result!("{}", e),
        }
    }
}

pub fn get_timestamp() -> DateTime<Local> {
    Local::now()
}

#[macro_export]
macro_rules! new {
    ($($arg:tt)*) => {{
        use $crate::error::RisError;

        let source = None;
        let message = Some(format!($($arg)*));
        let file = String::from(file!());
        let line = line!();
        let backtrace = $crate::get_backtrace!();
        RisError{source, message, file, line, backtrace}
    }};
}

#[macro_export]
macro_rules! new_result {
    ($($arg:tt)*) => {{
        let result = $crate::new!($($arg)*);
        Err(result)
    }};
}

#[macro_export]
macro_rules! get_backtrace {
    () => {{
        use std::backtrace::Backtrace;
        use std::sync::Arc;

        let backtrace = Arc::new(Backtrace::force_capture());

        if unsafe {$crate::error::PRINT_WARNING_ON_BACKTRACE} {
            ris_log::warning!("created backtrace. this operation is expensive. excessive use may cost performance");
        }

        backtrace
    }}
}

#[macro_export]
macro_rules! assert {
    ($value:expr) => {{
        if $value {
            Ok(())
        } else {
            ris_error::new_result!("assertion failed: `{}` was false", stringify!($value))
        }
    }};
}

#[macro_export]
macro_rules! debug_assert {
    ($value:expr) => {{
        #[cfg(not(debug_assertions))]
        {
            let _ = $value;
            let result: ris_error::RisResult<()> = Ok(());
            result
        }

        #[cfg(debug_assertions)]
        {
            if $value {
                Ok(())
            } else {
                ris_error::new_result!("assertion failed: `{}` was false", stringify!($value))
            }
        }
    }};
}
