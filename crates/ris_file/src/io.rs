#[macro_export]
macro_rules! seek {
    ($file:expr, $pos:expr) => {{
        use std::io::Seek;
        use std::io::SeekFrom;

        $file.seek($pos)
    }};
}

#[macro_export]
macro_rules! read {
    ($file:expr, $buf:expr) => {{
        use std::io::Read;

        let read_bytes = $file.read(&mut $buf)?;
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

        let written_bytes = $file.write($buf)?;
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
