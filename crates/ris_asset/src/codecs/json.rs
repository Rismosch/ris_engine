// JSON implemented in Rust
// original standard: https://www.rfc-editor.org/rfc/rfc8259

pub const BEGIN_ARRAY: char = '[';
pub const BEGIN_OBJECT: char = '{';
pub const END_ARRAY: char = ']';
pub const END_OBJECT: char = '}';
pub const NAME_SEPARATOR: char = ':';
pub const VALUE_SEPARATOR: char = ',';
pub const WS: [char; 4] = [
    '\u{20}', // Space
    '\u{09}', // Horizontal tab
    '\u{0A}', // Line feed or New line
    '\u{0D}', // Carriage return
];
pub const TRUE: &str = "true";
pub const NULL: &str = "null";
pub const FALSE: &str = "false";

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct JsonObject {
    pub members: Vec<JsonMember>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct JsonMember {
    pub name: String,
    pub value: JsonValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonValue {
    String(String),
    Number(()),
    Boolean(bool),
    Null,
    Object(Box<JsonObject>),
    Array(Vec<JsonObject>),
}

impl Default for JsonValue {
    fn default() -> Self {
        Self::Null
    }
}

impl JsonObject {
    pub fn serialize(&self) -> String {
        let mut serialized_members = String::new();

        if let Some(member) = self.members.get(0) {
            serialized_members = member.serialize();
        }

        for member in self.members.iter().skip(1) {
            let serialized_member = format!(
                "{}{}",
                VALUE_SEPARATOR,
                member.serialize(),
            );
            serialized_members.push_str(&serialized_member);
        }

        format!(
            "{}{}{}",
            BEGIN_OBJECT,
            serialized_members,
            END_OBJECT,
        )
    }

    pub fn deserialize(value: impl AsRef<str>) -> Self {
        let value = value.as_ref();
        panic!()
    }
}

impl JsonMember {
    pub fn serialize(&self) -> String {
        let serialized_name = serialize_string(&self.name);
        let serialized_value = self.value.serialize();
        format!(
            "{}{}{}",
            serialized_name,
            NAME_SEPARATOR,
            serialized_value,
        )
    }
}

impl JsonValue {
    pub fn serialize(&self) -> String {
        panic!();
    }
}

fn serialize_string(value: impl AsRef<str>) -> String {
    let value = value.as_ref();
    panic!();
}

//fn remove_whitespace(value: impl AsRef<str>) -> String {
//    let value = value.as_ref();
//    let mut result = value.to_string();
//    for &ws in WS.iter() {
//        result = result.replace(ws, "");
//    }
//
//    result
//}
