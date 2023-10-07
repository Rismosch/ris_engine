use std::error::Error;

pub type SourceError = Option<std::sync::Arc<dyn Error + 'static>>;

pub type RisResult<T> = Result<T, RisError>;

#[derive(Clone)]
pub struct RisError {
    source: SourceError,
    message: String,
    file: String,
    line: u32,
}

impl RisError {
    pub fn new(source: SourceError, message: String, file: String, line: u32) -> Self {
        Self {
            source,
            message,
            file,
            line,
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

        write!(f, "\"{}\", {}:{}", self.message, self.file, self.line)
    }
}

impl std::fmt::Debug for RisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source_string = match &self.source {
            Some(source) => format!("Some ({:?})", source),
            None => String::from("None"),
        };

        write!(
            f,
            "RisError {{inner: {}, message: {}, file: {}, line: {}}}",
            source_string, self.message, self.file, self.line
        )
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
    ($result:expr, $($arg:tt)*) => {
        match $result {
            Ok(value) => Ok(value),
            Err(error) => {
                let source: ris_util::ris_error::SourceError = Some(std::sync::Arc::new(error));
                let message = format!($($arg)*);
                let file = String::from(file!());
                let line = line!();
                let result = ris_util::ris_error::RisError::new(source, message, file, line);
                Err(result)
            }
        }
    };
}

#[macro_export]
macro_rules! unroll_option {
    ($result:expr, $($arg:tt)*) => {
        match $result {
            Some(value) => Ok(value),
            None => {
                let source: ris_util::ris_error::SourceError = Some(std::sync::Arc::new(ris_util::ris_error::OptionError));
                let message = format!($($arg)*);
                let file = String::from(file!());
                let line = line!();
                let result = ris_util::ris_error::RisError::new(source, message, file, line);
                Err(result)
            },
        }
    };
}

#[macro_export]
macro_rules! new_err {
    ($($arg:tt)*) => {
        {
            let source: ris_util::ris_error::SourceError = None;
            let message = format!($($arg)*);
            let file = String::from(file!());
            let line = line!();
            ris_util::ris_error::RisError::new(source, message, file, line)
        }
    };
}

#[macro_export]
macro_rules! result_err {
    ($($arg:tt)*) => {
        {
            let result = ris_util::new_err!($($arg)*);
            Err(result)
        }
    };
}
