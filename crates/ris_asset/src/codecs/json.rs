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

// structs
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
    Null,
    Boolean(bool),
    Object(Box<JsonObject>),
    Array(Vec<JsonValue>),
    Number(JsonNumber),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct JsonNumber {
    pub minus: Option<()>,
    pub int: usize,
    pub frac: Option<f32>,
    pub exp: Option<i32>,
}

impl Eq for JsonNumber {} // cannot derive Eq and must be implemented manually, because of some
                          // bullshit NaN reason. What if I don't care? NaN is not equal Nan, but
                          // this should be fine, and is imo no reason to not implement Eq

impl Default for JsonValue {
    fn default() -> Self {
        Self::Null
    }
}

impl Default for JsonNumber {
    fn default() -> Self {
        Self{
            minus: None,
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

impl From<usize> for JsonValue {
    fn from(value: usize) -> Self {
        Self::Number(JsonNumber{
            minus: None,
            int: value,
            frac: None,
            exp: None,
        })
    }
}

impl TryFrom<JsonValue> for usize {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(JsonNumber { minus: None, int, frac: None, exp: None }) => {
                Ok(int)
            },
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl TryFrom<isize> for JsonValue {
    type Error = JsonError;

    fn try_from(mut value: isize) -> Result<Self, Self::Error> {
        let minus = if value.is_negative() {
            value = match value.checked_mul(-1) {
                Some(positive) => positive,
                None => return Err(JsonError::MathOverflow),
            };
            Some(())
        } else {
            None
        };

        let int = value as usize;

        Ok(Self::Number(JsonNumber{
            minus,
            int,
            frac: None,
            exp: None,
        }))
    }
}

impl TryFrom<JsonValue> for isize {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(JsonNumber { minus, int, frac: None, exp: None }) => {
                let sign: isize = if minus.is_some() {
                    -1
                } else {
                    1
                };

                let int: isize = int.try_into().map_err(|_| JsonError::MathOverflow)?;
                match int.checked_mul(sign) {
                    Some(result) => Ok(result),
                    None => Err(JsonError::MathOverflow),
                }
            },
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl TryFrom<f32> for JsonValue {
    type Error = JsonError;

    fn try_from(mut value: f32) -> Result<Self, Self::Error> {
        if value.is_infinite() {
            return Err(JsonError::InvalidNumber);
        }

        if value.is_nan() {
            return Err(JsonError::InvalidNumber);
        }

        let minus = if value.is_sign_negative() {
            value *= -1.0;
            Some(())
        } else {
            None
        };

        let fract = value.fract();
        let frac = if fract == 0.0 {
            None
        } else {
            Some(fract)
        };

        let int = value as usize;

        Ok(Self::Number(JsonNumber{
            minus,
            int,
            frac,
            exp: None,
        }))
    }
}

impl TryFrom<JsonValue> for f32 {
    type Error = JsonError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(number) => {
                let sign = if number.minus.is_some() {
                    -1.0
                } else {
                    1.0
                };

                let int = number.int as f32;
                let frac = match number.frac {
                    Some(frac) => frac,
                    None => 0.0,
                };

                let exp = match number.exp {
                    Some(exp) => f32::powi(10.0, exp),
                    None => 1.0
                };

                let result = sign * (int + frac) * exp;
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

// member functions
impl JsonObject {
    fn get(&self, name: impl AsRef<str>) -> Option<&JsonValue> {
        let index = name.as_ref();
        self.members
            .iter()
            .rfind(|x| x.name == index)
            .map(|x| &x.value)
    }

    fn get_mut(&mut self, name: impl AsRef<str>) -> Option<&mut JsonValue> {
        let index = name.as_ref();
        self.members
            .iter_mut()
            .rfind(|x| x.name == index)
            .map(|x| &mut x.value)
    }
}

// serialization
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

    pub fn deserialize(value: impl AsRef<str>) -> Self {
        let value = value.as_ref();
        panic!()
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
                let serialized_minus = if number.minus.is_some() {
                    MINUS.to_string()
                } else {
                    "".to_string()
                };

                let serialized_int = number.int.to_string();

                let serialized_frac = match number.frac {
                    Some(frac) if frac != 0.0 => {
                        let frac_string = frac.to_string();
                        frac_string.trim_start_matches(ZERO).to_string()
                    },
                    _ => "".to_string(),
                };

                let serialized_exp = match number.exp {
                    Some(exp) => {
                        format!("{}{}", E[0], exp)
                    },
                    None => "".to_string(),
                };

                format!(
                    "{}{}{}{}",
                    serialized_minus,
                    serialized_int,
                    serialized_frac,
                    serialized_exp,
                )
            },
            JsonValue::String(value) => {
                todo!()
            },
        }
    }
}

