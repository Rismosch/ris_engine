use ris_asset::codecs::qoi;
use ris_asset::codecs::qoi::Channels;
use ris_asset::codecs::qoi::ColorSpace;
use ris_asset::codecs::qoi::QoiDesc;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::testing;
use ris_util::testing::miri_choose;

#[test]
fn should_encode_and_decode_rgb(){
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new(Seed::new().unwrap())));
    testing::repeat(miri_choose(1, 1), move |_| {
        let mut rng = rng.borrow_mut();

        let width = rng.range_i(0, 2000) as usize;
        let height = rng.range_i(0, 2000) as usize;
        let channels = Channels::RGB;
        let color_space = ColorSpace::SRGB;

        let data_len = width * height * 3;

        let desc = QoiDesc {width, height, channels, color_space};
        let mut data = vec![0; data_len];
        for i in 0..data_len {
            let random_value = rng.next_u();
            let random_byte = (0xFF & random_value) as u8;
            data[i] = random_byte;
        }

        let encoded = qoi::encode(&data, desc);
        let decoded = qoi::decode();

        panic!("reached end of test");
    });
}

#[test]
fn should_encode_and_decode_rgba(){
    panic!();
}
