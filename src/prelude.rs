//! Crate prelude
pub use crate::error::Error;
pub use crate::utils::*;
pub use crate::events;

pub type Result<T> = core::result::Result<T, Error>;