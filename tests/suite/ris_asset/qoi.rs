use ris_asset::codecs::qoi;
use ris_asset::codecs::qoi::Channels;
use ris_asset::codecs::qoi::ColorSpace;
use ris_asset::codecs::qoi::QoiDesc;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::testing;
use ris_util::testing::miri_choose;

#[test]
fn should_encode_and_decode_fuzzed() {
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new(Seed::new().unwrap())));
    testing::repeat(miri_choose(10, 1), move |_| {
        let mut rng = rng.borrow_mut();

        let width = rng.range_i(1, 1000) as u32;
        let height = rng.range_i(1, 1000) as u32;
        let channels = rng.range_i(3, 4) as u8;
        let color_space = rng.range_i(0, 1) as u8;

        let data = rng.next_bytes(width as usize * height as usize * channels as usize);

        let desc = QoiDesc {
            width,
            height,
            channels: Channels::try_from(channels).unwrap(),
            color_space: ColorSpace::try_from(color_space).unwrap(),
        };

        let encoded_bytes = qoi::encode(&data, desc).unwrap();
        let (decoded_bytes, decoded_desc) = qoi::decode(&encoded_bytes, None).unwrap();

        assert_eq!(desc, decoded_desc);
        ris_util::assert_bytes_eq!(&data, &decoded_bytes);
    });
}

#[test]
fn further_things_to_test() {
    //let data = [
    //    0x0C, 0x4E, 0x9F, 0x0C, 0x4E, 0x9F, 0xC6, 0x54, 0x6F, 0xDB, 0x1B, 0x87,
    //];
    //let width = 1;
    //let height = data.len() as u32 / 3;
    //let channels = 3;

    // rgba
    // encodeerror:
    //  width is zero
    //  height is zero
    //  dimensions too large
    //  dont forget todo in errors
    panic!();
}
