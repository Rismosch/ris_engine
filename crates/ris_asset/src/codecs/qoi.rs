// Adopted QOI by Dominic Szablewski: https://qoiformat.org/

use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;

#[derive(Debug, Clone, Copy)]
pub struct QoiDesc {
    pub width: u32,
    pub height: u32,
    pub channels: Channels,
    pub color_space: ColorSpace,
}

#[derive(Debug, Clone, Copy)]
pub enum Channels {
    RGB = 3,
    RGBA = 4,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorSpace {
    SRGB = 0,
    Linear = 1,
}

impl TryFrom<u8> for Channels {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(Channels::RGB),
            4 => Ok(Channels::RGBA),
            _ => Err(DecodeError{
                kind: DecodeErrorKind::InvalidCast(format!("expected Channels to be 3 or 4, but received {}", value)),
            }),
        }
    }
}

impl TryFrom<u8> for ColorSpace {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColorSpace::SRGB),
            1 => Ok(ColorSpace::Linear),
            _ => Err(DecodeError{
                kind: DecodeErrorKind::InvalidCast(format!("expected ColorSpace to be 0 or 1, but received {}", value)),
            }),
        }
    }
}

const OP_INDEX: u8 = 0x00; /* 00xxxxxx */
const OP_DIFF: u8 = 0x40; /* 01xxxxxx */
const OP_LUMA: u8 = 0x80; /* 10xxxxxx */
const OP_RUN: u8 = 0xc0; /* 11xxxxxx */
const OP_RGB: u8 = 0xfe; /* 11111110 */
const OP_RGBA: u8 = 0xff; /* 11111111 */

const MASK_2: u8 = 0xc0; /* 11000000 */

// "qoif"
const MAGIC: [u8; 4] = [0x71, 0x6f, 0x69, 0x66];
const HEADER_SIZE: u32 = 14;

const PIXELS_MAX: u32 = 400000000;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgba {
    pub fn from_bytes(value: &[u8; 4]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        [
            self.r,
            self.g,
            self.b,
            self.a,
        ]
    }

    pub fn hash(&self) -> u8 {
        let hr = self.r.wrapping_mul(3);
        let hg = self.g.wrapping_mul(5);
        let hb = self.b.wrapping_mul(7);
        let ha = self.a.wrapping_mul(11);

        hr.wrapping_add(hg).wrapping_add(hb).wrapping_add(ha)
    }
}

const PADDING: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 1];

#[derive(Debug)]
pub enum EncodeErrorKind {
    WidthIsZero,
    HeightIsZero,
    DimensionsTooLarge,
    IoError(std::io::Error),
}

#[derive(Debug)]
pub enum DecodeErrorKind {
    DataToSmall,
    IoError(std::io::Error),
    InvalidCast(String),
}

#[derive(Debug)]
pub struct EncodeError {
    pub kind: EncodeErrorKind,
}

#[derive(Debug)]
pub struct DecodeError {
    pub kind: DecodeErrorKind,
}

impl From<std::io::Error> for EncodeError {
    fn from(value: std::io::Error) -> Self {
        Self {
            kind: EncodeErrorKind::IoError(value),
        }
    }
}

impl From<std::io::Error> for DecodeError {
    fn from(value: std::io::Error) -> Self {
        Self {
            kind: DecodeErrorKind::IoError(value),
        }
    }
}

impl std::error::Error for EncodeError {}
impl std::error::Error for DecodeError {}

impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
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
        desc.width * desc.height * (desc.channels as u32 + 1) + HEADER_SIZE + PADDING.len() as u32;

    let mut bytes = Cursor::new(Vec::with_capacity(max_size as usize));

    bytes.write(&MAGIC)?;
    bytes.write(&desc.width.to_be_bytes())?;
    bytes.write(&desc.height.to_be_bytes())?;
    bytes.write(&[desc.channels as u8, desc.color_space as u8])?;

    let pixels = data;

    let mut index = [Rgba::default();64];

    let mut run = 0;
    let mut px_prev = Rgba::from_bytes(&[0,0,0,255]);
    let mut px = px_prev;

    let px_len = (desc.width * desc.height * desc.channels as u32) as usize;
    let px_end = px_len - desc.channels as u32 as usize;
    let channels = desc.channels as u32;

    for px_pos in (0..px_len).step_by(channels as usize) {
        px.r = pixels[px_pos];
        px.g = pixels[px_pos + 1];
        px.b = pixels[px_pos + 2];

        if channels == 4 {
            px.a = pixels[px_pos + 3];
        }

        if px == px_prev {
            run += 1;
            if run == 62 || px_pos == px_end {
                bytes.write(&[OP_RUN | (run - 1)])?;
                run = 0;
            }
        } else {
            if (run > 0) {
                bytes.write(&[OP_RUN | (run - 1)])?;
            }

            let index_pos = px.hash() % 64;

            if index[index_pos as usize] == px {
                bytes.write(&[OP_INDEX | index_pos]);
            } else {
                index[index_pos as usize] = px;

                if px.a == px_prev.a {
                    let vr = (px.r as i8).wrapping_sub(px_prev.r as i8);
                    let vg = (px.g as i8).wrapping_sub(px_prev.g as i8);
                    let vb = (px.b as i8).wrapping_sub(px_prev.b as i8);

                    let vg_r = vr.wrapping_sub(vg);
                    let vg_b = vb.wrapping_sub(vg);

                    if vr > -3 && vr < 2 && vg > -3 && vg < 2 && vb > -3 && vb < 2 {
                        let dr = unsafe {std::mem::transmute::<i8, u8>((vr + 2) << 4)};
                        let dg = unsafe {std::mem::transmute::<i8, u8>((vg + 2) << 2)};
                        let db = unsafe {std::mem::transmute::<i8, u8>(vb + 2)};
                        bytes.write(&[OP_DIFF | dr | dg | db])?;
                    } else if vg_r > -9 && vg_r < 8 && vg > -33 && vg < 32 && vg_b > -9 && vg_b < 8{
                        let dr = unsafe {std::mem::transmute::<i8, u8>((vg_r + 8) << 4)};
                        let dg = unsafe {std::mem::transmute::<i8, u8>(vg + 32)};
                        let db = unsafe {std::mem::transmute::<i8, u8>(vg_b + 8)};
                        bytes.write(&[OP_LUMA | dg])?;
                        bytes.write(&[dr | db])?;
                    } else {
                        bytes.write(&[OP_RGB, px.r, px.g, px.b])?;
                    }

                } else {
                    bytes.write(&[OP_RGBA, px.r, px.g, px.b, px.a])?;
                }
            }
        }

        px_prev = px;
    }

    let _ = bytes.write(&PADDING)?;

    println!("1 {:?}", desc);

    let result = bytes.into_inner();
    Ok(result)
}

pub fn decode(data: &[u8], channels: Channels) -> Result<(Vec<u8>,QoiDesc), DecodeError> {
    if data.len() < HEADER_SIZE as usize + PADDING.len() {
        return Err(DecodeError {
            kind: DecodeErrorKind::DataToSmall,
        });
    }

    let bytes = &mut Cursor::new(data);

    let mut header_magic_bytes = [0; 4];
    let mut width_bytes = [0; 4];
    let mut height_bytes = [0; 4];
    bytes.read(&mut header_magic_bytes)?;
    bytes.read(&mut width_bytes)?;
    bytes.read(&mut height_bytes)?;
    let desc = QoiDesc{
        width: u32::from_be_bytes(width_bytes),
        height: u32::from_be_bytes(height_bytes),
        channels: read_byte(bytes)?.try_into()?,
        color_space: read_byte(bytes)?.try_into()?,
    };

    println!("2 {:?}", desc);

    panic!("reached end of decode");
}

fn read_byte(stream: &mut impl Read) -> Result<u8, std::io::Error> {
    let mut bytes = [0];
    let _ = stream.read(&mut bytes)?;
    Ok(bytes[0])
}

