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
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new(Seed::new().unwrap())));
    testing::repeat(miri_choose(100, 1), move |_| {
        let mut rng = rng.borrow_mut();

        let width = rng.range_i(1, 100) as u32;
        let height = rng.range_i(1, 100) as u32;
        let channels = rng.range_i(3, 5) as u8;
        let color_space = rng.range_i(0, 2) as u8;

        let data_len = width * height * channels as u32;

        let desc = QoiDesc {
            width,
            height,
            channels: Channels::try_from(channels).unwrap(),
            color_space: ColorSpace::try_from(color_space).unwrap(),
        };

        let mut data = rng.next_bytes(data_len as usize);

        let encoded_bytes = qoi::encode(&data, desc).unwrap();
        let (decoded_desc, decoded_bytes) = qoi::decode(&encoded_bytes, None).unwrap();

        assert_eq!(desc, decoded_desc);
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
