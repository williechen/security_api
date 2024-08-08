use std::fmt::{Display, Formatter};


#[derive(Debug)]
pub enum SecurityError{
    RequestError(reqwest::Error),
    RetryError(retry::Error<Box<(dyn std::error::Error + 'static)>>),
    SQLError(diesel::result::Error),
    JsonError(serde_json::Error),
}

impl std::error::Error for SecurityError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            SecurityError::RequestError(ref err) => Some(err),
            SecurityError::RetryError(ref err) => err.error.source(),
            SecurityError::SQLError(ref err) => Some(err),
            SecurityError::JsonError(ref err) => Some(err),
        }
    }
}

impl Display for SecurityError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            SecurityError::RequestError(ref err) => write!(f, " Request Error -> {} ", err),
            SecurityError::RetryError(ref err) => write!(f, " Retry Error -> {} ", err.error),
            SecurityError::SQLError(ref err) => write!(f, " SQL Error -> {} ", err),
            SecurityError::JsonError(ref err) => write!(f, " Json Error -> {} ", err),
        }
    }
}

impl From<reqwest::Error> for SecurityError {
    fn from(s: reqwest::Error) -> Self {
        SecurityError::RequestError(s)
    }
}

impl From<retry::Error<Box<(dyn std::error::Error + 'static)>>> for SecurityError {
    fn from(s: retry::Error<Box<(dyn std::error::Error + 'static)>>) -> Self {
        SecurityError::RetryError(s)
    }
}

impl From<diesel::result::Error> for SecurityError {
    fn from(s: diesel::result::Error) -> Self {
        SecurityError::SQLError(s)
    }
}

impl From<serde_json::Error> for SecurityError {
    fn from(s: serde_json::Error) -> Self {
        SecurityError::JsonError(s)
    }
}