use std::result;

use ris_input::gate::Gate;

#[test]
fn should_create_correct_state() {
    let mut gate = Gate::new();
    let result0 = gate.clone();

    gate.update(true);
    let result1 = gate.clone();
    gate.update(false);
    let result2 = gate.clone();
    gate.update(false);
    let result3 = gate.clone();
    gate.update(true);
    let result4 = gate.clone();
    gate.update(true);
    let result5 = gate.clone();

    assert_eq!(result0.up(), false);
    assert_eq!(result0.down(), false);
    assert_eq!(result0.hold(), false);

    assert_eq!(result1.up(), false);
    assert_eq!(result1.down(), true);
    assert_eq!(result1.hold(), true);

    assert_eq!(result2.up(), true);
    assert_eq!(result2.down(), false);
    assert_eq!(result2.hold(), false);

    assert_eq!(result3.up(), false);
    assert_eq!(result3.down(), false);
    assert_eq!(result3.hold(), false);

    assert_eq!(result4.up(), false);
    assert_eq!(result4.down(), true);
    assert_eq!(result4.hold(), true);

    assert_eq!(result5.up(), false);
    assert_eq!(result5.down(), false);
    assert_eq!(result5.hold(), true);
}
