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
    testing::repeat(miri_choose(1, 1), move |_| {
        let mut rng = rng.borrow_mut();

        let width = rng.range_i(1, 100) as u32;
        let height = rng.range_i(1, 100) as u32;
        let channels = Channels::RGB;
        let color_space = ColorSpace::SRGB;

        let data_len = (width * height * 3) as usize;

        let desc = QoiDesc {
            width,
            height,
            channels,
            color_space,
        };
        println!("0 {:?}", desc);

        let mut data = vec![0; data_len];
        for i in 0..data_len {
            let random_value = rng.next_u();
            let random_byte = (0xFF & random_value) as u8;
            data[i] = random_byte;
        }

        let encoded = qoi::encode(&data, desc).unwrap();
        let decoded = qoi::decode(&encoded, desc.channels);

        panic!("{:?}", encoded);
    });
}

#[test]
fn further_things_to_test() {
    // rgba
    // encodeerror:
    //  width is zero
    //  height is zero
    //  dimensions too large
    panic!();
}
