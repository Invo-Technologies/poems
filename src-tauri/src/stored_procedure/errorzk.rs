use std::error::Error;
use std::fmt;

// =====================================
// CustomError Enum and its Implementations
// =====================================

#[derive(Debug)]
pub enum CustomError {
    HkdfError,
    Base64Error(data_encoding::DecodeError),
    AesError(aes_gcm::Error),
    Utf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::HkdfError => write!(f, "Failed to generate key"),
            CustomError::Base64Error(ref err) => write!(f, "Base64 decoding error: {}", err),
            CustomError::AesError(_) => {
                write!(f, "Decryption failed, please check your secret key")
            }
            CustomError::Utf8Error(ref err) => write!(f, "UTF-8 conversion error: {}", err),
        }
    }
}

impl From<aes_gcm::Error> for CustomError {
    fn from(err: aes_gcm::Error) -> CustomError {
        CustomError::AesError(err)
    }
}

impl From<data_encoding::DecodeError> for CustomError {
    fn from(err: data_encoding::DecodeError) -> CustomError {
        CustomError::Base64Error(err)
    }
}

impl From<std::string::FromUtf8Error> for CustomError {
    fn from(err: std::string::FromUtf8Error) -> CustomError {
        CustomError::Utf8Error(err)
    }
}

// =====================================
// AesError Enum and its Implementations
// =====================================

#[derive(Debug)]
pub enum AesError {
    Generic,
}

impl fmt::Display for AesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AesError::Generic => write!(
                f,
                "The provided key did not decrypt the Cipher. Please try again."
            ),
        }
    }
}

impl From<aes_gcm::Error> for AesError {
    fn from(_err: aes_gcm::Error) -> AesError {
        AesError::Generic
    }
}

// =====================================
// MyError Enum and its Implementations
// =====================================

#[derive(Debug)]
pub enum MyError {
    RecordNotFound,
    ReqwestError(reqwest::Error),
    JsonParseError(serde_json::Error),
    TaskJoinError(tokio::task::JoinError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::RecordNotFound => write!(f, "Record not found"),
            MyError::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
            MyError::JsonParseError(err) => write!(f, "JSON parse error: {}", err),
            MyError::TaskJoinError(err) => write!(f, "Task join error: {}", err),
        }
    }
}

impl Error for MyError {}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> MyError {
        MyError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> MyError {
        MyError::JsonParseError(err)
    }
}

impl From<tokio::task::JoinError> for MyError {
    fn from(err: tokio::task::JoinError) -> MyError {
        MyError::TaskJoinError(err)
    }
}
