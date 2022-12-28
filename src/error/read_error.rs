use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ReadError<T = Box<dyn std::error::Error>> {
    I8(T),
    I16(T),
    I32(T),
    I64(T),

    U8(T),
    U16(T),
    U32(T),
    U64(T),
}

impl<T: std::error::Error> Display for ReadError<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let name = format!("{:?}", self).to_lowercase();
        let name = name.split_once("(");
        let (name, _) = name.unwrap_or(("unknown_type", ""));
        writeln!(f, "Could not read {} from buffer. Error: {:?}", name, self)
    }
}
