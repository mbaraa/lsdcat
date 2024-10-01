use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AppError {
    InvalidArgumentName,
    InvalidArgumentValue,
    Io(std::io::Error),
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
