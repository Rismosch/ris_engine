use std::io::Read;
use std::io::Seek;
use std::io::Write;

use ris_util::error::RisResult;

pub const IN_EXT: &str = "png";
pub const OUT_EXT: &str = "qoi";

pub fn import(
    _filename: &str,
    _input: &mut (impl Read + Seek),
    _output: &mut (impl Write + Seek),
) -> RisResult<()> {
    panic!("not implemented");

    // decode png
    //let decoder = png::Decoder::new(input);
    //let mut reader = ris_util::unroll!(decoder.read_info(), "failed to read info",)?;
    //let mut buf = vec![0; reader.output_buffer_size()];
    //let info = ris_util::unroll!(reader.next_frame(&mut buf), "failed to get next frame",)?;

    //let bytes = &buf[..9];

    //ris_util::result_err!("not implemented. oha: {:x?}", bytes)
}
