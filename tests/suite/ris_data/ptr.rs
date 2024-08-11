use ris_data::cell::ArefCell;
use ris_data::ptr::StrongPtr;

#[test]
fn should_deref_strong_ptr() {
    let ptr = StrongPtr::new(42);
    assert_eq!(*ptr, 42);
}

#[test]
fn should_deref_weak_ptr() {
    let ptr = StrongPtr::new(42);
    let weak1 = ptr.to_weak();
    let weak2 = weak1.clone();
    let weak3 = weak1.clone();
    assert_eq!(*weak1, 42);
    assert_eq!(*weak2, 42);
    assert_eq!(*weak3, 42);
}

#[test]
fn should_deref_weak_ptr_after_strong_ptr() {
    let ptr = StrongPtr::new(ArefCell::new(42));
    let weak1 = ptr.to_weak();
    let weak2 = weak1.clone();
    let weak3 = weak2.clone();

    *ptr.borrow_mut() = 13;

    assert_eq!(*weak1.borrow(), 13);
    assert_eq!(*weak2.borrow(), 13);
    assert_eq!(*weak3.borrow(), 13);
}

#[test]
#[should_panic]
fn should_panic_on_deref_weak_ptr_when_strong_ptr_was_dropped() {
    unsafe {
        ris_error::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let ptr = StrongPtr::new(42);
    let weak = ptr.to_weak();
    drop(ptr);
    let _ = *weak;
}

#[test]
fn should_leak_when_dropping_while_a_reference_exists() {
    panic!()
}
