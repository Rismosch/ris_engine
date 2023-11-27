#[macro_export]
macro_rules! seek {
    ($file:expr, $pos:expr) => {{
        use std::io::Seek;
        use std::io::SeekFrom;

        ris_util::unroll!($file.seek($pos), "failed to seek")
    }};
}

#[macro_export]
macro_rules! read {
    ($file:expr, $buf:expr) => {{
        use std::io::Read;

        let read_bytes = ris_util::unroll!($file.read(&mut $buf), "failed to read")?;
        let buf_len = $buf.len();
        if read_bytes != buf_len {
            ris_util::result_err!(
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

        let written_bytes = ris_util::unroll!($file.write($buf), "failed to write")?;
        if written_bytes != $buf.len() {
            ris_util::result_err!(
                "expected to write {} bytes but actually wrote {}",
                $buf.len(),
                written_bytes,
            )
        } else {
            Ok(written_bytes)
        }
    }};
}

pub fn bytes_equal(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    for i in 0..left.len() {
        let left = left[i];
        let right = right[i];

        if left != right {
            return false;
        }
    }

    true
}
