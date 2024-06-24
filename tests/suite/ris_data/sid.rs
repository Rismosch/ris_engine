#[test]
fn should_eq() {
    let sid1 = ris_data::sid!("hoi");
    let sid2 = ris_data::sid!("poi");
    let sid3 = ris_data::sid!("hoi");

    assert!(sid1 != sid2);
    assert!(sid1 == sid3);
    assert!(sid2 != sid3);
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn should_detect_collision() {
    unsafe {
        ris_error::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
        ris_error::error::GET_TIMESTAMP_ON_BACKTRACE = false;
    }

    let sid1 = ris_data::sid!("wCVg");
    let sid2 = ris_data::sid!("S0jh");

    let _ = sid1 == sid2;
}

#[test]
#[cfg(not(debug_assertions))]
fn should_detect_collision() {
    let sid1 = ris_data::sid!("wCVg");
    let sid2 = ris_data::sid!("S0jh");

    // this wont panic. in release original strings are not stored. this means collision detection
    // cannot work and thus is turned off.
    assert!(sid1 == sid2);
}
