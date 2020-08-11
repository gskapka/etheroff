use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Custom(String),
    IOError(std::io::Error),
    CryptoError(secp256k1::Error),
    UTF8Error(std::str::Utf8Error),
    NoneError(std::option::NoneError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Custom(ref msg) => msg.to_string(),
            AppError::IOError(ref e) => format!("✘ I/O error: {}", e),
            AppError::UTF8Error(ref e) => format!("✘ UTF8 error: {}", e),
            AppError::NoneError(ref e) => format!("✘ None error: {:?}", e),
            AppError::CryptoError(ref e) => format!("✘ Crypto error: {}", e),

        };
        f.write_fmt(format_args!("{}", msg))
    }
}

impl From<secp256k1::Error> for AppError {
    fn from(e: secp256k1::Error) -> AppError {
        AppError::CryptoError(e)
    }
}

impl From<std::str::Utf8Error> for AppError {
    fn from(e: std::str::Utf8Error) -> AppError {
        AppError::UTF8Error(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        AppError::IOError(e)
    }
}

impl From<std::option::NoneError> for AppError {
    fn from(e: std::option::NoneError) -> AppError {
        AppError::NoneError(e)
    }
}
