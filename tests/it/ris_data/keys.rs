use sdl2::keyboard::Scancode;

use ris_data::input::keys::Keys;

#[test]
fn should_calculate_up() {
    let mut keys = Keys::default();

    assert!(!keys.is_up(Scancode::A));
    assert!(!keys.is_up(Scancode::B));
    assert!(!keys.is_up(Scancode::C));
    assert!(!keys.is_up(Scancode::D));
    assert!(!keys.is_up(Scancode::E));

    keys.set_old_and_clear(keys.hold());
    keys.set(Scancode::A);
    keys.set(Scancode::B);
    keys.set(Scancode::C);
    assert!(!keys.is_up(Scancode::A));
    assert!(!keys.is_up(Scancode::B));
    assert!(!keys.is_up(Scancode::C));
    assert!(!keys.is_up(Scancode::D));
    assert!(!keys.is_up(Scancode::E));

    keys.set_old_and_clear(keys.hold());
    keys.set(Scancode::C);
    keys.set(Scancode::D);
    keys.set(Scancode::E);
    assert!(keys.is_up(Scancode::A));
    assert!(keys.is_up(Scancode::B));
    assert!(!keys.is_up(Scancode::C));
    assert!(!keys.is_up(Scancode::D));
    assert!(!keys.is_up(Scancode::E));
}

#[test]
fn should_calculate_down() {
    let mut keys = Keys::default();

    assert!(!keys.is_down(Scancode::A));
    assert!(!keys.is_down(Scancode::B));
    assert!(!keys.is_down(Scancode::C));
    assert!(!keys.is_down(Scancode::D));
    assert!(!keys.is_down(Scancode::E));

    keys.set_old_and_clear(keys.hold());
    keys.set(Scancode::A);
    keys.set(Scancode::B);
    keys.set(Scancode::C);
    assert!(keys.is_down(Scancode::A));
    assert!(keys.is_down(Scancode::B));
    assert!(keys.is_down(Scancode::C));
    assert!(!keys.is_down(Scancode::D));
    assert!(!keys.is_down(Scancode::E));

    keys.set_old_and_clear(keys.hold());
    keys.set(Scancode::C);
    keys.set(Scancode::D);
    keys.set(Scancode::E);
    assert!(!keys.is_down(Scancode::A));
    assert!(!keys.is_down(Scancode::B));
    assert!(!keys.is_down(Scancode::C));
    assert!(keys.is_down(Scancode::D));
    assert!(keys.is_down(Scancode::E));
}

#[test]
fn should_calculate_hold() {
    let mut keys = Keys::default();

    assert!(!keys.is_hold(Scancode::A));
    assert!(!keys.is_hold(Scancode::B));
    assert!(!keys.is_hold(Scancode::C));
    assert!(!keys.is_hold(Scancode::D));
    assert!(!keys.is_hold(Scancode::E));

    keys.set_old_and_clear(keys.hold());
    keys.set(Scancode::A);
    keys.set(Scancode::B);
    keys.set(Scancode::C);
    assert!(keys.is_hold(Scancode::A));
    assert!(keys.is_hold(Scancode::B));
    assert!(keys.is_hold(Scancode::C));
    assert!(!keys.is_hold(Scancode::D));
    assert!(!keys.is_hold(Scancode::E));

    keys.set_old_and_clear(keys.hold());
    keys.set(Scancode::C);
    keys.set(Scancode::D);
    keys.set(Scancode::E);
    assert!(!keys.is_hold(Scancode::A));
    assert!(!keys.is_hold(Scancode::B));
    assert!(keys.is_hold(Scancode::C));
    assert!(keys.is_hold(Scancode::D));
    assert!(keys.is_hold(Scancode::E));
}

#[test]
fn should_check_key_combination() {
    let mut keys = Keys::default();
    let key_combination = [Scancode::LCtrl, Scancode::LAlt, Scancode::Delete];

    assert!(!keys.check_combination(&key_combination));

    // all down
    keys.set(Scancode::LCtrl);
    keys.set(Scancode::LAlt);
    keys.set(Scancode::Delete);
    assert!(keys.check_combination(&key_combination));

    // all hold
    keys.set_old_and_clear(keys.hold());
    keys.set(Scancode::LCtrl);
    keys.set(Scancode::LAlt);
    keys.set(Scancode::Delete);
    assert!(!keys.check_combination(&key_combination));

    // last up
    keys.set_old_and_clear(keys.hold());
    keys.set(Scancode::LCtrl);
    keys.set(Scancode::LAlt);
    assert!(!keys.check_combination(&key_combination));

    // last down
    keys.set_old_and_clear(keys.hold());
    keys.set(Scancode::LCtrl);
    keys.set(Scancode::LAlt);
    keys.set(Scancode::Delete);
    assert!(keys.check_combination(&key_combination));

    // only last down
    keys.set_old_and_clear(keys.hold());
    keys.set(Scancode::LCtrl);
    keys.set(Scancode::LAlt);
    keys.set_old_and_clear(keys.hold());
    keys.set(Scancode::Delete);
    assert!(!keys.check_combination(&key_combination));
}
