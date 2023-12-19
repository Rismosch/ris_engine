use ris_data::input::buttons::Buttons;
use ris_data::input::general_data::GeneralData;
use ris_data::input::rebind_matrix::set_rebind_matrix;
use ris_data::input::rebind_matrix::RebindMatrix;
use ris_input::general_logic::update_general;
use ris_input::general_logic::GeneralLogicArgs;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;

struct TestContext {
    new_general: GeneralData,
    old_general: GeneralData,
    mouse: Buttons,
    keyboard: Buttons,
    gamepad: Buttons,

    rng: Rng,

    rebind_matrix_mouse: RebindMatrix,
    rebind_matrix_keyboard: RebindMatrix,
    rebind_matrix_gamepad: RebindMatrix,
}

impl TestContext {
    fn new() -> Self {
        let new_general = GeneralData::default();
        let old_general = GeneralData::default();
        let mouse = Buttons::default();
        let keyboard = Buttons::default();
        let gamepad = Buttons::default();

        let rng = Rng::new(Seed::new().unwrap());

        let mut rebind_matrix: RebindMatrix = [0; 32];

        for (i, entry) in rebind_matrix.iter_mut().enumerate() {
            *entry = 1 << i;
        }

        Self {
            new_general,
            old_general,
            mouse,
            keyboard,
            gamepad,
            rng,
            rebind_matrix_mouse: rebind_matrix,
            rebind_matrix_keyboard: rebind_matrix,
            rebind_matrix_gamepad: rebind_matrix,
        }
    }

    fn update(&mut self) {
        let args = GeneralLogicArgs {
            new_general_data: &mut self.new_general,
            old_general_data: &self.old_general,
            mouse: &self.mouse,
            keyboard: &self.keyboard,
            gamepad: &self.gamepad,
            rebind_matrix_mouse: &self.rebind_matrix_mouse,
            rebind_matrix_keyboard: &self.rebind_matrix_keyboard,
            rebind_matrix_gamepad: &self.rebind_matrix_gamepad,
        };

        update_general(args);

        std::mem::swap(&mut self.new_general, &mut self.old_general);
    }

    fn generate_rebindmatrix(&mut self) -> RebindMatrix {
        let mut rebind_matrix: RebindMatrix = [0; 32];

        for entry in &mut rebind_matrix {
            *entry = self.rng.next_u();
        }

        rebind_matrix
    }
}

#[test]
fn should_forward_buttons_by_default() {
    let mut context = TestContext::new();

    for i in 0..32 {
        let input = i << 1;

        context.mouse.update(input);

        context.update();

        let actual = context.old_general.buttons.hold();
        assert_eq!(input, actual, "{}", i);
    }
}

#[test]
fn can_block_buttons() {
    let mut context = TestContext::new();

    let empty_rebind_matrix: RebindMatrix = [0; 32];

    set_rebind_matrix(&empty_rebind_matrix, &mut context.rebind_matrix_keyboard);

    for i in 0..32 {
        let input = i << 1;
        context.keyboard.update(input);

        context.update();

        let actual = context.old_general.buttons.hold();
        assert_eq!(0, actual, "{}", i);
    }
}

#[test]
fn should_rebind_buttons() {
    let mut context = TestContext::new();

    for _ in 0..100 {
        let input_index = context.rng.range_i(0, 31);
        let input = 1 << input_index as u32;

        context.gamepad.update(input);

        let rebind_matrix = context.generate_rebindmatrix();

        set_rebind_matrix(&rebind_matrix, &mut context.rebind_matrix_gamepad);

        context.update();

        let expected = rebind_matrix[input_index as usize];
        let actual = context.old_general.buttons.hold();

        assert_eq!(expected, actual);
    }
}

#[test]
fn should_bitwise_or_all_inputs() {
    let mut context = TestContext::new();

    context.mouse.update(0b0000_0000_0000_1111);
    context.keyboard.update(0b0000_0000_1111_0000);
    context.gamepad.update(0b0000_1111_0000_0000);

    context.update();

    let expected = 0b0000_1111_1111_1111;
    let actual = context.old_general.buttons.hold();
    assert_eq!(expected, actual);
}

#[test]
fn should_not_be_down_when_other_input_holds() {
    let mut context = TestContext::new();

    context.mouse.update(1);
    context.update();
    assert_eq!(context.old_general.buttons.down(), 1);

    for _ in 0..100 {
        context.gamepad.update(1);
        context.update();
        assert_eq!(context.old_general.buttons.down(), 0);

        context.gamepad.update(0);
        context.update();
        assert_eq!(context.old_general.buttons.down(), 0);
    }

    context.mouse.update(0);
    context.update();
    assert_eq!(context.old_general.buttons.down(), 0);
}

#[test]
fn should_not_be_up_when_other_input_holds() {
    let mut context = TestContext::new();

    context.mouse.update(1);
    context.update();
    assert_eq!(context.old_general.buttons.up(), 0);

    for _ in 0..100 {
        context.gamepad.update(1);
        context.update();
        assert_eq!(context.old_general.buttons.up(), 0);

        context.gamepad.update(0);
        context.update();
        assert_eq!(context.old_general.buttons.up(), 0);
    }

    context.mouse.update(0);
    context.update();
    assert_eq!(context.old_general.buttons.up(), 1);
}
