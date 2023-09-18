use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;

use ris_asset::byte_stream::ByteStream;
use ris_asset::util;

#[test]
fn should_seek_from_start() {
    let bytes = vec![0,1,2,3,4,5,6,7,8,9];
    let mut stream = ByteStream::new(bytes);

    let result = stream.seek(SeekFrom::Start(3)).unwrap();
    assert_eq!(result, 3);
    let result = stream.seek(SeekFrom::Start(10)).unwrap();
    assert_eq!(result, 10);
    let result = stream.seek(SeekFrom::Start(6)).unwrap();
    assert_eq!(result, 6);
    let result = stream.seek(SeekFrom::Start(123)).unwrap();
    assert_eq!(result, 10);
    let result = stream.seek(SeekFrom::Start(0)).unwrap();
    assert_eq!(result, 0);
}

#[test]
fn should_seek_from_end() {
    let bytes = vec![0,1,2,3,4,5,6,7,8,9];
    let mut stream = ByteStream::new(bytes);

    let result = stream.seek(SeekFrom::End(-5)).unwrap();
    assert_eq!(result, 5);
    let result = stream.seek(SeekFrom::End(-10)).unwrap();
    assert_eq!(result, 0);
    let result = stream.seek(SeekFrom::End(5)).unwrap();
    assert_eq!(result, 10);
    let result = stream.seek(SeekFrom::End(-123)).unwrap();
    assert_eq!(result, 0);
    let result = stream.seek(SeekFrom::End(0)).unwrap();
    assert_eq!(result, 10);
}

#[test]
fn should_seek_from_current() {
    let bytes = vec![0,1,2,3,4,5,6,7,8,9];
    let mut stream = ByteStream::new(bytes);

    let result = stream.seek(SeekFrom::Current(-5)).unwrap();
    assert_eq!(result, 0);
    let result = stream.seek(SeekFrom::Current(5)).unwrap();
    assert_eq!(result, 5);
    let result = stream.seek(SeekFrom::Current(-3)).unwrap();
    assert_eq!(result, 2);
    let result = stream.seek(SeekFrom::Current(8)).unwrap();
    assert_eq!(result, 10);
    let result = stream.seek(SeekFrom::Current(123)).unwrap();
    assert_eq!(result, 10);
    let result = stream.seek(SeekFrom::Current(-123)).unwrap();
    assert_eq!(result, 0);
}

#[test]
fn should_read() {
    let bytes = vec![0,1,2,3,4,5,6,7,8,9];
    let mut stream = ByteStream::new(bytes);

    let mut result = [0; 10];
    let count = stream.read(&mut result).unwrap();
    assert_eq!(count, 10);
    assert!(util::bytes_equal(&result, &[0,1,2,3,4,5,6,7,8,9]));

    stream.seek(SeekFrom::Start(3)).unwrap();
    let mut result = [0; 5];
    let count = stream.read(&mut result).unwrap();
    assert_eq!(count, 5);
    assert!(util::bytes_equal(&result, &[3,4,5,6,7]));

    stream.seek(SeekFrom::End(0)).unwrap();
    let mut result = [0; 5];
    let count = stream.read(&mut result).unwrap();
    assert_eq!(count, 0);
    assert!(util::bytes_equal(&result, &[0,0,0,0,0]));
    
    stream.seek(SeekFrom::End(-2)).unwrap();
    let mut result = [0; 5];
    let count = stream.read(&mut result).unwrap();
    assert_eq!(count, 2);
    assert!(util::bytes_equal(&result, &[8,9,0,0,0]));
}

#[test]
fn should_write_nothing() {
    let bytes = vec![];
    let mut stream = ByteStream::new(bytes);

    let count = stream.write(&[]).unwrap();
    let result = stream.to_bytes();
    assert_eq!(count, 0);
    assert!(util::bytes_equal(&result, &[]));
}

#[test]
fn should_append_on_write() {
    let bytes = vec![0,1,2,3,4];
    let mut stream = ByteStream::new(bytes);

    stream.seek(SeekFrom::End(0)).unwrap();
    let count = stream.write(&[5,6,7,8,9]).unwrap();
    let result = stream.to_bytes();
    assert_eq!(count, 5);
    assert!(util::bytes_equal(&result, &[0,1,2,3,4,5,6,7,8,9]));
}

#[test]
fn should_overwrite() {
    let bytes = vec![0,1,2,3,4,5,6,7,8,9];
    let mut stream = ByteStream::new(bytes);

    stream.seek(SeekFrom::Start(3)).unwrap();
    let count = stream.write(&[5,6,7,8,9]).unwrap();
    let result = stream.to_bytes();
    assert_eq!(count, 5);
    assert!(util::bytes_equal(&result, &[0,1,2,5,6,7,8,9,8,9]));
}

#[test]
fn should_overwrite_and_append() {
    let bytes = vec![0,1,2,3,4];
    let mut stream = ByteStream::new(bytes);

    stream.seek(SeekFrom::Start(2)).unwrap();
    let count = stream.write(&[5,6,7,8,9]).unwrap();
    let result = stream.to_bytes();
    assert_eq!(count, 5);
    assert!(util::bytes_equal(&result, &[0,1,5,6,7,8,9]), "result: {:?}", result);
}
