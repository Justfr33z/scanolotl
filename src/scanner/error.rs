use std::{
    num::ParseIntError,
    string::FromUtf8Error,
};
use async_std::{
    io::Error,
    net::AddrParseError,
    future::TimeoutError,
};

pub enum ScanError {
    Error(String),
    IOError(Error),
    TimeoutError(TimeoutError),
    FromUtf8Error(FromUtf8Error),
    JSONError(serde_json::Error),
    ParseIntError(ParseIntError),
    AddrParseError(AddrParseError),
}

impl From<Error> for ScanError {
    fn from(e: Error) -> Self {
        ScanError::IOError(e)
    }
}

impl From<FromUtf8Error> for ScanError {
    fn from(e: FromUtf8Error) -> Self {
        ScanError::FromUtf8Error(e)
    }
}

impl From<TimeoutError> for ScanError {
    fn from(e: TimeoutError) -> Self {
        ScanError::TimeoutError(e)
    }
}

impl From<serde_json::Error> for ScanError {
    fn from(e: serde_json::Error) -> Self {
        ScanError::JSONError(e)
    }
}

impl From<ParseIntError> for ScanError {
    fn from(e: ParseIntError) -> Self {
        ScanError::ParseIntError(e)
    }
}

impl From<AddrParseError> for ScanError {
    fn from(e: AddrParseError) -> Self {
        ScanError::AddrParseError(e)
    }
}