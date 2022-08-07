use std::cell::{Cell, UnsafeCell};

use ris_data::concurrent_queue::ConcurrentQueue;

#[test]
fn should_push_and_pop() {
    let mut queue = ConcurrentQueue::new();

    queue.push(1);
    queue.push(2);
    queue.push(3);

    let one = queue.pop();
    let two = queue.pop();

    queue.push(4);
    queue.push(5);

    let three = queue.pop();
    let four = queue.pop();
    let five = queue.pop();
    let six = queue.pop();

    assert_eq!(one, Some(1));
    assert_eq!(two, Some(2));
    assert_eq!(three, Some(3));
    assert_eq!(four, Some(4));
    assert_eq!(five, Some(5));
    assert_eq!(six, None);
}

// fn opaque_read(value: &i32){
//     println!("{}", value);
// }

// #[test]
// fn example(){
//     unsafe{
//         let mut data = Box::new(10);
//         let ptr = (&mut *data) as *mut i32;

//         *ptr += 1;
//         *data += 10;

//         println!("{}", data);
//     }
    
//     panic!()
// }