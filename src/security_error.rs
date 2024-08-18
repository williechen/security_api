use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum SecurityError {
    BaseError(Box<dyn Error>),
    RequestError(reqwest::Error),
    SQLError(diesel::result::Error),
    JsonError(serde_json::Error),
}

impl Display for SecurityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            SecurityError::RequestError(ref err) => write!(f, " Request Error -> {} ", err),
            SecurityError::SQLError(ref err) => write!(f, " SQL Error -> {} ", err),
            SecurityError::JsonError(ref err) => write!(f, " Json Error -> {} ", err),
            SecurityError::BaseError(ref err) => write!(f, " Base Error -> {} ", err),
        }
    }
}

impl From<diesel::result::Error> for SecurityError {
    fn from(e: diesel::result::Error) -> Self {
        SecurityError::SQLError(e)
    }
}
