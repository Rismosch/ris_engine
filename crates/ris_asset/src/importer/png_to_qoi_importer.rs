use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use png::ColorType;

use ris_error::Extensions;
use ris_error::RisResult;

use crate::codecs::qoi;
use crate::codecs::qoi::Channels;
use crate::codecs::qoi::ColorSpace;
use crate::codecs::qoi::QoiDesc;

pub const IN_EXT_PNG: &str = "png";
pub const OUT_EXT_QOI: &str = "qoi";

pub fn import(source: impl AsRef<Path>, target_dir: impl AsRef<Path>) -> RisResult<()> {
    let source = source.as_ref();
    let target_dir = target_dir.as_ref();

    // open file
    let input = File::open(source)?;

    // decode png
    let decoder = png::Decoder::new(input);
    let mut reader = decoder.read_info()?;
    let mut pixels = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut pixels)?;

    // encode qoi
    let width = info.width;
    let height = info.height;
    let channels = match info.color_type {
        ColorType::Rgb => Channels::RGB,
        ColorType::Rgba => Channels::RGBA,
        color_type => {
            return ris_error::new_result!(
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

    let encoded = qoi::encode(&pixels, desc)?;

    let mut output = crate::asset_importer::create_file(
        source,
        target_dir,
        OUT_EXT_QOI,
    )?;
    ris_io::write(&mut output, &encoded)?;

    Ok(())
}
