pub mod error;
pub mod throw;

use crate::error::RisError;

pub type RisResult<T> = Result<T, RisError>;
