// JSON implemented in Rust
// original standard: https://www.rfc-editor.org/rfc/rfc8259

pub const BEGIN_ARRAY: char = '[';
pub const BEGIN_OBJECT: char = '{';
pub const END_ARRAY: char = ']';
pub const END_OBJECT: char = '}';
pub const NAME_SEPARATOR: char = ':';
pub const VALUE_SEPARATOR: char = ',';
pub const WS_CARRIAGE_RETURN: char = '\u{000D}';
pub const WS_HORIZONTAL_TAB: char = '\u{0009}';
pub const WS_LINE_FEED: char = '\u{000A}';
pub const WS_SPACE: char = '\u{0020}';
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
pub const BYTE_ORDER_MARK: char = '\u{FEFF}';

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
    inner: String, // storing a json number as any kind or combination of 
                   // ints or floats leads to many footguns. thus we store 
                   // it as a string and convert only when the user 
                   // attempts to read the number
}

impl Default for JsonValue {
    fn default() -> Self {
        Self::Null
    }
}

impl Default for JsonNumber {
    fn default() -> Self {
        Self {
            inner: ZERO.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonError {
    InvalidCast,
    SyntaxError,
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCast => write!(f, "specified cast is not valid"),
            Self::SyntaxError => write!(f, "non valid json syntax"),
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

impl TryFrom<&JsonValue> for bool {
    type Error = JsonError;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Boolean(result) => Ok(*result),
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl From<JsonObject> for JsonValue {
    fn from(value: JsonObject) -> Self {
        JsonValue::Object(Box::new(value))
    }
}

impl TryFrom<&JsonValue> for JsonObject {
    type Error = JsonError;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Object(object) => Ok(*object.clone()),
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl<'a> TryFrom<&'a JsonValue> for &'a JsonObject {
    type Error = JsonError;

    fn try_from(value: &'a JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Object(object) => Ok(object),
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl<'a> TryFrom<&'a JsonValue> for Vec<&'a JsonObject> {
    type Error = JsonError;

    fn try_from(value: &'a JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Array(array) => {
                array.iter()
                    .map(|x| match x {
                        JsonValue::Object(object) => Ok(&**object),
                        _ => Err(JsonError::InvalidCast),
                    })
                    .collect::<Result<Vec<_>, JsonError>>()
            },
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl<T: Into<JsonValue> + Clone> From<&[T]> for JsonValue {
    fn from(value: &[T]) -> Self {
        let array = value.iter().map(|x| x.clone().into()).collect();
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
        Self::Number(JsonNumber {
            inner: value.to_string(),
        })
    }
}

impl TryFrom<&JsonValue> for i32 {
    type Error = JsonError;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(JsonNumber { inner }) => {
                // safety: construction of invalid number should be impossible
                Ok(inner.parse().unwrap())
            }
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl From<usize> for JsonValue {
    fn from(value: usize) -> Self {
        Self::Number(JsonNumber {
            inner: value.to_string(),
        })
    }
}

impl TryFrom<&JsonValue> for JsonNumber {
    type Error = JsonError;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(number) => Ok(number.clone()),
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl TryFrom<&JsonValue> for usize {
    type Error = JsonError;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(JsonNumber { inner }) => {
                // safety: construction of invalid number should be impossible
                Ok(inner.parse().unwrap())
            }
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl<'a> TryFrom<&'a JsonValue> for Vec<usize> {
    type Error = JsonError;

    fn try_from(value: &'a JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Array(array) => {
                array.iter()
                    .map(|x| usize::try_from(x))
                    .collect::<Result<Vec<_>, JsonError>>()
            },
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl From<isize> for JsonValue {
    fn from(value: isize) -> Self {
        Self::Number(JsonNumber {
            inner: value.to_string(),
        })
    }
}

impl TryFrom<&JsonValue> for isize {
    type Error = JsonError;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(JsonNumber { inner }) => {
                // safety: construction of invalid number should be impossible
                Ok(inner.parse().unwrap())
            }
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

        Self::Number(JsonNumber {
            inner: format!("{:?}", value),
        })
    }
}

impl TryFrom<&JsonValue> for f32 {
    type Error = JsonError;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Number(JsonNumber { inner }) => {
                // safety: construction of number that cannot be parsed should be impossible
                Ok(inner.parse().unwrap())
            }
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl<'a> TryFrom<&'a JsonValue> for Vec<f32> {
    type Error = JsonError;

    fn try_from(value: &'a JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Array(array) => {
                array.iter()
                    .map(|x| f32::try_from(x))
                    .collect::<Result<Vec<_>, JsonError>>()
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

impl TryFrom<&JsonValue> for String {
    type Error = JsonError;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::String(result) => Ok(result.clone()),
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl From<&str> for JsonValue {
    fn from(value: &str) -> Self {
        JsonValue::String(value.to_string())
    }
}

impl<'a> TryFrom<&'a JsonValue> for &'a str {
    type Error = JsonError;

    fn try_from(value: &'a JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::String(string) => Ok(&string),
            _ => Err(JsonError::InvalidCast),
        }
    }
}

impl<'a> TryFrom<&JsonValue> for Vec<String> {
    type Error = JsonError;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Array(array) => {
                array.iter()
                    .map(|x| match x {
                        JsonValue::String(string) => Ok(string.clone()),
                        _ => Err(JsonError::InvalidCast),
                    })
                    .collect::<Result<Vec<_>, JsonError>>()
            },
            _ => Err(JsonError::InvalidCast),
        }
    }
}

// member functions
impl JsonObject {
    pub fn get<'a, T: TryFrom<&'a JsonValue>>(&'a self, name: impl AsRef<str>) -> Option<T> {
        let index = name.as_ref();
        let value = self.members
            .iter()
            .rfind(|x| x.name == index)
            .map(|x| T::try_from(&x.value));

        match value {
            Some(Ok(value)) => Some(value),
            _ => None,
        }
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
        let member = JsonMember {
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
            JsonValue::Object(object) => object.serialize(),
            JsonValue::Array(array) => {
                let mut serialized_values = Vec::with_capacity(array.len());
                for value in array.iter() {
                    let serialized_value = value.serialize();
                    serialized_values.push(serialized_value);
                }

                let serialized_array = serialized_values.join(&VALUE_SEPARATOR.to_string());

                format!("{}{}{}", BEGIN_ARRAY, serialized_array, END_ARRAY,)
            }
            JsonValue::Number(number) => number.inner.clone(),
            JsonValue::String(string) => {
                let mut serialized_string = string.clone();

                let escape =
                    |x: &mut String, c: char| *x = x.replace(c, &format!("{}{}", ESCAPE, c));

                escape(&mut serialized_string, QUOTATION_MARK);
                escape(&mut serialized_string, ESCAPE);

                for i in 0..=0x1f {
                    let c = i as u8 as char;
                    escape(&mut serialized_string, c);
                }

                format!("{}{}{}", QUOTATION_MARK, serialized_string, QUOTATION_MARK,)
            }
        }
    }

    pub fn deserialize(value: impl AsRef<str>) -> Result<Self, JsonError> {
        let value = value.as_ref();

        let mut is_string = false;
        let mut tokens = vec![String::new()];

        // ignore byte order mark (U+FEFF) for interopability. see https://www.rfc-editor.org/rfc/rfc8259#section-8.1
        let skip = if value.starts_with(BYTE_ORDER_MARK) {
            1
        } else {
            0
        };

        // lexical analysis: turn the character sequence into a token sequence
        for c in value.chars().skip(skip) {
            let last_index = tokens.len() - 1;
            let token = &mut tokens[last_index];

            if c == QUOTATION_MARK {
                token.push(c);
                is_string = !is_string;

                if !is_string {
                    tokens.push(String::new());
                }

                continue;
            }

            if is_string {
                token.push(c);
                continue;
            }

            match c {
                WS_CARRIAGE_RETURN | WS_SPACE | WS_HORIZONTAL_TAB => {
                    if token.is_empty() {
                        continue;
                    }

                    tokens.push(String::new());
                }
                WS_LINE_FEED => {
                    if token.is_empty() {
                        continue;
                    }

                    tokens.push(String::new());
                }
                BEGIN_ARRAY | BEGIN_OBJECT | END_ARRAY | END_OBJECT | NAME_SEPARATOR
                | VALUE_SEPARATOR => {
                    if !token.is_empty() {
                        tokens.push(String::new());
                    }

                    let last_index = tokens.len() - 1;
                    let token = &mut tokens[last_index];
                    token.push(c);
                    tokens.push(String::new());
                }
                _ => token.push(c),
            }
        }

        // because of the particular lexer logic used, the last token may be empty. empty tokens
        // are illegal and thus the last token may be removed
        let last_index = tokens.len() - 1;
        let last = &tokens[last_index];
        if last.is_empty() {
            tokens.remove(last_index);
        }

        // parse syntax
        Self::from_tokens(&tokens)
    }

    fn from_tokens(tokens: &[String]) -> Result<Self, JsonError> {
        let len = tokens.len();
        if len == 0 {
            return Err(JsonError::SyntaxError);
        }

        if len == 1 {
            let token = &tokens[0];
            return Self::from_token(token);
        }

        // multiple tokens, separate them
        let first_token = &tokens[0];
        let last_token = &tokens[len - 1];

        // multiple tokens mean one of two scenarios: we are dealing with an object, or an array.
        // both start and end with a single character token, so we do an early check if the tokens
        // have a length of 1
        if first_token.len() != 1 {
            return Err(JsonError::SyntaxError);
        }

        if last_token.len() != 1 {
            return Err(JsonError::SyntaxError);
        }

        let first = first_token.chars().next().unwrap();
        let last = last_token.chars().next().unwrap();

        let is_object = first == BEGIN_OBJECT && last == END_OBJECT;
        let is_array = first == BEGIN_ARRAY && last == END_ARRAY;

        if !is_object && !is_array {
            return Err(JsonError::SyntaxError);
        }

        let mut object_generation = 0;
        let mut array_generation = 0;

        let mut elements = vec![Vec::new()];

        let start = 1;
        let end = tokens.len() - 1;
        for token in &tokens[start..end] {
            match token.len() {
                0 => return Err(JsonError::SyntaxError),
                1 => match token.chars().next().unwrap() {
                    BEGIN_OBJECT => object_generation += 1,
                    BEGIN_ARRAY => array_generation += 1,
                    END_OBJECT => object_generation -= 1,
                    END_ARRAY => array_generation -= 1,
                    VALUE_SEPARATOR => {
                        if object_generation == 0 && array_generation == 0 {
                            elements.push(Vec::new());
                            continue;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }

            if object_generation < 0 || array_generation < 0 {
                return Err(JsonError::SyntaxError);
            }

            let last_index = elements.len() - 1;
            let element = &mut elements[last_index];
            element.push(token.clone());
        }

        let result = if is_object {
            let members = elements
                .into_iter()
                .map(|x| JsonMember::from_tokens(&x))
                .collect::<Result<Vec<_>, JsonError>>()?;
            JsonValue::Object(Box::new(JsonObject { members }))
        } else {
            // is_array
            let values = elements
                .into_iter()
                .map(|x| Self::from_tokens(&x))
                .collect::<Result<Vec<_>, JsonError>>()?;
            JsonValue::Array(values)
        };

        Ok(result)
    }

    fn from_token(token: &str) -> Result<Self, JsonError> {
        // try parse literal
        match token {
            FALSE => return Ok(JsonValue::Boolean(false)),
            NULL => return Ok(JsonValue::Null),
            TRUE => return Ok(JsonValue::Boolean(true)),
            _ => (),
        }

        // try parse string
        if token.starts_with(QUOTATION_MARK) && token.ends_with(QUOTATION_MARK) {
            let inner: String = token
                .chars()
                .skip(1)
                .take(token.chars().count() - 2)
                .collect();

            // unescape characters
            let mut result = String::new();
            let mut char_iter = inner.chars();
            while let Some(c) = char_iter.next() {
                if c != ESCAPE {
                    result.push(c);
                    continue;
                }

                let Some(c) = char_iter.next() else {
                    return Err(JsonError::SyntaxError);
                };

                match c {
                    '"' => result.push(QUOTATION_MARK),
                    '\\' => result.push(ESCAPE),
                    '/' => result.push('/'),
                    'b' => result.push('\u{0008}'),
                    'f' => result.push('\u{000C}'),
                    'n' => result.push(WS_LINE_FEED),
                    'r' => result.push(WS_CARRIAGE_RETURN),
                    't' => result.push(WS_HORIZONTAL_TAB),
                    'u' => {
                        let digit_1 = char_iter.next().ok_or(JsonError::SyntaxError)?;
                        let digit_2 = char_iter.next().ok_or(JsonError::SyntaxError)?;
                        let digit_3 = char_iter.next().ok_or(JsonError::SyntaxError)?;
                        let digit_4 = char_iter.next().ok_or(JsonError::SyntaxError)?;

                        let hex_codepoint =
                            format!("{}{}{}{}", digit_1, digit_2, digit_3, digit_4,);
                        let codepoint = u32::from_str_radix(&hex_codepoint, 16)
                            .map_err(|_| JsonError::SyntaxError)?;
                        let character = char::from_u32(codepoint).ok_or(JsonError::SyntaxError)?;

                        result.push(character);
                    }
                    _ => return Err(JsonError::SyntaxError),
                }
            }

            return Ok(JsonValue::String(result));
        }

        // try parse number
        if token.parse::<f32>().is_ok() {
            return Ok(JsonValue::Number(JsonNumber {
                inner: token.to_string(),
            }));
        }

        // invalid token
        Err(JsonError::SyntaxError)
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

        format!("{}{}{}", BEGIN_OBJECT, serialized_object, END_OBJECT,)
    }
}

impl JsonMember {
    pub fn serialize(&self) -> String {
        let name = JsonValue::String(self.name.clone());
        let serialized_name = name.serialize();
        let serialized_value = self.value.serialize();
        format!("{}{}{}", serialized_name, NAME_SEPARATOR, serialized_value,)
    }

    fn from_tokens(tokens: &[String]) -> Result<Self, JsonError> {
        if tokens.len() < 3 {
            return Err(JsonError::SyntaxError);
        }

        let name_token = &tokens[0];
        let separator_token = &tokens[1];
        let value_tokens = &tokens[2..];

        let name_value = JsonValue::from_tokens(&[name_token.clone()])?;
        let JsonValue::String(name) = name_value else {
            return Err(JsonError::SyntaxError);
        };

        if separator_token.len() != 1 {
            return Err(JsonError::SyntaxError);
        }

        let separator = separator_token.chars().next().unwrap();
        if separator != NAME_SEPARATOR {
            return Err(JsonError::SyntaxError);
        }

        let value = JsonValue::from_tokens(value_tokens)?;

        Ok(Self { name, value })
    }
}
