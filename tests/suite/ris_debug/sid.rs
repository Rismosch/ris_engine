#[test]
fn should_eq() {
    let sid1 = ris_debug::sid!("hoi");
    let sid2 = ris_debug::sid!("poi");
    let sid3 = ris_debug::sid!("hoi");

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
    }

    let sid1 = ris_debug::sid!("wCVg");
    let sid2 = ris_debug::sid!("S0jh");

    let _ = sid1 == sid2;
}

#[test]
#[cfg(not(debug_assertions))]
fn should_detect_collision() {
    let sid1 = ris_debug::sid!("wCVg");
    let sid2 = ris_debug::sid!("S0jh");

    // this wont panic. in release original strings are not stored. this means collision detection
    // cannot work and thus is turned off.
    assert!(sid1 == sid2);
}

#[test]
fn should_create_file_sids() {
    let sid1 = ris_debug::fsid!();
    let sid2 = ris_debug::fsid!();
    let sid3 = ris_debug::fsid!();

    assert!(sid1 != sid2);
    assert!(sid1 != sid3);
    assert!(sid2 != sid3);
}
