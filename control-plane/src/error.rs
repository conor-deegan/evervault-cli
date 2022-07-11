use std::fmt::Formatter;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    IoError(#[from] std::io::Error),
    HyperError(#[from] hyper::Error),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = std::result::Result<T, ServerError>;
