pub mod error;
pub mod panic;

pub use error::Extensions;
pub use error::RisError;
pub use error::RisResult;

pub mod prelude {
    pub use crate::Extensions;
    pub use crate::RisError;
    pub use crate::RisResult;
}
