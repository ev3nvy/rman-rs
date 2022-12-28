use std::io::Read;

#[derive(Debug, Default)]
pub struct Cursor<'a> {
    pub cursor: std::io::Cursor<&'a [u8]>,
}

impl<'a> Cursor<'a> {
    pub fn new(cursor: std::io::Cursor<&'a [u8]>) -> Self {
        Self { cursor }
    }
}

impl<'a> From<&'a [u8]> for Cursor<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        let cursor = std::io::Cursor::new(bytes);
        Self { cursor }
    }
}

impl<'a> From<std::io::Cursor<&'a [u8]>> for Cursor<'a> {
    fn from(cursor: std::io::Cursor<&'a [u8]>) -> Self {
        Self { cursor }
    }
}

macro_rules! read {
    ($type: ident, $read_type: ident) => {
        paste::item! {
            impl<'a> Cursor<'a> {
                pub fn [<read_$type>](&mut self) -> Result<$type, crate::error::Error> {
                    let mut buffer = [0u8; std::mem::size_of::<$type>()];

                    if let Err(error) = self.cursor.read_exact(&mut buffer) {
                        let read_error = crate::error::ReadError::$read_type(error.into());
                        let cursor_error = crate::error::CursorError::ReadError(read_error);
                        return Err(crate::error::Error::CursorError(cursor_error));
                    }

                    let result = $type::from_le_bytes(buffer);

                    Ok(result)
                }
            }
        }
    };
}

read!(i8, I8);
read!(i16, I16);
read!(i32, I32);
read!(i64, I64);
read!(u8, U8);
read!(u16, U16);
read!(u32, U32);
read!(u64, U64);
