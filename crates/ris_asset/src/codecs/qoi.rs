// QOI implemented in Rust
// original format and C encoder/decoder by Dominic Szablewski: https://qoiformat.org/

use std::io::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QoiDesc {
    pub width: u32,
    pub height: u32,
    pub channels: Channels,
    pub color_space: ColorSpace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channels {
    RGB = 3,
    RGBA = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            _ => Err(DecodeError::InvalidCast(format!(
                "invalid Channels value. Expected 3 or 4, but received {}",
                value
            ))),
        }
    }
}

impl TryFrom<u8> for ColorSpace {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColorSpace::SRGB),
            1 => Ok(ColorSpace::Linear),
            _ => Err(DecodeError::InvalidCast(format!(
                "invalid ColorSpace value. Expected 0 or 1, but received {}",
                value
            ))),
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

const MAGIC: [u8; 4] = [0x71, 0x6f, 0x69, 0x66]; // "qoif"
const HEADER_SIZE: u32 = 14;

const DATA_MIN: usize = HEADER_SIZE as usize + PADDING.len();
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
pub enum EncodeError {
    WidthIsZero,
    HeightIsZero,
    DimensionsTooLarge,
    DataDoesNotMatchDimensions,
    WriteError(ris_io::WriteError),
}

#[derive(Debug)]
pub enum DecodeError {
    DataToSmall,
    IncorrectMagic,
    DescWidthIsZero,
    DescHeightIsZero,
    ReadError(ris_io::ReadError),
    WriteError(ris_io::WriteError),
    InvalidCast(String),
}

impl From<ris_io::WriteError> for EncodeError {
    fn from(value: ris_io::WriteError) -> Self {
        Self::WriteError(value)
    }
}

impl From<ris_io::ReadError> for DecodeError {
    fn from(value: ris_io::ReadError) -> Self {
        Self::ReadError(value)
    }
}

impl From<ris_io::WriteError> for DecodeError {
    fn from(value: ris_io::WriteError) -> Self {
        Self::WriteError(value)
    }
}

impl std::error::Error for EncodeError {}
impl std::error::Error for DecodeError {}

impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodeError::WidthIsZero => write!(f, "width may not be 0"),
            EncodeError::HeightIsZero => write!(f, "height may not be 0"),
            EncodeError::DimensionsTooLarge => {
                write!(f, "pixels may not exceed {}", PIXELS_MAX)
            }
            EncodeError::DataDoesNotMatchDimensions => {
                write!(f, "data must have length of width * height * channels")
            }
            EncodeError::WriteError(e) => write!(f, "write error occured: {}", e),
        }
    }
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::DataToSmall => write!(f, "data must be larger than {}", DATA_MIN),
            DecodeError::IncorrectMagic => write!(f, "magic must be {:?}", MAGIC),
            DecodeError::DescWidthIsZero => write!(f, "decoded header width was 0"),
            DecodeError::DescHeightIsZero => write!(f, "decoded header height was 0"),
            DecodeError::ReadError(e) => write!(f, "read error occured: {}", e),
            DecodeError::WriteError(e) => write!(f, "write error occured: {}", e),
            DecodeError::InvalidCast(e) => write!(f, "invalid cast: {}", e),
        }
    }
}

pub fn encode(data: &[u8], desc: QoiDesc) -> Result<Vec<u8>, EncodeError> {
    if desc.width == 0 {
        return Err(EncodeError::WidthIsZero);
    }

    if desc.height == 0 {
        return Err(EncodeError::HeightIsZero);
    }

    if desc.height >= PIXELS_MAX / desc.width {
        return Err(EncodeError::DimensionsTooLarge);
    }

    let max_size =
        desc.width * desc.height * (desc.channels as u32 + 1) + HEADER_SIZE + PADDING.len() as u32;

    let mut bytes = Cursor::new(Vec::with_capacity(max_size as usize));

    ris_io::write(&mut bytes, &MAGIC)?;
    ris_io::write(&mut bytes, &desc.width.to_be_bytes())?;
    ris_io::write(&mut bytes, &desc.height.to_be_bytes())?;
    ris_io::write(&mut bytes, &[desc.channels as u8, desc.color_space as u8])?;

    let pixels = data;

    let mut index = [Rgba::default(); 64];

    let mut run = 0;
    let mut px_prev = Rgba::from_bytes(&[0, 0, 0, 255]);
    let mut px = px_prev;

    let px_len = desc.width as usize * desc.height as usize * desc.channels as usize;
    let px_end = px_len - desc.channels as usize;
    let channels = desc.channels as u32;

    if px_len != pixels.len() {
        return Err(EncodeError::DataDoesNotMatchDimensions);
    }

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
                ris_io::write(&mut bytes, &[OP_RUN | (run - 1)])?;
                run = 0;
            }
        } else {
            if run > 0 {
                ris_io::write(&mut bytes, &[OP_RUN | (run - 1)])?;
                run = 0;
            }

            let index_pos = px.hash() % 64;

            if index[index_pos as usize] == px {
                ris_io::write(&mut bytes, &[OP_INDEX | index_pos])?;
            } else {
                index[index_pos as usize] = px;

                if px.a == px_prev.a {
                    let vr = (px.r as i8).wrapping_sub(px_prev.r as i8);
                    let vg = (px.g as i8).wrapping_sub(px_prev.g as i8);
                    let vb = (px.b as i8).wrapping_sub(px_prev.b as i8);

                    let vg_r = vr.wrapping_sub(vg);
                    let vg_b = vb.wrapping_sub(vg);

                    if vr > -3 && vr < 2 && vg > -3 && vg < 2 && vb > -3 && vb < 2 {
                        let dr = ((vr + 2) << 4) as u8;
                        let dg = ((vg + 2) << 2) as u8;
                        let db = (vb + 2) as u8;
                        ris_io::write(&mut bytes, &[OP_DIFF | dr | dg | db])?;
                    } else if vg_r > -9 && vg_r < 8 && vg > -33 && vg < 32 && vg_b > -9 && vg_b < 8
                    {
                        let dr = ((vg_r + 8) << 4) as u8;
                        let dg = (vg + 32) as u8;
                        let db = (vg_b + 8) as u8;
                        ris_io::write(&mut bytes, &[OP_LUMA | dg])?;
                        ris_io::write(&mut bytes, &[dr | db])?;
                    } else {
                        ris_io::write(&mut bytes, &[OP_RGB, px.r, px.g, px.b])?;
                    }
                } else {
                    ris_io::write(&mut bytes, &[OP_RGBA, px.r, px.g, px.b, px.a])?;
                }
            }
        }

        px_prev = px;
    }

    ris_io::write(&mut bytes, &PADDING)?;

    let result = bytes.into_inner();
    Ok(result)
}

pub fn decode(data: &[u8], channels: Option<Channels>) -> Result<(Vec<u8>, QoiDesc), DecodeError> {
    if data.len() < DATA_MIN {
        return Err(DecodeError::DataToSmall);
    }

    let mut bytes = &mut Cursor::new(data);

    let mut header_magic_bytes = [0; 4];
    let mut width_bytes = [0; 4];
    let mut height_bytes = [0; 4];
    ris_io::read(&mut bytes, &mut header_magic_bytes)?;
    ris_io::read(&mut bytes, &mut width_bytes)?;
    ris_io::read(&mut bytes, &mut height_bytes)?;

    let width = u32::from_be_bytes(width_bytes);
    let height = u32::from_be_bytes(height_bytes);

    if !ris_util::testing::bytes_eq(&header_magic_bytes, &MAGIC) {
        return Err(DecodeError::IncorrectMagic);
    }

    if width == 0 {
        return Err(DecodeError::DescWidthIsZero);
    }

    if height == 0 {
        return Err(DecodeError::DescHeightIsZero);
    }

    let desc = QoiDesc {
        width,
        height,
        channels: ris_io::read_u8(bytes)?.try_into()?,
        color_space: ris_io::read_u8(bytes)?.try_into()?,
    };

    let channels = match channels {
        Some(x) => x,
        None => desc.channels,
    };

    let px_len = desc.width as usize * desc.height as usize * channels as usize;
    let mut pixels = Cursor::new(Vec::with_capacity(px_len));

    let mut index = [Rgba::default(); 64];
    let mut px = Rgba::from_bytes(&[0, 0, 0, 255]);

    let mut run = 0;

    let chunks_len = data.len() - PADDING.len();
    for _ in (0..px_len).step_by(channels as usize) {
        if run > 0 {
            run -= 1;
        } else if bytes.position() < chunks_len as u64 {
            let b1 = ris_io::read_u8(bytes)?;

            if b1 == OP_RGB {
                px.r = ris_io::read_u8(bytes)?;
                px.g = ris_io::read_u8(bytes)?;
                px.b = ris_io::read_u8(bytes)?;
            } else if b1 == OP_RGBA {
                px.r = ris_io::read_u8(bytes)?;
                px.g = ris_io::read_u8(bytes)?;
                px.b = ris_io::read_u8(bytes)?;
                px.a = ris_io::read_u8(bytes)?;
            } else if (b1 & MASK_2) == OP_INDEX {
                px = index[b1 as usize];
            } else if (b1 & MASK_2) == OP_DIFF {
                px.r = px.r.wrapping_add((b1 >> 4) & 0x03).wrapping_sub(2);
                px.g = px.g.wrapping_add((b1 >> 2) & 0x03).wrapping_sub(2);
                px.b = px.b.wrapping_add(b1 & 0x03).wrapping_sub(2);
            } else if (b1 & MASK_2) == OP_LUMA {
                let b2 = ris_io::read_u8(bytes)?;
                let vg = (b1 & 0x3f).wrapping_sub(32);
                px.r =
                    px.r.wrapping_add(vg.wrapping_sub(8).wrapping_add((b2 >> 4) & 0x0f));
                px.g = px.g.wrapping_add(vg);
                px.b =
                    px.b.wrapping_add(vg.wrapping_sub(8).wrapping_add(b2 & 0x0f));
            } else if (b1 & MASK_2) == OP_RUN {
                run = b1 & 0x3f;
            }

            let index_pos = px.hash() % 64;
            index[index_pos as usize] = px;
        }

        ris_io::write(&mut pixels, &[px.r, px.g, px.b])?;

        if channels == Channels::RGBA {
            ris_io::write(&mut pixels, &[px.a])?;
        }
    }

    let result = pixels.into_inner();
    Ok((result, desc))
}
