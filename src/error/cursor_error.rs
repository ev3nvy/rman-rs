use thiserror::Error;

use super::ReadError;

#[derive(Error, Debug)]
pub enum CursorError<T = Box<dyn std::error::Error>> {
    #[error("{0}")]
    ReadError(#[from] ReadError),
    #[error("{0}")]
    ReadManyError(T),
    #[error("SeekError: {0}")]
    SeekError(#[from] std::io::Error),
}
