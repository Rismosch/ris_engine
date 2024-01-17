use std::backtrace::Backtrace;
use std::error::Error;
use std::sync::Arc;

pub type RisResult<T> = Result<T, RisError>;

pub type SourceError = Option<Arc<dyn Error + 'static>>;

#[derive(Clone)]
pub struct RisError {
    source: SourceError,
    message: String,
    file: String,
    line: u32,
    backtrace: Arc<Backtrace>,
}

impl RisError {
    pub fn new(
        source: SourceError,
        message: String,
        file: String,
        line: u32,
        backtrace: Arc<Backtrace>,
    ) -> Self {
        Self {
            source,
            message,
            file,
            line,
            backtrace,
        }
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn file(&self) -> &String {
        &self.file
    }

    pub fn line(&self) -> &u32 {
        &self.line
    }

    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }
}

impl Error for RisError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl std::fmt::Display for RisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(source) = self.source() {
            write!(f, "{}\n    ", source)?;
        }

        write!(f, "\"{}\", {}:{}", self.message, self.file, self.line,)
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

#[macro_export]
macro_rules! unroll {
    ($result:expr, $($arg:tt)*) => {{
        use std::sync::Arc;

        use $crate::RisError;
        use $crate::SourceError;

        match $result {
            Ok(value) => Ok(value),
            Err(error) => {
                let source: SourceError = Some(Arc::new(error));
                let message = format!($($arg)*);
                let file = String::from(file!());
                let line = line!();
                let backtrace = $crate::get_backtrace!();
                let result = RisError::new(
                    source,
                    message,
                    file,
                    line,
                    backtrace,
                );
                Err(result)
            }
        }
    }};
}

#[macro_export]
macro_rules! unroll_option {
    ($result:expr, $($arg:tt)*) => {{
        use std::sync::Arc;

        use $crate::OptionError;
        use $crate::RisError;
        use $crate::SourceError;

        match $result {
            Some(value) => Ok(value),
            None => {
                let source: SourceError = Some(Arc::new(OptionError));
                let message = format!($($arg)*);
                let file = String::from(file!());
                let line = line!();
                let backtrace = $crate::get_backtrace!();
                let result = RisError::new(source, message, file, line, backtrace);
                Err(result)
            },
        }
    }};
}

#[macro_export]
macro_rules! new {
    ($($arg:tt)*) => {{
        use $crate::RisError;

        let source = None;
        let message = format!($($arg)*);
        let file = String::from(file!());
        let line = line!();
        let backtrace = $crate::get_backtrace!();
        RisError::new(source, message, file, line, backtrace)
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
        eprintln!(
            "\u{001B}[93mWARNING\u{001B}[0m: created backtrace. this operation is expensive. excessive use may cost performance.\n    in {}:{}\n",
            file!(),
            line!(),
        );

        backtrace
    }}
}
