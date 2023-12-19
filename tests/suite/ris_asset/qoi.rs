use std::io::Write;
use std::path::PathBuf;

use ris_asset::codecs::qoi;
use ris_asset::codecs::qoi::Channels;
use ris_asset::codecs::qoi::ColorSpace;
use ris_asset::codecs::qoi::QoiDesc;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::testing;
use ris_util::testing::miri_choose;

#[test]
fn should_encode_and_decode_rgb() {
    //let test_dir = ris_util::prep_test_dir!();
    //let mut source_path = PathBuf::from(&test_dir);
    //source_path.push("source.txt");
    //let mut target_path = PathBuf::from(&test_dir);
    //target_path.push("target.txt");

    //let mut source_file = std::rc::Rc::new(std::cell::RefCell::new(
    //    std::fs::File::create(source_path).unwrap(),
    //));
    //let mut target_file = std::rc::Rc::new(std::cell::RefCell::new(
    //    std::fs::File::create(target_path).unwrap(),
    //));

    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new(Seed::new().unwrap())));
    testing::repeat(miri_choose(1000, 1), move |_| {
        let mut rng = rng.borrow_mut();

        let width = rng.range_i(1, 2000) as u32;
        let height = rng.range_i(1, 2000) as u32;
        let channels = rng.range_i(3, 4) as u8;
        let color_space = rng.range_i(0, 1) as u8;

        let data = rng.next_bytes(width as usize * height as usize * channels as usize);
        //let data = [
        //    0x0C, 0x4E, 0x9F, 0x0C, 0x4E, 0x9F, 0xC6, 0x54, 0x6F, 0xDB, 0x1B, 0x87, 0x66, 0x65,
        //    0xC5, 0x49, 0x61, 0x42, 0x5E, 0x42, 0x5C, 0x51, 0x5D, 0xCF, 0x8C, 0x73, 0x01, 0x82,
        //    0x22, 0xB6,
        //];
        //let data = [
        //    0x0C, 0x4E, 0x9F, 0x0C, 0x4E, 0x9F, 0xC6, 0x54, 0x6F, 0xDB, 0x1B, 0x87,
        //];
        //let width = 1;
        //let height = data.len() as u32 / 3;
        //let channels = 3;

        let desc = QoiDesc {
            width,
            height,
            channels: Channels::try_from(channels).unwrap(),
            color_space: ColorSpace::try_from(color_space).unwrap(),
        };

        let encoded_bytes = qoi::encode(&data, desc).unwrap();
        let (decoded_bytes, decoded_desc) = qoi::decode(&encoded_bytes, None).unwrap();

        assert_eq!(desc, decoded_desc);

        if !ris_util::testing::bytes_eq(&data, &decoded_bytes) {
            for i in 0..data.len() {
                let source_str = format!("0x{:02X}", data[i]);
                let target_str = format!("0x{:02X}", decoded_bytes[i]);

                println!("{:02} {} {}", i, source_str, target_str);

                //source_file.borrow_mut().write(source_str.as_bytes());
                //target_file.borrow_mut().write(target_str.as_bytes());
            }
            ris_util::assert_bytes_eq!(&data, &decoded_bytes);
            panic!("bruh i wrote something into my file");
        }
        ris_util::assert_bytes_eq!(&data, &decoded_bytes);
    });
}

#[test]
fn further_things_to_test() {
    // rgba
    // encodeerror:
    //  width is zero
    //  height is zero
    //  dimensions too large
    //  dont forget todo in errors
    panic!();
}
