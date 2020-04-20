use std::fmt;

#[derive(Debug)]
pub enum Error {
    CheckConnection(),
    FDError(std::io::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::CheckConnection() => format!("No connection with modem"),
            Error::FDError(err) => format!("Failed to write to fd: {}", err),
        })
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::FDError(err)
    }
}