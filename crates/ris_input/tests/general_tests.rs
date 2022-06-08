use ris_input::{general::{General, IGeneral, RebindMatrix}, buttons::{Buttons, IButtons}};
use ris_test::{harness::{ITestContext, test_harness}, repeat::test_repeat};

struct GeneralTestContext{
    general: General,
    mouse: Buttons,
    keyboard: Buttons,
    gamepad: Buttons,
}

impl GeneralTestContext {
    fn update(&mut self) {
        self.general.update_state(&self.mouse, &self.keyboard, &self.gamepad);
    }
}

impl ITestContext for GeneralTestContext {
    fn setup() -> Self {
        let general = General::default();
        let mouse = Buttons::default();
        let keyboard = Buttons::default();
        let gamepad = Buttons::default();

        GeneralTestContext{general, mouse, keyboard, gamepad}
    }

    fn teardown(&mut self) {}
}

#[test]
fn should_forward_buttons_by_default() {
    test_repeat(3, |index|{
        test_harness::<GeneralTestContext>(Box::new(move |context|{
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
    test_repeat(3, |index|{
        test_harness::<GeneralTestContext>(Box::new(move |context|{

            let empty_rebind_matrix: RebindMatrix = [0;32];
            match index {
                0 => context.general.set_rebind_matrix(ris_input::general::RebindMatrixKind::Mouse, &empty_rebind_matrix),
                1 => context.general.set_rebind_matrix(ris_input::general::RebindMatrixKind::Keyboard, &empty_rebind_matrix),
                2 => context.general.set_rebind_matrix(ris_input::general::RebindMatrixKind::Gamepad, &empty_rebind_matrix),
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
fn should_rebind_mouse() {
    panic!();
}

#[test]
fn should_rebind_keyboard() {
    panic!();
}

#[test]
fn should_rebind_gamepad() {
    panic!();
}

#[test]
fn should_bitwise_or_all_inputs() {
    panic!();
}

#[test]
fn can_rebind_to_multiple_buttons() {
    panic!();
}

#[test]
fn should_not_down_when_other_input_holds() {
    panic!();
}

#[test]
fn should_not_up_when_other_input_holds() {
    panic!();
}