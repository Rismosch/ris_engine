// QOI by Dominic Szablewski: https://qoiformat.org/

use std::io::Cursor;

pub struct QoiDesc {
    pub width: usize,
    pub height: usize,
    pub channels: Channels,
    pub color_space: ColorSpace,
}

pub enum Channels {
    RGB,
    RGBA,
}

pub enum ColorSpace {
    SRGB,
    Linear,
}

impl From<Channels> for usize {
    fn from(value: Channels) -> Self {
        match value {
            Channels::RGB => 3,
            Channels::RGBA => 4,
        }
    }
}

const OP_INDEX: usize = 0x00; /* 00xxxxxx */
const OP_DIFF: usize = 0x40; /* 01xxxxxx */
const OP_LUMA: usize = 0x80; /* 10xxxxxx */
const OP_RUN: usize = 0xc0; /* 11xxxxxx */
const OP_RGB: usize = 0xfe; /* 11111110 */
const OP_RGBA: usize = 0xff; /* 11111111 */

const MASK_2: usize = 0xc0; /* 11000000 */

// "qoif"
const MAGIC: [u8; 4] = [0x71, 0x6f, 0x69, 0x66];
const HEADER_SIZE: usize = 14;

const PIXELS_MAX: usize = 400000000;

const PADDING: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 1];

pub enum EncodeErrorKind {
    WidthIsZero,
    HeightIsZero,
    DimensionsTooLarge,
}

pub enum DecodeErrorKind {}

pub struct EncodeError {
    pub kind: EncodeErrorKind,
}

pub struct DecodeError {
    pub kind: DecodeErrorKind,
}

pub fn encode(data: &[u8], desc: QoiDesc) -> Result<Vec<u8>, EncodeError> {
    if desc.width == 0 {
        return Err(EncodeError {
            kind: EncodeErrorKind::WidthIsZero,
        });
    }

    if desc.height == 0 {
        return Err(EncodeError {
            kind: EncodeErrorKind::HeightIsZero,
        });
    }

    if desc.height >= PIXELS_MAX / desc.width {
        return Err(EncodeError {
            kind: EncodeErrorKind::DimensionsTooLarge,
        });
    }

    let max_size =
        desc.width * desc.height * (desc.channels as usize + 1) + HEADER_SIZE + PADDING.len();

    let bytes = Cursor::new(Vec::with_capacity(max_size));

    panic!("reached end of encode");
    let result = bytes.into_inner();
    Ok(result)
}

pub fn decode() {
    panic!();
}
