use ris_input::buttons::{Buttons, IButtons};

#[test]
fn should_calculate_up() {
    let mut buttons = Buttons::default();

    assert_eq!(buttons.up(), 0b0000_0000_0000_0000_0000_0000_0000_0000);

    buttons.update(&0b0110_1111_1100_0101_1011_0011_0011_0111);
    assert_eq!(buttons.up(), 0b0000_0000_0000_0000_0000_0000_0000_0000);

    buttons.update(&0b1011_1000_0010_1110_0110_0111_0010_0111);
    assert_eq!(buttons.up(), 0b0100_0111_1100_0001_1001_0000_0001_0000);
}

#[test]
fn should_calculate_down() {
    let mut buttons = Buttons::default();

    assert_eq!(buttons.down(), 0b0000_0000_0000_0000_0000_0000_0000_0000);

    buttons.update(&0b0110_1111_1100_0101_1011_0011_0011_0111);
    assert_eq!(buttons.down(), 0b0110_1111_1100_0101_1011_0011_0011_0111);

    buttons.update(&0b1011_1000_0010_1110_0110_0111_0010_0111);
    assert_eq!(buttons.down(), 0b1001_0000_0010_1010_0100_0100_0000_0000);
}

#[test]
fn should_calculate_hold() {
    let mut buttons = Buttons::default();

    assert_eq!(buttons.hold(), 0b0000_0000_0000_0000_0000_0000_0000_0000);

    buttons.update(&0b0110_1111_1100_0101_1011_0011_0011_0111);
    assert_eq!(buttons.hold(), 0b0110_1111_1100_0101_1011_0011_0011_0111);

    buttons.update(&0b1011_1000_0010_1110_0110_0111_0010_0111);
    assert_eq!(buttons.hold(), 0b1011_1000_0010_1110_0110_0111_0010_0111);
}
