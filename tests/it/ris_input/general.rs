use ris_input::{
    buttons::{Buttons, IButtons},
    general::{General, IGeneral, RebindMatrix},
};
use ris_rng::rng::Rng;
use ris_test::{
    harness::{test_harness, ITestContext},
    repeat::test_repeat,
};

struct GeneralTestContext {
    general: General,
    mouse: Buttons,
    keyboard: Buttons,
    gamepad: Buttons,

    rng: Rng,

    rebind_matrix_mouse: RebindMatrix,
    rebind_matrix_keyboard: RebindMatrix,
    rebind_matrix_gamepad: RebindMatrix,
}

impl GeneralTestContext {
    fn update(&mut self) {
        self.general
            .update_state(&self.mouse, &self.keyboard, &self.gamepad);
    }
}

impl ITestContext for GeneralTestContext {
    fn setup() -> Self {
        let general = General::default();
        let mouse = Buttons::default();
        let keyboard = Buttons::default();
        let gamepad = Buttons::default();

        let mut rng = Rng::new().unwrap();

        let mut rebind_matrix_mouse: RebindMatrix = [0; 32];
        let mut rebind_matrix_keyboard: RebindMatrix = [0; 32];
        let mut rebind_matrix_gamepad: RebindMatrix = [0; 32];

        for i in 0..32 {
            rebind_matrix_mouse[i] = rng.next_u();
            rebind_matrix_keyboard[i] = rng.next_u();
            rebind_matrix_gamepad[i] = rng.next_u();
        }

        GeneralTestContext {
            general,
            mouse,
            keyboard,
            gamepad,
            rng,
            rebind_matrix_mouse,
            rebind_matrix_keyboard,
            rebind_matrix_gamepad,
        }
    }

    fn teardown(&mut self) {}
}

#[test]
fn should_forward_buttons_by_default() {
    test_repeat(3, |index| {
        test_harness::<GeneralTestContext>(Box::new(move |context| {
            for i in 0..32 {
                let expected = i << 1;

                let buttons = match index {
                    0 => &mut context.mouse,
                    1 => &mut context.keyboard,
                    2 => &mut context.gamepad,
                    _ => panic!(),
                };
                buttons.update(&expected);

                context.update();

                let actual = context.general.buttons().hold();
                assert_eq!(expected, actual, "{}", i);
            }
        }));
    });
}

#[test]
fn can_block_buttons() {
    test_repeat(3, |index| {
        test_harness::<GeneralTestContext>(Box::new(move |context| {
            let empty_rebind_matrix: RebindMatrix = [0; 32];
            match index {
                0 => context.general.set_rebind_matrix(
                    ris_input::general::RebindMatrixKind::Mouse,
                    &empty_rebind_matrix,
                ),
                1 => context.general.set_rebind_matrix(
                    ris_input::general::RebindMatrixKind::Keyboard,
                    &empty_rebind_matrix,
                ),
                2 => context.general.set_rebind_matrix(
                    ris_input::general::RebindMatrixKind::Gamepad,
                    &empty_rebind_matrix,
                ),
                _ => panic!(),
            }

            for i in 0..32 {
                let buttons = match index {
                    0 => &mut context.mouse,
                    1 => &mut context.keyboard,
                    2 => &mut context.gamepad,
                    _ => panic!(),
                };
                buttons.update(&(i << 1));

                context.update();

                let actual = context.general.buttons().hold();
                assert_eq!(0, actual, "{}", i);
            }
        }));
    });
}

#[test]
fn should_rebind() {
    test_repeat(100, |_| {
        test_repeat(3, |index| {
            test_harness::<GeneralTestContext>(Box::new(move |context| {
                let input_index = context.rng.range_i(0, 32);
                let input = 1 << input_index as u32;

                let rebind_matrix = match index {
                    0 => {
                        context.mouse.update(&input);

                        context.general.set_rebind_matrix(
                            ris_input::general::RebindMatrixKind::Mouse,
                            &context.rebind_matrix_mouse,
                        );
                        context.rebind_matrix_mouse
                    }
                    1 => {
                        context.keyboard.update(&input);

                        context.general.set_rebind_matrix(
                            ris_input::general::RebindMatrixKind::Keyboard,
                            &context.rebind_matrix_keyboard,
                        );
                        context.rebind_matrix_keyboard
                    }
                    2 => {
                        context.gamepad.update(&input);

                        context.general.set_rebind_matrix(
                            ris_input::general::RebindMatrixKind::Gamepad,
                            &context.rebind_matrix_gamepad,
                        );
                        context.rebind_matrix_gamepad
                    }
                    _ => panic!(),
                };

                context.update();

                let expected = rebind_matrix[input_index as usize];
                let actual = context.general.buttons().hold();

                println!("5: {:#034b}", actual);

                assert_eq!(expected, actual);
            }));
        });
    });
}

#[test]
fn should_bitwise_or_all_inputs() {
    test_harness::<GeneralTestContext>(Box::new(|context| {
        context.mouse.update(&0b0000_0000_0000_1111);
        context.keyboard.update(&0b0000_0000_1111_0000);
        context.gamepad.update(&0b0000_1111_0000_0000);

        context.update();

        let expected = 0b0000_1111_1111_1111;
        let actual = context.general.buttons().hold();
        assert_eq!(expected, actual);
    }));
}

#[test]
fn should_not_be_down_when_other_input_holds() {
    test_harness::<GeneralTestContext>(Box::new(|context| {
        context.mouse.update(&1);
        context.update();
        assert_eq!(context.general.buttons().down(), 1);

        for _ in 0..100 {
            context.gamepad.update(&1);
            context.update();
            assert_eq!(context.general.buttons().down(), 0);

            context.gamepad.update(&0);
            context.update();
            assert_eq!(context.general.buttons().down(), 0);
        }

        context.mouse.update(&0);
        context.update();
        assert_eq!(context.general.buttons().down(), 0);
    }));
}

#[test]
fn should_not_be_up_when_other_input_holds() {
    test_harness::<GeneralTestContext>(Box::new(|context| {
        context.mouse.update(&1);
        context.update();
        assert_eq!(context.general.buttons().up(), 0);

        for _ in 0..100 {
            context.gamepad.update(&1);
            context.update();
            assert_eq!(context.general.buttons().up(), 0);

            context.gamepad.update(&0);
            context.update();
            assert_eq!(context.general.buttons().up(), 0);
        }

        context.mouse.update(&0);
        context.update();
        assert_eq!(context.general.buttons().up(), 1);
    }));
}
