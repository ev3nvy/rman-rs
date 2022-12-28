use std::fmt::{Debug, Display, Formatter, Result};

use super::ReadError;

#[derive(Debug)]
pub enum CursorError<T = Box<dyn std::error::Error>> {
    ReadError(ReadError<T>),
    ReadManyError(T),
    SeekError(T),
}

impl<T: std::error::Error> std::error::Error for CursorError<T> {}

impl<T: std::error::Error> Display for CursorError<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            CursorError::ReadError(error) => writeln!(f, "{}", error),
            CursorError::ReadManyError(error) => writeln!(f, "{}", error),
            CursorError::SeekError(error) => writeln!(f, "SeekError: {}", error),
        }
    }
}
