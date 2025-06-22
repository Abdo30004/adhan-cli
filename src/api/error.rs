use std::fmt;

#[derive(Debug)]
pub enum AdhanError {
    Reqwest(reqwest::Error),
    ChronoParse(chrono::ParseError),
    InvalidPeriod,
}

impl fmt::Display for AdhanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdhanError::Reqwest(e) => write!(f, "Request error: {}", e),
            AdhanError::ChronoParse(e) => write!(f, "Parse error: {}", e),
            AdhanError::InvalidPeriod => write!(f, "Invalid prayer times period"),
        }
    }
}

impl std::error::Error for AdhanError {}

impl From<reqwest::Error> for AdhanError {
    fn from(err: reqwest::Error) -> Self {
        AdhanError::Reqwest(err)
    }
}

impl From<chrono::ParseError> for AdhanError {
    fn from(err: chrono::ParseError) -> Self {
        AdhanError::ChronoParse(err)
    }
}
