use thiserror::Error;

use super::ReadError;

#[derive(Error, Debug)]
pub enum CursorError<T = std::io::Error> {
    #[error("{0}")]
    ReadError(#[from] ReadError),
    #[error("{0}")]
    ReadExactError(T),
    #[error("SeekError: {0}")]
    SeekError(T),
}
