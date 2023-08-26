// use crate::generation_procedure;
// use crate::stored_procedure;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    HkdfError,
    Base64Error(data_encoding::DecodeError),
    AesError(aes_gcm::Error), // Here aes_gcm::Error is used directly
    Utf8Error(std::string::FromUtf8Error),
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

#[derive(Debug)]
pub enum AesError {
    Generic,
}

impl From<aes_gcm::Error> for AesError {
    fn from(_err: aes_gcm::Error) -> AesError {
        AesError::Generic
    }
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

use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use std::error::Error;

#[derive(Debug)]
pub enum MyError {
    RecordNotFound,
    ReqwestError(ReqwestError),
    JsonParseError(SerdeJsonError),
    TaskJoinError(tokio::task::JoinError),
    // Add other error variants as needed
}

impl From<ReqwestError> for MyError {
    fn from(err: ReqwestError) -> MyError {
        MyError::ReqwestError(err)
    }
}

impl From<SerdeJsonError> for MyError {
    fn from(err: SerdeJsonError) -> MyError {
        MyError::JsonParseError(err)
    }
}

impl From<tokio::task::JoinError> for MyError {
    fn from(err: tokio::task::JoinError) -> MyError {
        MyError::TaskJoinError(err)
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::RecordNotFound => write!(f, "Record not found"),
            MyError::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
            MyError::JsonParseError(err) => write!(f, "JSON parse error: {}", err),
            MyError::TaskJoinError(err) => write!(f, "Task join error: {}", err),
        }
    }
}

impl Error for MyError {}
