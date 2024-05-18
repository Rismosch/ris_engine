use std::error::Error;

pub type CiResult<T> = Result<T, CiError>;

#[derive(Clone)]
pub struct CiError {
    pub message: String,
}

impl std::fmt::Display for CiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<E: Error + 'static> From<E> for CiError {
    fn from(value: E) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

pub trait CiResultExtensions<T> {
    fn to_ci_result(self) -> CiResult<T>;
}

impl<T> CiResultExtensions<T> for Option<T> {
    fn to_ci_result(self) -> CiResult<T> {
        match self {
            Some(value) => Ok(value),
            None => crate::new_error_result!("option was none"),
        }
    }
}

#[macro_export]
macro_rules! new_error {
    ($($arg:tt)*) => {{
        use $crate::ci_error::CiError;

        let message = format!($($arg)*);
        CiError{message}
    }};
}

#[macro_export]
macro_rules! new_error_result {
    ($($arg:tt)*) => {{
        let result = $crate::new_error!($($arg)*);
        Err(result)
    }};
}

#[macro_export]
macro_rules! get_backtrace {
    () => {{
        use std::backtrace::Backtrace;
        use std::sync::Arc;

        Arc::new(Backtrace::force_capture())
    }}
}
