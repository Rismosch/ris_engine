use std::sync::Arc;

use ris_data::ecs::registry::Registry;
use ris_data::ecs::scene::SceneCreateInfo;
use ris_data::god_state::GodState;
use ris_data::input::rebind_matrix::RebindMatrix;
use ris_data::settings::Settings;
use ris_input::general_logic::update_general;
use ris_rng::rng::Rng;
use ris_rng::rng::Seed;
use ris_util::testing::miri_choose;

struct TestContext {
    rng: Rng,
    state: GodState,
}

impl TestContext {
    fn new() -> Self {
        let rng = Rng::new(Seed::new().unwrap());

        let mut scene_create_info =
            miri_choose(SceneCreateInfo::default(), SceneCreateInfo::empty());
        scene_create_info.registry = Some(Arc::new(Registry::new(Vec::new()).unwrap()));

        let state = GodState::new(Settings::default(), scene_create_info).unwrap();

        Self { rng, state }
    }

    fn generate_rebindmatrix(&mut self) -> RebindMatrix {
        let mut rebind_matrix = RebindMatrix::new_empty();

        for entry in &mut rebind_matrix.data {
            *entry = self.rng.next_u32();
        }

        rebind_matrix
    }
}

#[test]
fn should_forward_buttons_by_default() {
    let mut context = TestContext::new();

    for i in 0..32 {
        let input = 1 << i;

        context.state.input.mouse.buttons.update(input);

        update_general(&mut context.state);

        let actual = context.state.input.general.buttons.hold();
        assert_eq!(input, actual, "{}", i);
    }
}

#[test]
fn can_block_buttons() {
    let mut context = TestContext::new();

    let empty_rebind_matrix = RebindMatrix::new_empty();

    RebindMatrix::copy(
        &empty_rebind_matrix,
        &mut context.state.input.keyboard.rebind_matrix,
    );

    for i in 0..32 {
        let input = i << 1;
        context.state.input.keyboard.buttons.update(input);

        update_general(&mut context.state);

        let actual = context.state.input.general.buttons.hold();
        assert_eq!(0, actual, "{}", i);
    }
}

#[test]
fn should_rebind_buttons() {
    let mut context = TestContext::new();

    for _ in 0..100 {
        let input_index = context.rng.next_i32_between(0, 31);
        let input = 1 << input_index as u32;

        context.state.input.gamepad.buttons.update(input);

        let rebind_matrix = context.generate_rebindmatrix();

        RebindMatrix::copy(
            &rebind_matrix,
            &mut context.state.input.gamepad.rebind_matrix,
        );

        update_general(&mut context.state);

        let expected = rebind_matrix.data[input_index as usize];
        let actual = context.state.input.general.buttons.hold();
        assert_eq!(expected, actual);
    }
}

#[test]
fn should_bitwise_or_all_inputs() {
    let mut context = TestContext::new();

    context
        .state
        .input
        .mouse
        .buttons
        .update(0b0000_0000_0000_1111);
    context
        .state
        .input
        .keyboard
        .buttons
        .update(0b0000_0000_1111_0000);
    context
        .state
        .input
        .gamepad
        .buttons
        .update(0b0000_1111_0000_0000);

    update_general(&mut context.state);

    let expected = 0b0000_1111_1111_1111;
    let actual = context.state.input.general.buttons.hold();
    assert_eq!(expected, actual);
}

#[test]
fn should_not_be_down_when_other_input_holds() {
    let mut context = TestContext::new();

    context.state.input.mouse.buttons.update(1);
    update_general(&mut context.state);
    assert_eq!(context.state.input.general.buttons.down(), 1);

    for _ in 0..100 {
        context.state.input.gamepad.buttons.update(1);
        update_general(&mut context.state);
        assert_eq!(context.state.input.general.buttons.down(), 0);

        context.state.input.gamepad.buttons.update(0);
        update_general(&mut context.state);
        assert_eq!(context.state.input.general.buttons.down(), 0);
    }

    context.state.input.mouse.buttons.update(0);
    update_general(&mut context.state);
    assert_eq!(context.state.input.general.buttons.down(), 0);
}

#[test]
fn should_not_be_up_when_other_input_holds() {
    let mut context = TestContext::new();

    context.state.input.mouse.buttons.update(1);
    update_general(&mut context.state);
    assert_eq!(context.state.input.general.buttons.up(), 0);

    for _ in 0..100 {
        context.state.input.gamepad.buttons.update(1);
        update_general(&mut context.state);
        assert_eq!(context.state.input.general.buttons.up(), 0);

        context.state.input.gamepad.buttons.update(0);
        update_general(&mut context.state);
        assert_eq!(context.state.input.general.buttons.up(), 0);
    }

    context.state.input.mouse.buttons.update(0);
    update_general(&mut context.state);
    assert_eq!(context.state.input.general.buttons.up(), 1);
}
