pub mod error;
pub mod panic;

pub use error::OptionExtensions;
pub use error::ResultExtensions;
pub use error::RisError;
pub use error::RisResult;

pub mod prelude {
    pub use crate::OptionExtensions;
    pub use crate::ResultExtensions;
    pub use crate::RisError;
    pub use crate::RisResult;
}
