use std::result;
use crate::lib::errors::AppError;

pub type Byte = u8;
pub type Bytes = Vec<Byte>;
pub type Result<T> = result::Result<T, AppError>;
