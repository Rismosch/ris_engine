use ris_error::RisResult;

pub fn add_alpha_channel(pixels: &[u8]) -> RisResult<Vec<u8>> {
    ris_error::assert!(pixels.len() % 3 == 0)?;
    let pixels_rgba_len = (pixels.len() * 4) / 3;
    let mut pixels_rgba = Vec::with_capacity(pixels_rgba_len);

    for chunk in pixels.chunks_exact(3) {
        let r = chunk[0];
        let g = chunk[1];
        let b = chunk[2];
        let a = u8::MAX;

        pixels_rgba.push(r);
        pixels_rgba.push(g);
        pixels_rgba.push(b);
        pixels_rgba.push(a);
    }

    Ok(pixels_rgba)
}
