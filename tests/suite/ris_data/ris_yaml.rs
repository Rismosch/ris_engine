use ris_data::ris_yaml::RisYaml;

// serialize
#[test]
fn should_serialize() {
    let mut yaml = RisYaml::default();

    yaml.add_entry(Some(("my first key", "my first value")), Some("my first comment"));
    yaml.add_entry(Some(("my second key", "my second value")), Some("my second comment"));
    yaml.add_entry(None, None);
    yaml.add_entry(None, Some("this line has no key/value"));
    yaml.add_entry(Some(("this line", "has no comment")), None);

    let result = yaml.serialize().unwrap();

    assert_eq!(
        result,
        "my first key: my first value # my first comment
my second key: my second value # my second comment

# this line has no key/value
this line: has no comment
"
    );
}

#[test]
fn should_not_serialize_when_key_is_invalid() {
    // no colon
    let mut yaml = RisYaml::default();
    yaml.add_entry(Some((":", "")), None);
    assert!(yaml.serialize().is_err());

    // no newline
    let mut yaml = RisYaml::default();
    yaml.add_entry(Some(("\n", "")), None);
    assert!(yaml.serialize().is_err());

    // no empty key
    let mut yaml = RisYaml::default();
    yaml.add_entry(Some(("", "")), None);
    assert!(yaml.serialize().is_err());
}

#[test]
fn should_not_serialize_when_value_is_invalid() {
    // no colon
    let mut yaml = RisYaml::default();
    yaml.add_entry(Some(("key", ":")), None);
    assert!(yaml.serialize().is_err());

    // no newline
    let mut yaml = RisYaml::default();
    yaml.add_entry(Some(("key", "\n")), None);
    assert!(yaml.serialize().is_err());

    // empty value is fine
    let mut yaml = RisYaml::default();
    yaml.add_entry(Some(("key", "")), None);
    assert!(yaml.serialize().is_ok());
}

#[test]
fn should_not_serialize_when_comment_is_invalid() {
    let mut yaml = RisYaml::default();
    yaml.add_entry(None, Some("\n"));
    assert!(yaml.serialize().is_err());
}

// deserialize
#[test]
fn should_parse_empty() {
    let yaml = RisYaml::deserialize("").unwrap();
    assert!(yaml.entries.is_empty());
}

#[test]
fn should_parse_key_value() {
    let yaml = RisYaml::deserialize("my key: my value").unwrap();
    assert_eq!(yaml.entries.len(), 1);

    let first = &yaml.entries[0];
    assert!(first.key_value.is_some());
    assert!(first.comment.is_none());
    assert_eq!(first.line, 1);

    let (key, value) = first.key_value.as_ref().unwrap();
    assert_eq!(key, "my key");
    assert_eq!(value, "my value");
}

#[test]
fn should_parse_comment() {
    let yaml = RisYaml::deserialize("# my comment").unwrap();
    assert_eq!(yaml.entries.len(), 1);

    let first = &yaml.entries[0];
    assert!(first.key_value.is_none());
    assert!(first.comment.is_some());
    assert_eq!(first.line, 1);

    let comment = first.comment.as_ref().unwrap();
    assert_eq!(comment, "my comment");
}

#[test]
fn should_parse_mutliple_comments() {
    let yaml = RisYaml::deserialize(" # 1 # 2 # 3 # 4 ").unwrap();
    assert_eq!(yaml.entries.len(), 1);

    let first = &yaml.entries[0];
    assert!(first.key_value.is_none());
    assert!(first.comment.is_some());
    assert_eq!(first.line, 1);

    let comment = first.comment.as_ref().unwrap();
    assert_eq!(comment, "1 # 2 # 3 # 4");
}

#[test]
fn should_parse_key_value_and_comment() {
    let yaml = RisYaml::deserialize("my key: my value # my comment").unwrap();
    assert_eq!(yaml.entries.len(), 1);

    let first = &yaml.entries[0];
    assert!(first.key_value.is_some());
    assert!(first.comment.is_some());
    assert_eq!(first.line, 1);

    let (key, value) = first.key_value.as_ref().unwrap();
    let comment = first.comment.as_ref().unwrap();
    assert_eq!(key, "my key");
    assert_eq!(value, "my value");
    assert_eq!(comment, "my comment");
}

#[test]
fn should_parse_everything() {
    let yaml = RisYaml::deserialize("
my first key: my first value # my first comment
my second key: my second value # my second comment

# this line has no key/value
this line: has no comment 
",
    )
    .unwrap();

    assert_eq!(yaml.entries.len(), 6);

    let entry = &yaml.entries[0];
    assert!(entry.key_value.is_none());
    assert!(entry.comment.is_none());
    assert_eq!(entry.line, 1);

    let entry = &yaml.entries[1];
    assert!(entry.key_value.is_some());
    assert!(entry.comment.is_some());
    assert_eq!(entry.line, 2);
    let (key, value) = entry.key_value.as_ref().unwrap();
    let comment = entry.comment.as_ref().unwrap();
    assert_eq!(key, "my first key");
    assert_eq!(value, "my first value");
    assert_eq!(comment, "my first comment");

    let entry = &yaml.entries[2];
    assert!(entry.key_value.is_some());
    assert!(entry.comment.is_some());
    assert_eq!(entry.line, 3);
    let (key, value) = entry.key_value.as_ref().unwrap();
    let comment = entry.comment.as_ref().unwrap();
    assert_eq!(key, "my second key");
    assert_eq!(value, "my second value");
    assert_eq!(comment, "my second comment");

    let entry = &yaml.entries[3];
    assert!(entry.key_value.is_none());
    assert!(entry.comment.is_none());
    assert_eq!(entry.line, 4);

    let entry = &yaml.entries[4];
    assert!(entry.key_value.is_none());
    assert!(entry.comment.is_some());
    assert_eq!(entry.line, 5);
    let comment = entry.comment.as_ref().unwrap();
    assert_eq!(comment, "this line has no key/value");

    let entry = &yaml.entries[5];
    assert!(entry.key_value.is_some());
    assert!(entry.comment.is_none());
    assert_eq!(entry.line, 6);
    let (key, value) = entry.key_value.as_ref().unwrap();
    assert_eq!(key, "this line");
    assert_eq!(value, "has no comment");
}

#[test]
fn should_not_parse_when_key_is_empty() {
    let yaml = RisYaml::deserialize(": my value");
    assert!(yaml.is_err());
}

#[test]
fn should_not_parse_when_value_is_invalid() {
    let yaml = RisYaml::deserialize("my key: my:value");
    assert!(yaml.is_err());
}
