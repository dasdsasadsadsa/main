use std::fmt;

#[derive(Debug)]
pub struct ScanError {
    pub message: String,
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScanError: {}", self.message)
    }
}

impl std::error::Error for ScanError {}

impl From<std::io::Error> for ScanError {
    fn from(err: std::io::Error) -> Self {
        ScanError {
            message: err.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, ScanError>;

pub fn new_error(message: &str) -> ScanError {
    ScanError {
        message: message.to_string(),
    }
}
