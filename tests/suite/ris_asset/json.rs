use ris_asset::codecs::json::JsonError;
use ris_asset::codecs::json::JsonObject;
use ris_asset::codecs::json::JsonMember;
use ris_asset::codecs::json::JsonValue;
use ris_asset::codecs::json::JsonNumber;

#[test]
fn should_serialize_example_1() {
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

    let json = object.serialize();
    let expected = "{\"Image\":{\"Width\":800,\"Height\":600,\"Title\":\"View from 15th Floor\",\"Thumbnail\":{\"Url\":\"http://www.example.com/image/481989943\",\"Height\":125,\"Width\":100},\"Animated\":false,\"IDs\":[116,943,234,38793]}}";
    assert_eq!(json, expected);
}

#[test]
fn should_serialize_example_2() {
    panic!();
}

#[test]
fn should_serialize_example_3() {
    let value = JsonValue::try_from("Hello world!").unwrap();
    let json = value.serialize();
    assert_eq!(json, "\"Hello world!\"");
}

#[test]
fn should_serialize_example_4() {
    let value = JsonValue::from(42);
    let json = value.serialize();
    assert_eq!(json, "42");
}

#[test]
fn should_serialize_example_5() {
    let value = JsonValue::from(true);
    let json = value.serialize();
    assert_eq!(json, "true");
}

#[test]
fn should_deserialize_example_1() {
    let example = "
        {
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
      }";
    panic!();
}

#[test]
fn should_deserialize_example_2() {
    let example = "
    [
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
           \"Latitude\":  37.371991,
           \"Longitude\": -122.026020,
           \"Address\":   \"\",
           \"City\":      \"SUNNYVALE\",
           \"State\":     \"CA\",
           \"Zip\":       \"94085\",
           \"Country\":   \"US\"
        }
      ]";
    panic!();
}

#[test]
fn should_deserialize_example_3() {
    let example = "Hello world!";
    panic!();
}

#[test]
fn should_deserialize_example_4() {
    let example = "42";
    panic!();
}

#[test]
fn should_deserialize_example_5() {
    let example = "true";
    panic!();
}
