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
    pub int: i32,
    pub frac: Option<u32>,
    pub exp: Option<i32>,
}

impl Default for JsonValue {
    fn default() -> Self {
        Self::Null
    }
}

impl Default for JsonNumber {
    fn default() -> Self {
        Self{
            int: 0,
            frac: None,
            exp: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonError {
    InvalidCast,
    InvalidNumber,
    MathOverflow,
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCast => write!(f, "specified cast is not valid"),
            Self::InvalidNumber => write!(f, "the number is not supported by JSON"),
            Self::MathOverflow => write!(f, "an operation caused a math overflow"),
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
    fn from(mut value: i32) -> Self {
        let minus = if value.is_negative() {
            value *= -1;
            Some(())
        } else {
            None
        };

        Self::Number(JsonNumber{
            int: value,
            frac: None,
            exp: None,
        })
    }
}

impl TryFrom<JsonValue> for i32 {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(JsonNumber { int, frac: None, exp: None }) => {
                Ok(int as i32)
            },
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl From<usize> for JsonValue {
    fn from(value: usize) -> Self {
        Self::Number(JsonNumber{
            int: value as i32,
            frac: None,
            exp: None,
        })
    }
}

impl TryFrom<JsonValue> for usize {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(JsonNumber { int, frac: None, exp: None }) => {
                Ok(int as usize)
            },
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl From<isize> for JsonValue {
    fn from(mut value: isize) -> Self {
        let minus = if value.is_negative() {
            value *= -1;
            Some(())
        } else {
            None
        };

        let int = value as i32;

        Self::Number(JsonNumber{
            int,
            frac: None,
            exp: None,
        })
    }
}

impl TryFrom<JsonValue> for isize {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(JsonNumber { int, frac: None, exp: None }) => {
                Ok(int as isize)
            },
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl From<f32> for JsonValue {
    fn from(mut value: f32) -> Self {
        if value.is_infinite() {
            panic!("{}", JsonError::InvalidNumber);
        }

        if value.is_nan() {
            panic!("{}", JsonError::InvalidNumber);
        }

        let sign = ;

        let fract = value.fract();
        let frac = if fract == 0.0 {
            None
        } else {
            let frac = format!("{}", fract);
            let frac = frac
                .trim_start_matches('-')
                .trim_start_matches('0')
                .trim_start_matches('.');
            println!("deine mom: {:?}", frac);
            let frac = frac.parse().expect(&format!("{}", JsonError::InvalidCast));

            Some(frac)
        };

        let int = value as i32;

        Self::Number(JsonNumber{
            int,
            frac,
            exp: None,
        })
    }
}

impl TryFrom<JsonValue> for f32 {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(number) => {
                let int = number.int as f32;
                let frac = match number.frac {
                    Some(frac) => match format!("0.{}", frac).parse() {
                        Ok(parsed) => parsed,
                        Err(_) => return Err(JsonError::InvalidCast),
                    },
                    None => 0.0,
                };

                let exp = match number.exp {
                    Some(exp) => f32::powi(10.0, exp),
                    None => 1.0
                };

                let result = (int + frac) * exp;
                Ok(result)
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
                let serialized_int = number.int.to_string();

                let serialized_frac = match number.frac {
                    Some(frac) => format!(".{}", frac),
                    _ => "".to_string(),
                };

                let serialized_exp = match number.exp {
                    Some(exp) => {
                        format!("{}{}", E[0], exp)
                    },
                    None => "".to_string(),
                };

                format!(
                    "{}{}{}",
                    serialized_int,
                    serialized_frac,
                    serialized_exp,
                )
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

