// JSON implemented in Rust
// original standard: https://www.rfc-editor.org/rfc/rfc8259

pub const BEGIN_ARRAY: char = '[';
pub const BEGIN_OBJECT: char = '{';
pub const END_ARRAY: char = ']';
pub const END_OBJECT: char = '}';
pub const NAME_SEPARATOR: char = ':';
pub const VALUE_SEPARATOR: char = ',';
pub const WS_SPACE: char = '\u{20}';
pub const WS_HORIZONTAL_TAB: char = '\u{09}';
pub const WS_LINE_FEED: char = '\u{0A}';
pub const WS_CARRIAGE_RETURN: char = '\u{0D}';
pub const TRUE: &str = "true";
pub const NULL: &str = "null";
pub const FALSE: &str = "false";
pub const DECIMAL_POINT: char = '.';
pub const E: [char; 2] = ['e', 'E'];
pub const MINUS: char = '-';
pub const PLUS: char = '+';
pub const ZERO: char = '0';
pub const ESCAPE: char = '\\';
pub const QUOTATION_MARK: char = '"';

// structs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Object(Box<JsonObject>),
    Array(Vec<JsonValue>),
    Number(JsonNumber),
    String(String),
}

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
pub struct JsonNumber {
    inner: String, // storing it as any kind of number is a major footgun
}

impl Default for JsonValue {
    fn default() -> Self {
        Self::Null
    }
}

impl Default for JsonNumber {
    fn default() -> Self {
        Self{inner: "0".to_string()}
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonError {
    InvalidCast,
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCast => write!(f, "specified cast is not valid"),
        }
    }
}

impl std::error::Error for JsonError {}

// conversion
impl From<bool> for JsonValue {
    fn from(value: bool) -> Self {
        JsonValue::Boolean(value)
    }
}

impl TryFrom<JsonValue> for bool {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Boolean(result) => Ok(result),
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl From<JsonObject> for JsonValue {
    fn from(value: JsonObject) -> Self {
        JsonValue::Object(Box::new(value))
    }
}

impl TryFrom<JsonValue> for JsonObject {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Object(result) => Ok(*result),
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl<T: Into<JsonValue> + Clone> From<&[T]> for JsonValue {
    fn from(value: &[T]) -> Self {
        let array = value
            .into_iter()
            .map(|x| x.clone().into())
            .collect();
        JsonValue::Array(array)
    }
}

impl<T: Into<JsonValue> + Clone, const N: usize> From<&[T; N]> for JsonValue {
    fn from(value: &[T; N]) -> Self {
        let slice: &[_] = value;
        JsonValue::from(slice)
    }
}

impl From<i32> for JsonValue {
    fn from(value: i32) -> Self {
        Self::Number(JsonNumber{inner: value.to_string()})
    }
}

impl TryFrom<JsonValue> for i32 {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(JsonNumber {inner}) => {
                // safety: construction of number that cannot be parsed should be impossible
                Ok(inner.parse().unwrap())
            },
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl From<usize> for JsonValue {
    fn from(value: usize) -> Self {
        Self::Number(JsonNumber{
            inner: value.to_string()
        })
    }
}

impl TryFrom<JsonValue> for usize {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(JsonNumber {inner}) => {
                // safety: construction of number that cannot be parsed should be impossible
                Ok(inner.parse().unwrap())
            },
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl From<isize> for JsonValue {
    fn from(value: isize) -> Self {
        Self::Number(JsonNumber{
            inner: value.to_string()
        })
    }
}

impl TryFrom<JsonValue> for isize {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(JsonNumber {inner}) => {
                // safety: construction of number that cannot be parsed should be impossible
                Ok(inner.parse().unwrap())
            },
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl From<f32> for JsonValue {
    fn from(value: f32) -> Self {
        if value.is_infinite() {
            panic!("{}", JsonError::InvalidCast);
        }

        if value.is_nan() {
            panic!("{}", JsonError::InvalidCast);
        }

        Self::Number(JsonNumber{
            inner: format!("{:?}", value),
        })
    }
}

impl TryFrom<JsonValue> for f32 {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(JsonNumber {inner}) => {
                // safety: construction of number that cannot be parsed should be impossible
                Ok(inner.parse().unwrap())
            },
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl From<String> for JsonValue {
    fn from(value: String) -> Self {
        JsonValue::String(value)
    }
}

impl TryFrom<JsonValue> for String {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::String(result) => Ok(result),
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl From<&str> for JsonValue {
    fn from(value: &str) -> Self {
        JsonValue::String(value.to_string())
    }
}

// member functions
impl JsonObject {
    pub fn get(&self, name: impl AsRef<str>) -> Option<&JsonValue> {
        let index = name.as_ref();
        self.members
            .iter()
            .rfind(|x| x.name == index)
            .map(|x| &x.value)
    }

    pub fn get_mut(&mut self, name: impl AsRef<str>) -> Option<&mut JsonValue> {
        let index = name.as_ref();
        self.members
            .iter_mut()
            .rfind(|x| x.name == index)
            .map(|x| &mut x.value)
    }

    pub fn push(&mut self, name: impl AsRef<str>, value: impl Into<JsonValue>) {
        let name = name.as_ref().to_string();
        let member = JsonMember{
            name,
            value: value.into(),
        };
        self.members.push(member);
    }
}

// serialization
impl JsonValue {
    pub fn serialize(&self) -> String {
        match self {
            JsonValue::Null => NULL.to_string(),
            JsonValue::Boolean(true) => TRUE.to_string(),
            JsonValue::Boolean(false) => FALSE.to_string(),
            JsonValue::Object(value) => value.serialize(),
            JsonValue::Array(values) => {
                let mut serialized_values = Vec::with_capacity(values.len());
                for value in values.iter() {
                    let serialized_value = value.serialize();
                    serialized_values.push(serialized_value);
                }

                let serialized_array = serialized_values.join(&VALUE_SEPARATOR.to_string());

                format!(
                    "{}{}{}",
                    BEGIN_ARRAY,
                    serialized_array,
                    END_ARRAY,
                )
            },
            JsonValue::Number(number) => {
                number.inner.clone()
            },
            JsonValue::String(value) => {
                let mut serialized_string = value.clone();

                let escape = |x: &mut String, c: char| {
                    *x = x.replace(c, &format!("{}{}", ESCAPE, c))
                };

                escape(&mut serialized_string, QUOTATION_MARK);
                escape(&mut serialized_string, ESCAPE);

                for i in 0..=0x1f {
                    let c = i as u8 as char;
                    escape(&mut serialized_string, c);
                }

                format!(
                    "{}{}{}",
                    QUOTATION_MARK,
                    serialized_string,
                    QUOTATION_MARK,
                )
            },
        }
    }

    pub fn deserialize(value: impl AsRef<str>) -> Self {
        let value = value.as_ref();

        let mut line = 1;
        let mut column = 1;

        // ignore potential U+FEFF
        // parse escaped character, including optional escape characters
        panic!()
    }
}


impl JsonObject {
    pub fn serialize(&self) -> String {
        let mut serialized_members = Vec::with_capacity(self.members.len());
        for member in self.members.iter() {
            let serialized_member = member.serialize();
            serialized_members.push(serialized_member);
        }

        let serialized_object = serialized_members.join(&VALUE_SEPARATOR.to_string());

        format!(
            "{}{}{}",
            BEGIN_OBJECT,
            serialized_object,
            END_OBJECT,
        )
    }
}

impl JsonMember {
    pub fn serialize(&self) -> String {
        let name = JsonValue::String(self.name.clone());
        let serialized_name = name.serialize();
        let serialized_value = self.value.serialize();
        format!(
            "{}{}{}",
            serialized_name,
            NAME_SEPARATOR,
            serialized_value,
        )
    }
}

