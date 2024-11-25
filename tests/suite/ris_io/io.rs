use std::cell::RefCell;
use std::f32::consts::PI;
use std::io::Cursor;
use std::io::SeekFrom;
use std::rc::Rc;

use ris_io::FatPtr;
use ris_math::vector::Bvec2;
use ris_math::vector::Bvec3;
use ris_math::vector::Bvec4;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::assert_bytes_eq;
use ris_util::assert_feq;
use ris_util::assert_quat_eq;
use ris_util::testing;
use ris_util::testing::miri_choose;

use ris_util::testing::*;

#[test]
fn should_compare_bytes() {
    let array1 = [1, 2, 3];
    let array2 = [1, 2, 3];
    let array3 = [];
    let array4 = [];
    let array5 = [1, 2, 4];
    let array6 = [1, 2];
    let array7 = [1, 2, 3, 4];
    let array8 = [4, 5, 6];

    assert!(bytes_eq(&array1, &array2));
    assert!(!bytes_eq(&array1, &array3));
    assert!(!bytes_eq(&array1, &array4));
    assert!(!bytes_eq(&array1, &array5));
    assert!(!bytes_eq(&array1, &array6));
    assert!(!bytes_eq(&array1, &array7));
    assert!(!bytes_eq(&array1, &array8));
    assert!(bytes_eq(&array3, &array4));
}

#[test]
fn should_read_and_write_bytes() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let input = rng.next_bytes(100);
        let mut stream = Cursor::new(Vec::new());
        let mut output = vec![0; input.len()];
        ris_io::write(&mut stream, &input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        ris_io::read(&mut stream, &mut output).unwrap();
        assert_bytes_eq!(input, output);
    });
}

#[test]
fn should_not_read_bytes_when_stream_is_empty() {
    let mut stream = Cursor::new(Vec::new());
    let mut output = vec![0; 5];
    let result = ris_io::read(&mut stream, &mut output);
    assert!(result.is_err());
}

#[test]
fn should_not_read_bytes_when_stream_has_not_enough_bytes() {
    let mut stream = Cursor::new(vec![1, 2, 3, 4, 5]);
    let mut output = vec![0; 5];
    ris_io::seek(&mut stream, SeekFrom::Start(2)).unwrap();
    let result = ris_io::read(&mut stream, &mut output);
    assert!(result.is_err());
}

#[test]
fn should_not_read_bytes_when_stream_is_at_the_end() {
    let mut stream = Cursor::new(vec![1, 2, 3, 4, 5]);
    let mut output = vec![0; 5];
    ris_io::seek(&mut stream, SeekFrom::End(0)).unwrap();
    let result = ris_io::read(&mut stream, &mut output);
    assert!(result.is_err());
}

#[test]
fn should_read_and_write_u8() {
    for input in u8::MAX..u8::MIN {
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_u8(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_u8(&mut stream).unwrap();
        assert_eq!(input, output);
    }
}

#[test]
fn should_read_and_write_int() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let input = rng.next_i32() as isize;
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_int(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_int(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_uint() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let input = rng.next_u32() as usize;
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_uint(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        println!("stream: {:?}", stream);
        let output = ris_io::read_uint(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_u64() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let input0 = rng.next_u32() as u64;
        let input1 = rng.next_u32() as u64;
        let input: u64 = input0 | input1 << 32;
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_u64(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_u64(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_f32() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let input = rng.next_f32();
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_f32(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_f32(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_bool() {
    for input in [true, false] {
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_bool(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_bool(&mut stream).unwrap();
        assert_eq!(input, output);
    }
}

#[test]
fn should_not_read_invalid_bool() {
    let mut stream = Cursor::new(vec![2u8]);
    let result = ris_io::read_bool(&mut stream);
    assert!(result.is_err());
}

#[test]
fn should_read_and_write_fat_ptr() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let addr0 = rng.next_u32() as u64;
        let addr1 = rng.next_u32() as u64;
        let addr = addr0 | addr1 << 32;
        let len0 = rng.next_u32() as u64;
        let len1 = rng.next_u32() as u64;
        let len = len0 | len1 << 32;
        let input = FatPtr {addr, len};
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_fat_ptr(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_fat_ptr(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_string() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let int = rng.next_u32();
        let input = int.to_string();
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_string(&mut stream, &input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_string(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_vec2() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let input = rng.next_pos_2();
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_vec2(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_vec2(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_vec3() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let input = rng.next_pos_3();
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_vec3(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_vec3(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_vec4() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let input = rng.next_pos_4();
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_vec4(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_vec4(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_bvec2() {
    let values = [true, false];
    for x in values {
        for y in values {
            let input = Bvec2(x, y);
            let mut stream = Cursor::new(Vec::new());
            ris_io::write_bvec2(&mut stream, input).unwrap();
            ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
            let output = ris_io::read_bvec2(&mut stream).unwrap();
            assert_eq!(input, output);
        }
    }
}

#[test]
fn should_read_and_write_bvec3() {
    let values = [true, false];
    for x in values {
        for y in values {
            for z in values {
                let input = Bvec3(x, y, z);
                let mut stream = Cursor::new(Vec::new());
                ris_io::write_bvec3(&mut stream, input).unwrap();
                ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
                let output = ris_io::read_bvec3(&mut stream).unwrap();
                assert_eq!(input, output);
            }
        }
    }
}

#[test]
fn should_read_and_write_bvec4() {
    let values = [true, false];
    for x in values {
        for y in values {
            for z in values {
                for w in values {
                    let input = Bvec4(x, y, z, w);
                    let mut stream = Cursor::new(Vec::new());
                    ris_io::write_bvec4(&mut stream, input).unwrap();
                    ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
                    let output = ris_io::read_bvec4(&mut stream).unwrap();
                    assert_eq!(input, output);
                }
            }
        }
    }
}

#[test]
fn should_not_read_invalid_bvec2() {
    let mut stream = Cursor::new(vec![4]);
    let result = ris_io::read_bvec2(&mut stream);
    assert!(result.is_err());
}

#[test]
fn should_not_read_invalid_bvec3() {
    let mut stream = Cursor::new(vec![8]);
    let result = ris_io::read_bvec3(&mut stream);
    assert!(result.is_err());
}

#[test]
fn should_not_read_invalid_bvec4() {
    let mut stream = Cursor::new(vec![16]);
    let result = ris_io::read_bvec4(&mut stream);
    assert!(result.is_err());
}

#[test]
fn should_read_and_write_quat() {
    let seed = Seed::new().unwrap();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let input = rng.next_rot();
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_quat(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_quat(&mut stream).unwrap();
        assert_quat_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_mat2() {
    panic!();
}

#[test]
fn should_read_and_write_mat2x3() {
    panic!();
}

#[test]
fn should_read_and_write_mat2x4() {
    panic!();
}

#[test]
fn should_read_and_write_mat3x2() {
    panic!();
}

#[test]
fn should_read_and_write_mat3() {
    panic!();
}

#[test]
fn should_read_and_write_mat3x4() {
    panic!();
}

#[test]
fn should_read_and_write_mat4x2() {
    panic!();
}

#[test]
fn should_read_and_write_mat4x3() {
    panic!();
}

#[test]
fn should_read_and_write_mat4() {
    panic!();
}

#[test]
fn should_read_and_write_everything() {
    panic!();
}
