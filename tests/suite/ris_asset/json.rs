use ris_asset::codecs::json::JsonObject;
use ris_asset::codecs::json::JsonValue;

// examples directly taken from the rfc
fn example_1_json() -> &'static str {
    "{
        \"Image\": {
            \"Width\":  800,
            \"Height\": 600,
            \"Title\":  \"View from 15th Floor\",
            \"Thumbnail\": {
                \"Url\":    \"http://www.example.com/image/481989943\",
                \"Height\": 125,
                \"Width\":  100
            },
            \"Animated\" : false,
            \"IDs\": [116, 943, 234, 38793]
        }
    }"
}


fn example_2_json() -> &'static str {

    // because of floating point imprecision and how numbers are stored and serilialized in rust, the two lines
    //
    //     \"Latitude\":  37.371991,
    //     \"Longitude\": -122.026020,
    //
    // have been changed to
    //
    //     \"Latitude\":  37.37199,
    //     \"Longitude\": -122.02602,
    "[
        {
           \"precision\": \"zip\",
           \"Latitude\":  37.7668,
           \"Longitude\": -122.3959,
           \"Address\":   \"\",
           \"City\":      \"SAN FRANCISCO\",
           \"State\":     \"CA\",
           \"Zip\":       \"94107\",
           \"Country\":   \"US\"
        },
        {
           \"precision\": \"zip\",
           \"Latitude\":  37.37199,
           \"Longitude\": -122.02602,
           \"Address\":   \"\",
           \"City\":      \"SUNNYVALE\",
           \"State\":     \"CA\",
           \"Zip\":       \"94085\",
           \"Country\":   \"US\"
        }
    ]"
}

fn example_3_json() -> &'static str {
    "\"Hello world!\""
}

fn example_4_json() -> &'static str {
    "42"
}

fn example_5_json() -> &'static str {
    "true"
}

fn example_1_value() -> JsonValue {
    let mut object = JsonObject::default();
    let mut image = JsonObject::default();
    image.push("Width", 800);
    image.push("Height", 600);
    image.push("Title", "View from 15th Floor");
    let mut thumbnail = JsonObject::default();
    thumbnail.push("Url", "http://www.example.com/image/481989943");
    thumbnail.push("Height", 125);
    thumbnail.push("Width", 100);
    image.push("Thumbnail", thumbnail);
    image.push("Animated", false);
    image.push("IDs", &[116, 943, 234, 38793]);
    object.push("Image", image);

    JsonValue::from(object)
}

fn example_2_value() -> JsonValue {
    let mut object1 = JsonObject::default();
    object1.push("precision", "zip");
    object1.push("Latitude", 37.7668);
    object1.push("Longitude", -122.3959);
    object1.push("Address", "");
    object1.push("City", "SAN FRANCISCO");
    object1.push("State", "CA");
    object1.push("Zip", "94107");
    object1.push("Country", "US");
    let mut object2 = JsonObject::default();
    object2.push("precision", "zip");
    object2.push("Latitude", 37.37199);
    object2.push("Longitude", -122.02602);
    object2.push("Address", "");
    object2.push("City", "SUNNYVALE");
    object2.push("State", "CA");
    object2.push("Zip", "94085");
    object2.push("Country", "US");
    JsonValue::from(&[object1, object2])
}

fn example_3_value() -> JsonValue {
    JsonValue::from("Hello world!")
}

fn example_4_value() -> JsonValue {
    JsonValue::from(42)
}

fn example_5_value() -> JsonValue {
    JsonValue::from(true)
}

#[test]
fn should_serialize_example_1() {
    let example = example_1_value();
    let json = example.serialize();
    let expected = "{\"Image\":{\"Width\":800,\"Height\":600,\"Title\":\"View from 15th Floor\",\"Thumbnail\":{\"Url\":\"http://www.example.com/image/481989943\",\"Height\":125,\"Width\":100},\"Animated\":false,\"IDs\":[116,943,234,38793]}}";
    assert_eq!(json, expected);
}

#[test]
fn should_serialize_example_2() {
    let example = example_2_value();
    let json = example.serialize();
    let expected = "[{\"precision\":\"zip\",\"Latitude\":37.7668,\"Longitude\":-122.3959,\"Address\":\"\",\"City\":\"SAN FRANCISCO\",\"State\":\"CA\",\"Zip\":\"94107\",\"Country\":\"US\"},{\"precision\":\"zip\",\"Latitude\":37.37199,\"Longitude\":-122.02602,\"Address\":\"\",\"City\":\"SUNNYVALE\",\"State\":\"CA\",\"Zip\":\"94085\",\"Country\":\"US\"}]";
    assert_eq!(json, expected);
}

#[test]
fn should_serialize_example_3() {
    let example = example_3_value();
    let json = example.serialize();
    assert_eq!(json, "\"Hello world!\"");
}

#[test]
fn should_serialize_example_4() {
    let example = example_4_value();
    let json = example.serialize();
    assert_eq!(json, "42");
}

#[test]
fn should_serialize_example_5() {
    let example = example_5_value();
    let json = example.serialize();
    assert_eq!(json, "true");
}

#[test]
fn should_deserialize_example_1() {
    let example = example_1_json();
    let value = JsonValue::deserialize(example).unwrap();
    let expected = example_1_value();
    assert_eq!(value, expected);
}

#[test]
fn should_deserialize_example_2() {
    let example = example_2_json();
    let value = JsonValue::deserialize(example).unwrap();
    let expected = example_2_value();
    assert_eq!(value, expected);
}

#[test]
fn should_deserialize_example_3() {
    let example = example_3_json();
    let value = JsonValue::deserialize(example).unwrap();
    let expected = example_3_value();
    assert_eq!(value, expected);
}

#[test]
fn should_deserialize_example_4() {
    let example = example_4_json();
    let value = JsonValue::deserialize(example).unwrap();
    let expected = example_4_value();
    assert_eq!(value, expected);
}

#[test]
fn should_deserialize_example_5() {
    let example = example_5_json();
    let value = JsonValue::deserialize(example).unwrap();
    let expected = example_5_value();
    assert_eq!(value, expected);
}

#[test]
#[should_panic]
fn number_should_not_be_infinity() {
    let _ = JsonValue::from(f32::INFINITY);
}

#[test]
#[should_panic]
fn number_should_not_be_neg_infinity() {
    let _ = JsonValue::from(f32::NEG_INFINITY);
}

#[test]
#[should_panic]
fn number_should_not_be_nan() {
    let _ = JsonValue::from(f32::NAN);
}

#[test]
fn should_deserialize_edge_cases() {
    panic!("make sure to hit every branch, place a panic in each and generate edgecases until all panics have been removed");
}
