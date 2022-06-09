use ris_input::buttons::{Buttons, IButtons};

#[test]
fn should_calculate_up() {
    let mut buttons = Buttons::default();

    assert_eq!(
        buttons.up(),
        0b0000_0000___0000_0000___0000_0000___0000_0000
    );

    buttons.update(&0b0110_1111___1100_0101___1011_0011___0011_0111);
    assert_eq!(
        buttons.up(),
        0b0000_0000___0000_0000___0000_0000___0000_0000
    );

    buttons.update(&0b1011_1000___0010_1110___0110_0111___0010_0111);
    assert_eq!(
        buttons.up(),
        0b0100_0111___1100_0001___1001_0000___0001_0000
    );
}

#[test]
fn should_calculate_down() {
    let mut buttons = Buttons::default();

    assert_eq!(
        buttons.down(),
        0b0000_0000___0000_0000___0000_0000___0000_0000
    );

    buttons.update(&0b0110_1111___1100_0101___1011_0011___0011_0111);
    assert_eq!(
        buttons.down(),
        0b0110_1111___1100_0101___1011_0011___0011_0111
    );

    buttons.update(&0b1011_1000___0010_1110___0110_0111___0010_0111);
    assert_eq!(
        buttons.down(),
        0b1001_0000___0010_1010___0100_0100___0000_0000
    );
}

#[test]
fn should_calculate_hold() {
    let mut buttons = Buttons::default();

    assert_eq!(
        buttons.hold(),
        0b0000_0000___0000_0000___0000_0000___0000_0000
    );

    buttons.update(&0b0110_1111___1100_0101___1011_0011___0011_0111);
    assert_eq!(
        buttons.hold(),
        0b0110_1111___1100_0101___1011_0011___0011_0111
    );

    buttons.update(&0b1011_1000___0010_1110___0110_0111___0010_0111);
    assert_eq!(
        buttons.hold(),
        0b1011_1000___0010_1110___0110_0111___0010_0111
    );
}
