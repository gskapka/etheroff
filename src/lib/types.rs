use std::result;

use crate::lib::errors::AppError;
// NOTE: Temporary, until try_trait is stabilized
pub(crate) use crate::lib::errors::AppError::NoneError;

pub type Byte = u8;
pub type Bytes = Vec<Byte>;
pub type Result<T> = result::Result<T, AppError>;
