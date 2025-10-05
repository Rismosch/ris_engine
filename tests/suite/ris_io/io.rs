use std::cell::RefCell;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::rc::Rc;

use ris_io::FatPtr;
use ris_math::matrix::Mat2;
use ris_math::matrix::Mat2x3;
use ris_math::matrix::Mat2x4;
use ris_math::matrix::Mat3;
use ris_math::matrix::Mat3x2;
use ris_math::matrix::Mat3x4;
use ris_math::matrix::Mat4;
use ris_math::matrix::Mat4x2;
use ris_math::matrix::Mat4x3;
use ris_math::vector::Bvec2;
use ris_math::vector::Bvec3;
use ris_math::vector::Bvec4;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::assert_bytes_eq;
use ris_util::testing;
use ris_util::testing::miri_choose;

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

    assert!(ris_util::testing::bytes_eq(&array1, &array2));
    assert!(!ris_util::testing::bytes_eq(&array1, &array3));
    assert!(!ris_util::testing::bytes_eq(&array1, &array4));
    assert!(!ris_util::testing::bytes_eq(&array1, &array5));
    assert!(!ris_util::testing::bytes_eq(&array1, &array6));
    assert!(!ris_util::testing::bytes_eq(&array1, &array7));
    assert!(!ris_util::testing::bytes_eq(&array1, &array8));
    assert!(ris_util::testing::bytes_eq(&array3, &array4));
}

#[test]
fn should_read_and_write_bytes() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let mut input = [0; 100];
        rng.next_u8s(&mut input);
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
    for input in u8::MIN..u8::MAX {
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_u8(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_u8(&mut stream).unwrap();
        assert_eq!(input, output);
    }
}

#[test]
fn should_read_and_write_int() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
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
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
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
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let input0 = rng.next_u32() as u64;
        let input1 = rng.next_u32() as u64;
        let input: u64 = input0 | (input1 << 32);
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_u64(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_u64(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_f32() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
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
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let addr0 = rng.next_u32() as u64;
        let addr1 = rng.next_u32() as u64;
        let addr = addr0 | (addr1 << 32);
        let len0 = rng.next_u32() as u64;
        let len1 = rng.next_u32() as u64;
        let len = len0 | (len1 << 32);
        let input = FatPtr { addr, len };
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_fat_ptr(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_fat_ptr(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_string() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
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
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
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
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
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
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
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
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let input = rng.next_rot();
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_quat(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_quat(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_mat2() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let col0 = rng.next_pos_2();
        let col1 = rng.next_pos_2();
        let input = Mat2(col0, col1);
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_mat2(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_mat2(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_mat2x3() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let col0 = rng.next_pos_3();
        let col1 = rng.next_pos_3();
        let input = Mat2x3(col0, col1);
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_mat2x3(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_mat2x3(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_mat2x4() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let col0 = rng.next_pos_4();
        let col1 = rng.next_pos_4();
        let input = Mat2x4(col0, col1);
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_mat2x4(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_mat2x4(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_mat3x2() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let col0 = rng.next_pos_2();
        let col1 = rng.next_pos_2();
        let col2 = rng.next_pos_2();
        let input = Mat3x2(col0, col1, col2);
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_mat3x2(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_mat3x2(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_mat3() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let col0 = rng.next_pos_3();
        let col1 = rng.next_pos_3();
        let col2 = rng.next_pos_3();
        let input = Mat3(col0, col1, col2);
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_mat3(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_mat3(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_mat3x4() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let col0 = rng.next_pos_4();
        let col1 = rng.next_pos_4();
        let col2 = rng.next_pos_4();
        let input = Mat3x4(col0, col1, col2);
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_mat3x4(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_mat3x4(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_mat4x2() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let col0 = rng.next_pos_2();
        let col1 = rng.next_pos_2();
        let col2 = rng.next_pos_2();
        let col3 = rng.next_pos_2();
        let input = Mat4x2(col0, col1, col2, col3);
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_mat4x2(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_mat4x2(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_mat4x3() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let col0 = rng.next_pos_3();
        let col1 = rng.next_pos_3();
        let col2 = rng.next_pos_3();
        let col3 = rng.next_pos_3();
        let input = Mat4x3(col0, col1, col2, col3);
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_mat4x3(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_mat4x3(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_mat4() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();
        let col0 = rng.next_pos_4();
        let col1 = rng.next_pos_4();
        let col2 = rng.next_pos_4();
        let col3 = rng.next_pos_4();
        let input = Mat4(col0, col1, col2, col3);
        let mut stream = Cursor::new(Vec::new());
        ris_io::write_mat4(&mut stream, input).unwrap();
        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();
        let output = ris_io::read_mat4(&mut stream).unwrap();
        assert_eq!(input, output);
    });
}

#[test]
fn should_read_and_write_everything_via_fat_ptrs() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let rng = Rc::new(RefCell::new(Rng::new(seed)));

    testing::repeat(miri_choose(1_000, 10), move |_| {
        let mut rng = rng.borrow_mut();

        let mut input_bytes = [0; 10];
        rng.next_u8s(&mut input_bytes);
        let input_u8 = rng.next_u8();
        let input_int = rng.next_i32() as isize;
        let input_uint = rng.next_u32() as usize;
        let input_u64_0 = rng.next_u32() as u64;
        let input_u64_1 = rng.next_u32() as u64;
        let input_u64 = input_u64_0 | input_u64_1;
        let input_f32 = rng.next_f32();
        let input_bool = rng.next_bool();
        let addr0 = rng.next_u32() as u64;
        let addr1 = rng.next_u32() as u64;
        let addr = addr0 | (addr1 << 32);
        let len0 = rng.next_u32() as u64;
        let len1 = rng.next_u32() as u64;
        let len = len0 | (len1 << 32);
        let input_fat_ptr = FatPtr { addr, len };
        let input_string = rng.next_u32().to_string();
        let input_vec2 = rng.next_pos_2();
        let input_vec3 = rng.next_pos_3();
        let input_vec4 = rng.next_pos_4();
        let input_bvec2 = Bvec2(rng.next_bool(), rng.next_bool());
        let input_bvec3 = Bvec3(rng.next_bool(), rng.next_bool(), rng.next_bool());
        let input_bvec4 = Bvec4(
            rng.next_bool(),
            rng.next_bool(),
            rng.next_bool(),
            rng.next_bool(),
        );
        let input_quat = rng.next_rot();
        let input_mat2 = Mat2(rng.next_pos_2(), rng.next_pos_2());
        let input_mat2x3 = Mat2x3(rng.next_pos_3(), rng.next_pos_3());
        let input_mat2x4 = Mat2x4(rng.next_pos_4(), rng.next_pos_4());
        let input_mat3x2 = Mat3x2(rng.next_pos_2(), rng.next_pos_2(), rng.next_pos_2());
        let input_mat3 = Mat3(rng.next_pos_3(), rng.next_pos_3(), rng.next_pos_3());
        let input_mat3x4 = Mat3x4(rng.next_pos_4(), rng.next_pos_4(), rng.next_pos_4());
        let input_mat4x2 = Mat4x2(
            rng.next_pos_2(),
            rng.next_pos_2(),
            rng.next_pos_2(),
            rng.next_pos_2(),
        );
        let input_mat4x3 = Mat4x3(
            rng.next_pos_3(),
            rng.next_pos_3(),
            rng.next_pos_3(),
            rng.next_pos_3(),
        );
        let input_mat4 = Mat4(
            rng.next_pos_4(),
            rng.next_pos_4(),
            rng.next_pos_4(),
            rng.next_pos_4(),
        );

        let mut stream = Cursor::new(Vec::new());
        let ptr_bytes = ris_io::write(&mut stream, &input_bytes).unwrap();
        let ptr_u8 = ris_io::write_u8(&mut stream, input_u8).unwrap();
        let ptr_int = ris_io::write_int(&mut stream, input_int).unwrap();
        let ptr_uint = ris_io::write_uint(&mut stream, input_uint).unwrap();
        let ptr_u64 = ris_io::write_u64(&mut stream, input_u64).unwrap();
        let ptr_f32 = ris_io::write_f32(&mut stream, input_f32).unwrap();
        let ptr_bool = ris_io::write_bool(&mut stream, input_bool).unwrap();
        let ptr_fat_ptr = ris_io::write_fat_ptr(&mut stream, input_fat_ptr).unwrap();
        let ptr_string = ris_io::write_string(&mut stream, &input_string).unwrap();
        let ptr_vec2 = ris_io::write_vec2(&mut stream, input_vec2).unwrap();
        let ptr_vec3 = ris_io::write_vec3(&mut stream, input_vec3).unwrap();
        let ptr_vec4 = ris_io::write_vec4(&mut stream, input_vec4).unwrap();
        let ptr_bvec2 = ris_io::write_bvec2(&mut stream, input_bvec2).unwrap();
        let ptr_bvec3 = ris_io::write_bvec3(&mut stream, input_bvec3).unwrap();
        let ptr_bvec4 = ris_io::write_bvec4(&mut stream, input_bvec4).unwrap();
        let ptr_quat = ris_io::write_quat(&mut stream, input_quat).unwrap();
        let ptr_mat2 = ris_io::write_mat2(&mut stream, input_mat2).unwrap();
        let ptr_mat2x3 = ris_io::write_mat2x3(&mut stream, input_mat2x3).unwrap();
        let ptr_mat2x4 = ris_io::write_mat2x4(&mut stream, input_mat2x4).unwrap();
        let ptr_mat3x2 = ris_io::write_mat3x2(&mut stream, input_mat3x2).unwrap();
        let ptr_mat3 = ris_io::write_mat3(&mut stream, input_mat3).unwrap();
        let ptr_mat3x4 = ris_io::write_mat3x4(&mut stream, input_mat3x4).unwrap();
        let ptr_mat4x2 = ris_io::write_mat4x2(&mut stream, input_mat4x2).unwrap();
        let ptr_mat4x3 = ris_io::write_mat4x3(&mut stream, input_mat4x3).unwrap();
        let ptr_mat4 = ris_io::write_mat4(&mut stream, input_mat4).unwrap();

        ris_io::seek(&mut stream, SeekFrom::Start(0)).unwrap();

        let output_bytes = test_read(&mut rng, &mut stream, ptr_bytes, |s| {
            let mut bytes = Vec::new();
            s.read_to_end(&mut bytes).unwrap();
            bytes
        });
        let output_u8 = test_read(&mut rng, &mut stream, ptr_u8, ris_io::read_u8).unwrap();
        let output_int = test_read(&mut rng, &mut stream, ptr_int, ris_io::read_int).unwrap();
        let output_uint = test_read(&mut rng, &mut stream, ptr_uint, ris_io::read_uint).unwrap();
        let output_u64 = test_read(&mut rng, &mut stream, ptr_u64, ris_io::read_u64).unwrap();
        let output_f32 = test_read(&mut rng, &mut stream, ptr_f32, ris_io::read_f32).unwrap();
        let output_bool = test_read(&mut rng, &mut stream, ptr_bool, ris_io::read_bool).unwrap();
        let output_fat_ptr =
            test_read(&mut rng, &mut stream, ptr_fat_ptr, ris_io::read_fat_ptr).unwrap();
        let output_string =
            test_read(&mut rng, &mut stream, ptr_string, ris_io::read_string).unwrap();
        let output_vec2 = test_read(&mut rng, &mut stream, ptr_vec2, ris_io::read_vec2).unwrap();
        let output_vec3 = test_read(&mut rng, &mut stream, ptr_vec3, ris_io::read_vec3).unwrap();
        let output_vec4 = test_read(&mut rng, &mut stream, ptr_vec4, ris_io::read_vec4).unwrap();
        let output_bvec2 = test_read(&mut rng, &mut stream, ptr_bvec2, ris_io::read_bvec2).unwrap();
        let output_bvec3 = test_read(&mut rng, &mut stream, ptr_bvec3, ris_io::read_bvec3).unwrap();
        let output_bvec4 = test_read(&mut rng, &mut stream, ptr_bvec4, ris_io::read_bvec4).unwrap();
        let output_quat = test_read(&mut rng, &mut stream, ptr_quat, ris_io::read_quat).unwrap();
        let output_mat2 = test_read(&mut rng, &mut stream, ptr_mat2, ris_io::read_mat2).unwrap();
        let output_mat2x3 =
            test_read(&mut rng, &mut stream, ptr_mat2x3, ris_io::read_mat2x3).unwrap();
        let output_mat2x4 =
            test_read(&mut rng, &mut stream, ptr_mat2x4, ris_io::read_mat2x4).unwrap();
        let output_mat3x2 =
            test_read(&mut rng, &mut stream, ptr_mat3x2, ris_io::read_mat3x2).unwrap();
        let output_mat3 = test_read(&mut rng, &mut stream, ptr_mat3, ris_io::read_mat3).unwrap();
        let output_mat3x4 =
            test_read(&mut rng, &mut stream, ptr_mat3x4, ris_io::read_mat3x4).unwrap();
        let output_mat4x2 =
            test_read(&mut rng, &mut stream, ptr_mat4x2, ris_io::read_mat4x2).unwrap();
        let output_mat4x3 =
            test_read(&mut rng, &mut stream, ptr_mat4x3, ris_io::read_mat4x3).unwrap();
        let output_mat4 = test_read(&mut rng, &mut stream, ptr_mat4, ris_io::read_mat4).unwrap();

        assert_bytes_eq!(input_bytes, output_bytes);
        assert_eq!(input_u8, output_u8);
        assert_eq!(input_int, output_int);
        assert_eq!(input_uint, output_uint);
        assert_eq!(input_u64, output_u64);
        assert_eq!(input_f32, output_f32);
        assert_eq!(input_bool, output_bool);
        assert_eq!(input_fat_ptr, output_fat_ptr);
        assert_eq!(input_string, output_string);
        assert_eq!(input_vec2, output_vec2);
        assert_eq!(input_vec3, output_vec3);
        assert_eq!(input_vec4, output_vec4);
        assert_eq!(input_bvec2, output_bvec2);
        assert_eq!(input_bvec3, output_bvec3);
        assert_eq!(input_bvec4, output_bvec4);
        assert_eq!(input_quat, output_quat);
        assert_eq!(input_mat2, output_mat2);
        assert_eq!(input_mat2x3, output_mat2x3);
        assert_eq!(input_mat2x4, output_mat2x4);
        assert_eq!(input_mat3x2, output_mat3x2);
        assert_eq!(input_mat3, output_mat3);
        assert_eq!(input_mat3x4, output_mat3x4);
        assert_eq!(input_mat4x2, output_mat4x2);
        assert_eq!(input_mat4x3, output_mat4x3);
        assert_eq!(input_mat4, output_mat4);
    });
}

fn test_read<T>(
    rng: &mut Rng,
    stream: &mut (impl Read + Seek),
    ptr: FatPtr,
    callback: impl Fn(&mut Cursor<Vec<u8>>) -> T,
) -> T {
    // do a random seek, to ensure that `read_at()` is reading at the correct location
    let end = ris_io::seek(stream, SeekFrom::End(0)).unwrap();
    let random_addr = rng.next_i32_between(0, end as i32) as i64;
    ris_io::seek(stream, SeekFrom::Current(random_addr)).unwrap();

    // read and pass the read bytes to the callback
    let bytes = ris_io::read_at(stream, ptr).unwrap();
    let mut byte_stream = Cursor::new(bytes);
    let result = callback(&mut byte_stream);

    // assert that all bytes were actually read
    let current_addr = ris_io::seek(&mut byte_stream, SeekFrom::Current(0)).unwrap();
    let end_addr = ris_io::seek(&mut byte_stream, SeekFrom::End(0)).unwrap();
    assert_eq!(current_addr, end_addr);

    result
}
