use std::backtrace::Backtrace;
use std::error::Error;
use std::sync::Arc;

pub static mut PRINT_WARNING_ON_BACKTRACE: bool = true;
// useful, for finding errors that are not logged
pub const PRINT_BACKTRACE_WHEN_GENERATED: bool = false;

pub type RisResult<T> = Result<T, RisError>;

#[derive(Clone)]
pub struct RisError {
    pub source_type_name: Option<String>,
    pub message: String,
    pub file: String,
    pub line: u32,
    pub backtrace: Option<Arc<Backtrace>>,
}

impl std::fmt::Display for RisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RisError")?;

        if let Some(source_type_name) = &self.source_type_name {
            write!(f, " from {}", source_type_name)?;
        }

        write!(
            f,
            ": \"{}\"\n    at {}:{}",
            self.message, self.file, self.line,
        )
    }
}

impl std::fmt::Debug for RisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)?;
        if let Some(backtrace) = self.backtrace.as_ref() {
            write!(f, "\nbacktrace:\n{}", backtrace)?;
        }
        Ok(())
    }
}

impl<E: Error + 'static> From<E> for RisError {
    fn from(value: E) -> Self {
        let source_type_name = Some(std::any::type_name::<E>().to_string());
        let message = format!("{}", value);
        Self {
            source_type_name,
            message,
            file: file!().to_string(),
            line: line!(),
            backtrace: Some(crate::get_backtrace!()),
        }
    }
}

pub trait Extensions<T> {
    fn ris_expect(self, msg: &str) -> Result<T, RisError>;
}

impl<T> Extensions<T> for Option<T> {
    fn ris_expect(self, msg: &str) -> Result<T, RisError> {
        self.ok_or("Option was None").ris_expect(msg)
    }
}

impl<T, E: std::fmt::Display> Extensions<T> for Result<T, E> {
    fn ris_expect(self, msg: &str) -> Result<T, RisError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => crate::new_result!("expected {}. error: {}", msg, e),
        }
    }
}

#[macro_export]
macro_rules! new {
    ($($arg:tt)*) => {{
        use $crate::error::RisError;

        let source_type_name = None;
        let message = format!($($arg)*);
        let file = String::from(file!());
        let line = line!();
        let backtrace = Some($crate::get_backtrace!());
        RisError {
            source_type_name,
            message,
            file,
            line,
            backtrace,
        }
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

        if unsafe { $crate::error::PRINT_WARNING_ON_BACKTRACE } {
            ris_log::warning!(
                "created backtrace. this operation is expensive. excessive use may cost performance"
            );
        }

        if $crate::error::PRINT_BACKTRACE_WHEN_GENERATED {
            ris_log::trace!("backtrace:\n{}", backtrace);
        }

        backtrace
    }};
}

#[macro_export]
macro_rules! assert {
    ($value:expr) => {{
        $crate::assert!($value, "")
    }};
    ($value:expr, $($arg:tt)*) => {{
        #[cfg(not(debug_assertions))]
        {
            let _ = $value;
            Ok(())
        }

        #[cfg(debug_assertions)]
        {
            if $value {
                Ok(())
            } else {
                let message = format!($($arg)*);
                if message.len() == 0 {
                    $crate::new_result!(
                        "assertion failed: `{}` was false",
                        stringify!($value),
                    )
                } else {
                    $crate::new_result!(
                        "assertion failed: {}",
                        message,
                    )
                }
            }
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
            $crate::assert!($value)
        }
    }};
}

#[macro_export]
macro_rules! declare_error {
    (
        $error_name:ident,
        $error_msg:literal $(,)?
    ) => {
        #[derive(Debug)]
        pub struct $error_name;

        impl std::fmt::Display for $error_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $error_msg)
            }
        }

        impl std::error::Error for $error_name {}
    };
}
