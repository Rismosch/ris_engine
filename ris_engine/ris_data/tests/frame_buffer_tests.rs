use std::time::Duration;

use ris_data::*;

use ris_test::util::*;

#[test]
fn should_initialize_n_frames() {
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

    assert_eq!(frame0.delta(), frame::IDEAL_DELTA);
    assert_eq!(frame1.delta(), frame::IDEAL_DELTA);
    assert_eq!(frame2.delta(), frame::IDEAL_DELTA);
    assert_eq!(frame3.delta(), frame::IDEAL_DELTA);
}

#[test]
fn should_add_frames() {
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
    assert_eq!(frame2.delta(), frame::IDEAL_DELTA);
    assert_eq!(frame3.delta(), frame::IDEAL_DELTA);
}

#[test]
fn should_wrap_around() {
    unsafe {
        frame_buffer::init(3);

        frame_buffer::add(Duration::from_millis(0));
        frame_buffer::add(Duration::from_millis(1));
        frame_buffer::add(Duration::from_millis(2));
        frame_buffer::add(Duration::from_millis(3));
        frame_buffer::add(Duration::from_millis(4));
        frame_buffer::add(Duration::from_millis(5));
        frame_buffer::add(Duration::from_millis(6));
    }

    let frame0 = frame_buffer::get(0);
    let frame1 = frame_buffer::get(1);
    let frame2 = frame_buffer::get(2);

    assert_eq!(frame0.number(), 6);
    assert_eq!(frame1.number(), 5);
    assert_eq!(frame2.number(), 4);

    assert_eq!(frame0.delta(), Duration::from_millis(6));
    assert_eq!(frame1.delta(), Duration::from_millis(5));
    assert_eq!(frame2.delta(), Duration::from_millis(4));
}

#[test]
#[should_panic]
fn should_panic_when_getting_frame_outside_of_range() {
    unsafe {
        frame_buffer::init(10);
    }

    let _ = frame_buffer::get(10);
}

#[test]
fn should_calculate_average_delta() {
    let expected =
        (2 * frame::IDEAL_DELTA + Duration::from_millis(123) + Duration::from_millis(456)) / 4;

    unsafe {
        frame_buffer::init(4);

        frame_buffer::add(Duration::from_millis(123));
        frame_buffer::add(Duration::from_millis(456));
    }

    assert_eq!(frame_buffer::delta(), expected);
}

#[test]
fn should_set_delta_to_ideal_when_duration_is_too_big() {
    unsafe {
        frame_buffer::init(4);

        frame_buffer::add(Duration::from_secs(10));
    }

    let frame = frame_buffer::get(0);

    assert_eq!(frame.delta(), frame::IDEAL_DELTA);
}
