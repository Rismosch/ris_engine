#[macro_export]
macro_rules! seek {
    ($file:expr, $pos:expr) => {{
        use std::io::Seek;
        use std::io::SeekFrom;

        ris_error::unroll!($file.seek($pos), "failed to seek")
    }};
}

#[macro_export]
macro_rules! read {
    ($file:expr, $buf:expr) => {{
        use std::io::Read;

        let read_bytes = ris_error::unroll!($file.read(&mut $buf), "failed to read")?;
        let buf_len = $buf.len();
        if read_bytes != buf_len {
            ris_error::new_result!(
                "expected to read {} bytes but actually read {}",
                buf_len,
                read_bytes,
            )
        } else {
            Ok(read_bytes)
        }
    }};
}

#[macro_export]
macro_rules! write {
    ($file:expr, $buf:expr) => {{
        use std::io::Write;

        let written_bytes = ris_error::unroll!($file.write($buf), "failed to write")?;
        if written_bytes != $buf.len() {
            ris_error::new_result!(
                "expected to write {} bytes but actually wrote {}",
                $buf.len(),
                written_bytes,
            )
        } else {
            Ok(written_bytes)
        }
    }};
}
