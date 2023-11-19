use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::io::Seek;

use ris_util::error::RisResult;

#[derive(Default, Debug)]
pub struct RisYamlEntry {
    pub key_value: Option<(String, String)>,
    pub comment: Option<String>,
    pub line: usize,
}

#[derive(Default)]
pub struct RisYaml{
    pub entries: Vec<RisYamlEntry>,
}

pub fn serialize(yaml: &RisYaml) -> String {
    let mut result = String::new();
    let mut cursor = Cursor::new(&mut result);

    for (i, entry) in yaml.entries.iter().enumerate() {
        let mut was_written = false;

        if let Some((key, value)) = &entry.key_value {
            result.push_str(&format!("{}: {}", key, value));
            was_written = true;
        }

        if let Some(comment) = &entry.comment {
            result.push_str(&format!(" # {}", comment));
            was_written = true;
        }

        if was_written {
            result.push('\n');
        }
    }

    result
}

pub fn deserialize(string: String) -> RisResult<RisYaml> {
    let mut entries = Vec::new();

    let mut cursor = Cursor::new(bytes);

    Ok(RisYaml{entries})
}
