use ris_data::ris_map::RisMap;

#[test]
fn should_insert_and_get_items() {
    let mut map = RisMap::default();
    let _ = map.assign("the answer", 42);
    let _ = map.assign("my favorite number", 13);
    let _ = map.assign("hoi", 7);
    let _ = map.assign("poi", 1111111111);
    let _ = map.assign("im not negative", -2);
    let _ = map.assign("min", i32::MIN);
    let _ = map.assign("max", i32::MAX);

    let the_answer = *map.find("the answer").unwrap().unwrap();
    let my_favorite_number = *map.find("my favorite number").unwrap().unwrap();
    let hoi = *map.find("hoi").unwrap().unwrap();
    let poi = *map.find("poi").unwrap().unwrap();
    let im_not_negative = *map.find("im not negative").unwrap().unwrap();
    let min = *map.find("min").unwrap().unwrap();
    let max = *map.find("max").unwrap().unwrap();

    assert_eq!(the_answer, 42);
    assert_eq!(my_favorite_number, 13);
    assert_eq!(hoi, 7);
    assert_eq!(poi, 1111111111);
    assert_eq!(im_not_negative, -2);
    assert_eq!(min, i32::MIN);
    assert_eq!(max, i32::MAX);
}

#[test]
fn should_overwrite_existing_item() {
    let mut map = RisMap::default();
    let _ = map.assign("my key", -14);
    let _ = map.assign("my key", 42);

    let item = *map.find("my key").unwrap().unwrap();
    assert_eq!(item, 42);
}

#[test]
fn should_return_error_on_assign_when_out_of_memory() {
    let mut map = RisMap::default();
    for i in 0..(1 << ris_data::ris_map::EXP) - 1 {
        let result = map.assign(&format!("key {}", i), i);
        assert!(result.is_ok());
    }

    let result = map.assign("i don't fit :(", 42);
    assert!(result.is_err());
}

#[test]
fn should_remove_items() {
    let mut map = RisMap::default();
    let _ = map.assign("the answer", 42);
    let _ = map.assign("my favorite number", 13);
    let _ = map.assign("hoi", 7);
    let _ = map.assign("poi", 1111111111);
    let _ = map.assign("im not negative", -2);
    let _ = map.assign("min", i32::MIN);
    let _ = map.assign("max", i32::MAX);

    let _ = map.remove("hoi");
    let _ = map.remove("poi");
    let _ = map.remove("my favorite number");

    let the_answer = map.find("the answer").unwrap();
    assert!(the_answer.is_some());

    let my_favorite_number = map.find("my favorite number").unwrap();
    assert!(my_favorite_number.is_none());

    let hoi = map.find("hoi").unwrap();
    assert!(hoi.is_none());

    let poi = map.find("poi").unwrap();
    assert!(poi.is_none());

    let im_not_negative = map.find("im not negative").unwrap();
    assert!(im_not_negative.is_some());

    let min = map.find("min").unwrap();
    assert!(min.is_some());

    let max = map.find("max").unwrap();
    assert!(max.is_some());
}

#[test]
fn should_return_error_on_remove_when_key_does_not_exist() {
    let mut map = RisMap::default();
    let _ = map.assign("my key", 42);
    let result1 = map.remove("my key");
    let result2 = map.remove("this key does not exist");

    assert!(result1.is_ok());
    assert!(result2.is_err());
}

#[test]
fn should_get_none_when_item_does_not_exist() {
    let mut map = RisMap::<i32>::default();
    let result = map.find("this key does not exist").unwrap();
    assert!(result.is_none());
}
