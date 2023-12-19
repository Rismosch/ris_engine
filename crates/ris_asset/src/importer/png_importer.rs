use std::io::Read;
use std::io::Seek;
use std::io::Write;

use png::ColorType;

use ris_util::error::RisResult;

use crate::codecs::qoi;
use crate::codecs::qoi::Channels;
use crate::codecs::qoi::ColorSpace;
use crate::codecs::qoi::QoiDesc;

pub const IN_EXT: &str = "png";
pub const OUT_EXT: &str = "qoi";

pub fn import(
    _filename: &str,
    input: &mut (impl Read + Seek),
    output: &mut (impl Write + Seek),
) -> RisResult<()> {
    // decode png
    let decoder = png::Decoder::new(input);
    let mut reader = ris_util::unroll!(decoder.read_info(), "failed to read info",)?;
    let mut pixels = vec![0; reader.output_buffer_size()];
    let info = ris_util::unroll!(reader.next_frame(&mut pixels), "failed to get next frame",)?;

    // encode qoi
    let width = info.width;
    let height = info.height;
    let channels = match info.color_type {
        ColorType::Rgb => Channels::RGB,
        ColorType::Rgba => Channels::RGBA,
        color_type => {
            return ris_util::result_err!(
                "cannot encode qoi. unsupported color type: {:?}",
                color_type
            )
        }
    };
    let color_space = ColorSpace::SRGB;

    let desc = QoiDesc {
        width,
        height,
        channels,
        color_space,
    };

    let encoded = ris_util::unroll!(qoi::encode(&pixels, desc), "failed to encode qoi",)?;

    ris_util::write!(output, &encoded)?;

    Ok(())
}
