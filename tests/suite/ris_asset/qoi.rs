use ris_asset::codecs::qoi;
use ris_asset::codecs::qoi::Channels;
use ris_asset::codecs::qoi::ColorSpace;
use ris_asset::codecs::qoi::DecodeErrorKind;
use ris_asset::codecs::qoi::EncodeErrorKind;
use ris_asset::codecs::qoi::QoiDesc;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::testing;
use ris_util::testing::miri_choose;

#[test]
fn should_encode_and_decode_fuzzed() {
    let rng = std::rc::Rc::new(std::cell::RefCell::new(Rng::new(Seed::new().unwrap())));
    testing::repeat(10, move |_| {
        let mut rng = rng.borrow_mut();

        let width = rng.range_i(1, miri_choose(1000, 10)) as u32;
        let height = rng.range_i(1, miri_choose(1000, 10)) as u32;
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
#[cfg(not(miri))]
fn should_encode_and_decode_raw_assets() {
    let executable_string = std::env::args().next().expect("no cli args");
    let executable_path = std::path::PathBuf::from(executable_string);
    let executable_directory = executable_path.parent().expect("executable has no parent");
    let mut raw_assets_directory = std::path::PathBuf::from(executable_directory);
    raw_assets_directory.push("..");
    raw_assets_directory.push("..");
    raw_assets_directory.push("..");
    raw_assets_directory.push("assets/__raw");

    let mut pngs = Vec::new();
    let mut directories = std::collections::VecDeque::new();
    directories.push_back(raw_assets_directory);

    while let Some(current) = directories.pop_front() {
        let entries = std::fs::read_dir(&current).unwrap();

        for entry in entries {
            let entry = entry.unwrap();
            let metadata = entry.metadata().unwrap();

            let entry_path = entry.path();
            if metadata.is_file() {
                let extension = entry_path
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_lowercase();
                if extension == "png" {
                    pngs.push(entry_path);
                }
            } else if metadata.is_dir() {
                directories.push_back(entry_path);
            } else {
                panic!("unsupported file: \"{:?}\"", entry_path);
            }
        }
    }

    for png in pngs {
        // decode png
        let decoder = png::Decoder::new(std::fs::File::open(png).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut original = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut original).unwrap();

        // encode/decode qoi
        let width = info.width;
        let height = info.height;
        let channels = match info.color_type {
            png::ColorType::Rgb => Channels::RGB,
            png::ColorType::Rgba => Channels::RGBA,
            color_type => panic!("unsupported format: {:?}", color_type),
        };
        let color_space = ColorSpace::SRGB;

        let desc = QoiDesc {
            width,
            height,
            channels,
            color_space,
        };

        let encoded = qoi::encode(&original, desc).unwrap();
        let (copy, copy_desc) = qoi::decode(&encoded, None).unwrap();

        assert_eq!(desc, copy_desc);
        ris_util::assert_bytes_eq!(&original, &copy);
    }
}

#[test]
fn should_not_encode_when_width_is_zero() {
    let desc = QoiDesc {
        width: 0,
        height: 10,
        channels: Channels::RGB,
        color_space: ColorSpace::SRGB,
    };

    let data = [];
    let error = qoi::encode(&data, desc).unwrap_err();

    assert!(matches!(error.kind, EncodeErrorKind::WidthIsZero));
}

#[test]
fn should_not_encode_when_height_is_zero() {
    let desc = QoiDesc {
        width: 10,
        height: 0,
        channels: Channels::RGB,
        color_space: ColorSpace::SRGB,
    };

    let data = [];
    let error = qoi::encode(&data, desc).unwrap_err();

    assert!(matches!(error.kind, EncodeErrorKind::HeightIsZero));
}

#[test]
fn should_not_encode_when_dimensions_are_too_large() {
    let desc = QoiDesc {
        width: 20000,
        height: 20000,
        channels: Channels::RGBA,
        color_space: ColorSpace::SRGB,
    };

    let data = [];
    let error = qoi::encode(&data, desc).unwrap_err();

    assert!(matches!(error.kind, EncodeErrorKind::DimensionsTooLarge));
}

#[test]
fn should_not_encode_when_data_does_not_match_dimensions() {
    let desc = QoiDesc {
        width: 1,
        height: 2,
        channels: Channels::RGB,
        color_space: ColorSpace::SRGB,
    };

    let data = [1, 2, 3, 4, 5];
    let error = qoi::encode(&data, desc).unwrap_err();

    assert!(matches!(
        error.kind,
        EncodeErrorKind::DataDoesNotMatchDimensions
    ));
}

#[test]
fn should_not_decode_when_data_is_too_small() {
    let data = [0; 21];

    let error = qoi::decode(&data, None).unwrap_err();
    assert!(matches!(error.kind, DecodeErrorKind::DataToSmall));
}

#[test]
fn should_not_decode_when_magic_is_incorrect() {
    let data = [0; 22];

    let error = qoi::decode(&data, None).unwrap_err();
    println!("{:?}", error.kind);
    assert!(matches!(error.kind, DecodeErrorKind::IncorrectMagic));
}

#[test]
fn should_not_decode_when_desc_width_is_zero() {
    let mut data = [0; 22];
    data[0] = 0x71;
    data[1] = 0x6f;
    data[2] = 0x69;
    data[3] = 0x66;
    data[21] = 0x01;

    let error = qoi::decode(&data, None).unwrap_err();
    println!("{:?}", error.kind);
    assert!(matches!(error.kind, DecodeErrorKind::DescWidthIsZero));
}

#[test]
fn should_not_decode_when_height_is_zero() {
    let mut data = [0; 22];
    data[0] = 0x71;
    data[1] = 0x6f;
    data[2] = 0x69;
    data[3] = 0x66;
    data[6] = 0x7D;
    data[21] = 0x01;

    let error = qoi::decode(&data, None).unwrap_err();
    println!("{:?}", error.kind);
    assert!(matches!(error.kind, DecodeErrorKind::DescHeightIsZero));
}

#[test]
fn should_not_decode_when_invalid_channel() {
    let mut data = [0; 22];
    data[0] = 0x71;
    data[1] = 0x6f;
    data[2] = 0x69;
    data[3] = 0x66;
    data[6] = 0x7D;
    data[10] = 0x7D;
    data[21] = 0x01;

    let error = qoi::decode(&data, None).unwrap_err();
    println!("{:?}", error.kind);
    assert!(matches!(error.kind, DecodeErrorKind::InvalidCast(_)));
}

#[test]
fn should_not_decode_when_invalid_color_space() {
    let mut data = [0; 22];
    data[0] = 0x71;
    data[1] = 0x6f;
    data[2] = 0x69;
    data[3] = 0x66;
    data[6] = 0x7D;
    data[10] = 0x7D;
    data[11] = 0x03;
    data[12] = 0x02;
    data[21] = 0x01;

    let error = qoi::decode(&data, None).unwrap_err();
    println!("{:?}", error.kind);
    assert!(matches!(error.kind, DecodeErrorKind::InvalidCast(_)));
}
