use std::io::Cursor;

use byteorder::{ReadBytesExt, LE};
use log::{debug, info, warn};

use crate::error::ManifestError;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Header {
    pub magic: u32,
    pub major: u8,
    pub minor: u8,
    pub flags: u16,
    pub offset: u32,
    pub compressed_size: u32,
    pub manifest_id: u64,
    pub uncompressed_size: u32,
}

impl TryFrom<&[u8]> for Header {
    type Error = ManifestError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(bytes);
        let magic = cursor.read_u32::<LE>()?;

        // N A M R (RMAN bacwards because I am reading this as an u32, instead
        // of as an array of chars)
        if magic != 0x4E414D52 {
            return Err(ManifestError::InvalidMagicBytes(magic));
        }

        let major = cursor.read_u8()?;
        if major != 2 {
            warn!("Invalid major version. Parsing the manfiset may not work.");
            info!("If you want the crate to throw an error instead, you can enable the \"version_error\" feature");
            #[cfg(feature = "version_error")]
            return Err(ManifestError::InvalidMajor(major));
        }

        let minor = cursor.read_u8()?;
        if major == 2 && minor != 0 {
            info!("Invalid minor version. Parsing the manfiset will probably still work.");
            info!("If you want the crate to throw an error instead, you can enable the \"version_error\" feature");
            #[cfg(feature = "version_error")]
            return Err(ManifestError::InvalidMinor(minor));
        }

        let flags = cursor.read_u16::<LE>()?;

        let offset = cursor.read_u32::<LE>()?;

        debug!("Attempting to convert \"bytes.len()\" into \"u32\".");
        let size: u32 = bytes.len().try_into()?;
        debug!("Successfully converted \"bytes.len()\" into \"u32\".");

        debug!("The file is {size} bytes in size");

        if offset < 28 || offset >= size {
            return Err(ManifestError::InvalidOffset(offset));
        }

        let compressed_size = cursor.read_u32::<LE>()?;
        if compressed_size > size - 28 {
            return Err(ManifestError::CompressedSizeTooLarge(compressed_size));
        }
        if compressed_size + offset > size {
            return Err(ManifestError::CompressedSizeTooLarge(
                compressed_size + offset,
            ));
        }

        let manifest_id = cursor.read_u64::<LE>()?;
        let uncompressed_size = cursor.read_u32::<LE>()?;

        let file_header = Self {
            magic,
            major,
            minor,
            flags,
            offset,
            compressed_size,
            manifest_id,
            uncompressed_size,
        };

        Ok(file_header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod helpers {
        pub const VALID_HEADER: [u8; 32] = [
            0x52, 0x4D, 0x41, 0x4E, 0x02, 0x00, 0x00, 0x02, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xD2, 0x17, 0xC3, 0xEF, 0xAB, 0x4A, 0x9C, 0x2B, 0x60, 0xA4, 0xD0, 0x01,
            0x00, 0x00, 0x00, 0x00,
        ];

        macro_rules! assert_error {
            ($buf: ident, $error: ident) => {
                let Err(error) = crate::Header::try_from(&$buf[..]) else {
                                                    panic!("did not throw an error");
                                                };
                let crate::error::ManifestError::$error(..) = error else {
                                                    panic!("some other error was thrown");
                                                };
            };
        }

        pub(crate) use assert_error;
    }

    #[test]
    fn should_parse_when_valid_header() {
        if let Err(error) = Header::try_from(&helpers::VALID_HEADER[..]) {
            panic!(
                "there was an error when parsing header, header: {:?}",
                error
            );
        };
    }

    #[test]
    fn should_have_correct_values_when_valid_header() {
        let header = Header::try_from(&helpers::VALID_HEADER[..]).unwrap();

        assert_eq!(header.magic, 0x4E414D52, "magic bytes did not match");
        assert_eq!(header.major, 2, "major version did not match");
        assert_eq!(header.minor, 0, "minor version did not match");
        assert_eq!(header.flags, 512, "flags did not match");
        assert_eq!(header.offset, 28, "offset did not match");
        assert_eq!(header.compressed_size, 0, "compressed size did not match");
        assert_eq!(
            header.manifest_id, 3142468742320166866,
            "manifest id did not match"
        );
        assert_eq!(
            header.uncompressed_size, 30450784,
            "unompressed size did not match"
        );
    }

    #[test]
    fn should_throw_correct_errors_when_eof() {
        // EOF when reading magic bytes
        let error = Header::try_from(&helpers::VALID_HEADER[..3])
            .err()
            .expect("did not throw an error on missing bytes");
        match error {
            crate::error::ManifestError::ReadBytesError(_) => (),
            _ => panic!("invalid ManifestError error when eof"),
        };

        // EOF when reading major
        let error = Header::try_from(&helpers::VALID_HEADER[..4])
            .err()
            .expect("did not throw an error on missing bytes");
        match error {
            crate::error::ManifestError::ReadBytesError(_) => (),
            _ => panic!("invalid ManifestError error when eof"),
        };

        // EOF when reading minor
        let error = Header::try_from(&helpers::VALID_HEADER[..5])
            .err()
            .expect("did not throw an error on missing bytes");
        match error {
            crate::error::ManifestError::ReadBytesError(_) => (),
            _ => panic!("invalid ManifestError error when eof"),
        };

        // EOF when reading flags
        let error = Header::try_from(&helpers::VALID_HEADER[..7])
            .err()
            .expect("did not throw an error on missing bytes");
        match error {
            crate::error::ManifestError::ReadBytesError(_) => (),
            _ => panic!("invalid ManifestError error when eof"),
        };

        // EOF when reading offset
        let error = Header::try_from(&helpers::VALID_HEADER[..11])
            .err()
            .expect("did not throw an error on missing bytes");
        match error {
            crate::error::ManifestError::ReadBytesError(_) => (),
            _ => panic!("invalid ManifestError error when eof"),
        };

        // it should be impossible for reading to fail at this point, because
        // offset must be greater than 28 and and less then file size, which
        // in turn ensures that the file size is at least 28 bytes
    }

    #[test]
    fn should_error_when_invalid_magic_bytes() {
        let buf = [&[0x53], &helpers::VALID_HEADER[1..]].concat();

        helpers::assert_error!(buf, InvalidMagicBytes);
    }

    #[test]
    fn should_error_when_invalid_major() {
        let buf = [
            &helpers::VALID_HEADER[..4],
            &[0x01],
            &helpers::VALID_HEADER[5..],
        ]
        .concat();

        #[cfg(not(feature = "version_error"))]
        if let Err(_) = Header::try_from(&buf[..]) {
            panic!("error was thrown");
        }

        #[cfg(feature = "version_error")]
        helpers::assert_error!(buf, InvalidMajor);
    }

    #[test]
    fn should_error_when_invalid_minor() {
        let buf = [
            &helpers::VALID_HEADER[..5],
            &[0x01],
            &helpers::VALID_HEADER[6..],
        ]
        .concat();

        #[cfg(not(feature = "version_error"))]
        if let Err(_) = Header::try_from(&buf[..]) {
            panic!("error was thrown")
        }

        #[cfg(feature = "version_error")]
        helpers::assert_error!(buf, InvalidMinor);
    }

    #[test]
    fn should_error_when_offset_too_small() {
        // too small
        let buf = [
            &helpers::VALID_HEADER[..8],
            &0u32.to_le_bytes(),
            &helpers::VALID_HEADER[12..],
        ]
        .concat();

        helpers::assert_error!(buf, InvalidOffset);

        // slightly too small
        let buf = [
            &helpers::VALID_HEADER[..8],
            &27u32.to_le_bytes(),
            &helpers::VALID_HEADER[12..],
        ]
        .concat();

        helpers::assert_error!(buf, InvalidOffset);
    }

    #[test]
    fn should_error_when_offset_too_large() {
        // slightly too large
        let size = u32::try_from(helpers::VALID_HEADER.len()).unwrap();
        let buf = [
            &helpers::VALID_HEADER[..8],
            &size.to_le_bytes(),
            &helpers::VALID_HEADER[12..],
        ]
        .concat();

        helpers::assert_error!(buf, InvalidOffset);

        // too large
        let buf = [
            &helpers::VALID_HEADER[..8],
            &u32::MAX.to_le_bytes(),
            &helpers::VALID_HEADER[12..],
        ]
        .concat();

        helpers::assert_error!(buf, InvalidOffset);
    }

    #[test]
    fn should_error_when_compressed_size_too_large() {
        // slightly too large
        let size = u32::try_from(helpers::VALID_HEADER.len()).unwrap();
        let buf = [
            &helpers::VALID_HEADER[..12],
            &(size - 27).to_le_bytes(),
            &helpers::VALID_HEADER[16..],
        ]
        .concat();

        helpers::assert_error!(buf, CompressedSizeTooLarge);

        // too large
        let buf = [
            &helpers::VALID_HEADER[..12],
            &u32::MAX.to_le_bytes(),
            &helpers::VALID_HEADER[16..],
        ]
        .concat();

        helpers::assert_error!(buf, CompressedSizeTooLarge);
    }

    #[test]
    fn should_error_when_compressed_size_and_offset_too_large() {
        let offset = 28u32;
        let size = u32::try_from(helpers::VALID_HEADER.len()).unwrap();
        let buf = [
            &helpers::VALID_HEADER[..8],
            &offset.to_le_bytes(),
            &(size - offset + 1).to_le_bytes(),
            &helpers::VALID_HEADER[16..],
        ]
        .concat();

        helpers::assert_error!(buf, CompressedSizeTooLarge);
    }
}
