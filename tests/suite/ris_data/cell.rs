use std::sync::Arc;

use ris_data::ptr::ArefCell;

//-----------------------------//
//                             //
//    single threaded tests    //
//                             //
//-----------------------------//

#[test]
fn should_deref() {
    let cell = ArefCell::new(42);
    let borrow = cell.borrow();
    assert_eq!(*borrow, 42);
}

#[test]
fn should_deref_mut() {
    let cell = ArefCell::new(42);
    let mut borrow_mut = cell.borrow_mut();
    assert_eq!(*borrow_mut, 42);
    *borrow_mut = 13;
    assert_eq!(*borrow_mut, 13);
}

#[test]
fn should_deref_mut_and_deref() {
    let cell = ArefCell::new(42);
    {
        let mut borrow_mut = cell.borrow_mut();
        *borrow_mut = 13;
    }
    let borrow = cell.borrow();
    assert_eq!(*borrow, 13);
}

#[test]
fn should_not_panic_when_borrowing_multiple_times() {
    let cell = ArefCell::new(42);

    let mut borrows = Vec::new();
    for _ in 0..100 {
        let borrow = cell.borrow();
        borrows.push(borrow);
    }

    for borrow in borrows {
        assert_eq!(*borrow, 42);
    }
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn should_panic_when_borrowing_while_ref_mut_exists() {
    unsafe {
        ris_error::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let cell = ArefCell::new(42);
    let _borrow_mut = cell.borrow_mut();
    let _borrow = cell.borrow();
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn should_panic_when_borrowing_mut_while_ref_exists() {
    unsafe {
        ris_error::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let cell = ArefCell::new(42);
    let _borrow = cell.borrow();
    let _borrow_mut = cell.borrow_mut();
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn should_panic_when_borrowing_mut_while_ref_mut_exists() {
    unsafe {
        ris_error::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let cell = ArefCell::new(42);
    let _borrow_mut1 = cell.borrow_mut();
    let _borrow_mut2 = cell.borrow_mut();
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn should_panic_when_deref_and_cell_was_dropped() {
    unsafe {
        ris_error::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let cell = ArefCell::new(42);
    let borrow = cell.borrow();
    drop(cell);
    let _ = *borrow;
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn should_panic_when_deref_mut_and_cell_was_dropped() {
    unsafe {
        ris_error::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let cell = ArefCell::new(42);
    let mut borrow_mut = cell.borrow_mut();
    drop(cell);
    *borrow_mut = 13;
}

//----------------------------//
//                            //
//    multi threaded tests    //
//                            //
//----------------------------//

#[test]
fn should_deref_from_different_thread() {
    let cell = ArefCell::new(42);

    let handle = std::thread::spawn(move || {
        let borrow = cell.borrow();
        *borrow
    });

    let value = handle.join().unwrap();

    assert_eq!(value, 42);
}

#[test]
fn should_deref_mut_and_deref_from_different_threads() {
    let cell = Arc::new(ArefCell::new(42));

    let cell_copy = cell.clone();
    let handle = std::thread::spawn(move || {
        let mut borrow_mut = cell_copy.borrow_mut();
        *borrow_mut = 13;
    });

    handle.join().unwrap();

    let borrow = cell.borrow();
    assert_eq!(*borrow, 13);
}

#[test]
fn should_not_panic_when_borrowing_multiple_times_from_different_threads() {
    let cell = Arc::new(ArefCell::new(42));

    let mut handles = Vec::new();
    for _ in 0..100 {
        let cell_copy = cell.clone();
        let handle = std::thread::spawn(move || {
            let borrow = cell_copy.borrow();
            *borrow
        });

        handles.push(handle);
    }

    for handle in handles {
        let value = handle.join().unwrap();
        assert_eq!(value, 42);
    }
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn should_panic_when_borrowing_while_ref_mut_exists_in_multiple_threads() {
    unsafe {
        ris_error::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let cell = Arc::new(ArefCell::new(42));
    let cell_copy = cell.clone();

    let handle1 = std::thread::spawn(move || {
        let _borrow_mut = cell.borrow_mut();
        std::thread::sleep(std::time::Duration::from_millis(100));
    });

    let handle2 = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(10));
        let _borrow = cell_copy.borrow();
        std::thread::sleep(std::time::Duration::from_millis(100));
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn should_panic_when_borrowing_mut_while_ref_exists_in_other_thread() {
    unsafe {
        ris_error::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let cell = Arc::new(ArefCell::new(42));
    let cell_copy = cell.clone();

    let handle1 = std::thread::spawn(move || {
        let _borrow = cell.borrow();
        std::thread::sleep(std::time::Duration::from_millis(100));
    });

    let handle2 = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(10));
        let _borrow_mut = cell_copy.borrow_mut();
        std::thread::sleep(std::time::Duration::from_millis(100));
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn should_panic_when_borrowing_mut_while_ref_mut_exists_in_other_thread() {
    unsafe {
        ris_error::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let cell = Arc::new(ArefCell::new(42));
    let cell_copy = cell.clone();

    let handle1 = std::thread::spawn(move || {
        let _borrow_mut = cell.borrow_mut();
        std::thread::sleep(std::time::Duration::from_millis(100));
    });

    let handle2 = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(10));
        let _borrow_mut = cell_copy.borrow_mut();
        std::thread::sleep(std::time::Duration::from_millis(100));
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn should_panic_when_deref_and_cell_was_dropped_in_different_thread() {
    unsafe {
        ris_error::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let cell = Arc::new(ArefCell::new(42));
    let cell_copy = cell.clone();

    let handle = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(10));
        let borrow = cell_copy.borrow();
        drop(cell_copy);
        let _ = *borrow;
    });

    drop(cell);

    handle.join().unwrap();
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn should_panic_when_deref_mut_and_cell_was_dropped_in_multiple_threads() {
    unsafe {
        ris_error::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let cell = Arc::new(ArefCell::new(42));
    let cell_copy = cell.clone();

    let handle = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(10));
        let mut borrow_mut = cell_copy.borrow_mut();
        drop(cell_copy);
        *borrow_mut = 13;
    });

    drop(cell);

    handle.join().unwrap();
}
