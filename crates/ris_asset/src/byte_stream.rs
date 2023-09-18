use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;

pub struct ByteStream {
    bytes: Vec<u8>,
    pointer: usize,
}

impl ByteStream {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            pointer: 0,
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl Read for ByteStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let remaining_bytes = self.bytes.len() - self.pointer;
        let byte_count = usize::min(remaining_bytes, buf.len());

        for i in 0..byte_count {
            let pointer = self.pointer + i;
            buf[i] = self.bytes[pointer];
        }

        self.pointer += byte_count;

        Ok(byte_count)
    }
}

impl Write for ByteStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let existing = self.bytes.len() - self.pointer;
        let overwrite = usize::min(existing, buf.len());
        let remaining = buf.len() - overwrite;

        println!("hoi {} {}", overwrite, remaining);

        for i in 0..overwrite {
            let pointer = self.pointer + i;
            self.bytes[pointer] = buf[i]
        }

        for i in 0..remaining {
            let pointer = overwrite + i;
            self.bytes.push(buf[pointer]);
        }

        self.pointer += buf.len();

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Seek for ByteStream {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let pointer = match pos {
            SeekFrom::Start(pos) => {
                pos as isize
            },
            SeekFrom::End(pos) => {
                self.bytes.len() as isize + pos as isize
            },
            SeekFrom::Current(pos) => {
                self.pointer as isize + pos as isize
            },
        };

        self.pointer = if pointer < 0 {
            0
        } else if pointer as usize > self.bytes.len() {
            self.bytes.len()
        } else {
            pointer as usize
        };

        if self.pointer > self.bytes.len() {
            self.pointer = self.bytes.len();
        }

        Ok(self.pointer as u64)
    }
}
