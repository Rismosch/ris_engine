use std::backtrace::Backtrace;
use std::error::Error;
use std::sync::Arc;

pub type RisResult<T> = Result<T, RisError>;

#[derive(Clone)]
pub struct RisError {
    pub source_type_name: Option<String>,
    pub message: String,
    pub context: Vec<String>,
    pub file: String,
    pub line: u32,
    pub backtrace: Option<Arc<Backtrace>>,
}

impl std::fmt::Display for RisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(source_type_name) = &self.source_type_name {
            write!(f, " from {}", source_type_name)?;
        }

        write!(
            f,
            "\"{}\" at {}:{}",
            self.message, self.file, self.line,
        )
    }
}

impl std::fmt::Debug for RisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)?;

        write!(f, "\ncontext:")?;

        if self.context.is_empty() {
            write!(f, "\n   None")?;
        } else {
            for (i, message) in self.context.iter().rev().enumerate() {
                write!(f, "\n{:4}: {}", i, message)?;
            }
        }

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
            context: Vec::new(),
            file: file!().to_string(),
            line: line!(),
            backtrace: Some(crate::get_backtrace!()),
        }
    }
}

impl RisError {
    pub fn panic(self) -> ! {
        unsafe {
            if let Some(ref mut hook) = crate::FATAL_LOG_HOOK {
                hook(&format!("panic on RisError: {:?}", self));
            }
        }

        crate::panic!("panic on RisError. check log for more information");
    }
}

pub trait OptionExtensions<T> {
    fn ris_expect(self, msg: impl AsRef<str>) -> RisResult<T>;
    fn ris_pexpect(self, msg: impl AsRef<str>);
}

pub trait ResultExtensions<T> : OptionExtensions<T> {
    fn ris_context(self, msg: impl AsRef<str>) -> Self;
}

impl<T> OptionExtensions<T> for Option<T> {
    fn ris_expect(self, msg: impl AsRef<str>) -> RisResult<T> {
        self.ok_or("Option was None").ris_expect(msg)
    }

    fn ris_pexpect(self, msg: impl AsRef<str>) {
        if let Err(e) = self.ris_expect(msg) {
            crate::panic!("{}", e);
        }
    }
}

impl<T, E: std::fmt::Display> OptionExtensions<T> for Result<T, E> {
    fn ris_expect(self, msg: impl AsRef<str>) -> RisResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => crate::new!("expected {}. error: {}", msg.as_ref(), e),
        }
    }

    fn ris_pexpect(self, msg: impl AsRef<str>) {
        if let Err(e) = self.ris_expect(msg) {
            crate::panic!("{}", e);
        }
    }
}

impl<T> ResultExtensions<T> for RisResult<T> {
    fn ris_context(mut self, msg: impl AsRef<str>) -> Self {
        if let Err(e) = self.as_mut() {
            e.context.push(msg.as_ref().to_string());
        }

        self
    }
}

#[macro_export]
macro_rules! new_error {
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
            context: Vec::new(),
            file,
            line,
            backtrace,
        }
    }};
}

#[macro_export]
macro_rules! new {
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

        let backtrace = Arc::new(Backtrace::force_capture());

        unsafe {
            if let Some(ref mut hook) = $crate::BACKTRACE_HOOK {
                hook(&backtrace);
            }
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
            let retval: ris_error::RisResult<()> = Ok(());
            retval
        }

        #[cfg(debug_assertions)]
        {
            if $value {
                Ok(())
            } else {
                let message = format!($($arg)*);
                if message.len() == 0 {
                    $crate::new!(
                        "assertion failed: `{}` was false",
                        stringify!($value),
                    )
                } else {
                    $crate::new!(
                        "assertion failed: {}",
                        message,
                    )
                }
            }
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
