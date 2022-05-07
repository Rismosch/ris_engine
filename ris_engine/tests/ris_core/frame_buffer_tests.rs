use std::time::Duration;

use ris_core::frame_buffer;

use ris_testing_utility::*;

#[test]
fn should_initialize_n_frames() {
    retry(10, || {
        unsafe {
            frame_buffer::init(4);
        }

        let frame0 = frame_buffer::get(0);
        let frame1 = frame_buffer::get(1);
        let frame2 = frame_buffer::get(2);
        let frame3 = frame_buffer::get(3);

        println!("test1 {}", frame0.number());
        println!("test1 {}", frame1.number());
        println!("test1 {}", frame2.number());
        println!("test1 {}", frame3.number());

        let frame0_number: usize = !0;
        let frame1_number: usize = !0 - 1;
        let frame2_number: usize = !0 - 2;
        let frame3_number: usize = !0 - 3;

        assert_eq!(frame0.number(), frame0_number);
        assert_eq!(frame1.number(), frame1_number);
        assert_eq!(frame2.number(), frame2_number);
        assert_eq!(frame3.number(), frame3_number);

        assert_eq!(frame0.delta(), frame_buffer::IDEAL_DELTA);
        assert_eq!(frame1.delta(), frame_buffer::IDEAL_DELTA);
        assert_eq!(frame2.delta(), frame_buffer::IDEAL_DELTA);
        assert_eq!(frame3.delta(), frame_buffer::IDEAL_DELTA);
    });
}

#[test]
fn should_add_frames() {
    retry(10, || {
        unsafe {
            frame_buffer::init(4);

            frame_buffer::add(Duration::from_millis(123));
            frame_buffer::add(Duration::from_millis(456));
        }

        let frame0 = frame_buffer::get(0);
        let frame1 = frame_buffer::get(1);
        let frame2 = frame_buffer::get(2);
        let frame3 = frame_buffer::get(3);

        println!("test2 {}", frame0.number());
        println!("test2 {}", frame0.number());
        println!("test2 {}", frame1.number());
        println!("test2 {}", frame1.number());
        println!("test2 {}", frame2.number());
        println!("test2 {}", frame2.number());
        println!("test2 {}", frame3.number());
        println!("test2 {}", frame3.number());

        let frame0_number: usize = 1;
        let frame1_number: usize = 0;
        let frame2_number: usize = !0;
        let frame3_number: usize = !0 - 1;

        assert_eq!(frame0.number(), frame0_number);
        assert_eq!(frame1.number(), frame1_number);
        assert_eq!(frame2.number(), frame2_number);
        assert_eq!(frame3.number(), frame3_number);

        assert_eq!(frame0.delta(), Duration::from_millis(456));
        assert_eq!(frame1.delta(), Duration::from_millis(123));
        assert_eq!(frame2.delta(), frame_buffer::IDEAL_DELTA);
        assert_eq!(frame3.delta(), frame_buffer::IDEAL_DELTA);
    });
}
