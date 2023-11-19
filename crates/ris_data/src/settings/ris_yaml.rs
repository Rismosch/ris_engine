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

pub fn serialize(yaml: &RisYaml) -> RisResult<String> {
    let mut result = String::new();

    for (i, entry) in yaml.entries.iter().enumerate() {
        if let Some((key, value)) = &entry.key_value {
            assert_valid_key_value(key, i)?;
            assert_valid_key_value(value, i)?;

            result.push_str(&format!("{}: {}", key, value));
        }

        if let Some(comment) = &entry.comment {
            assert_no_newline(comment, i)?;

            result.push_str(&format!(" # {}", comment));
        }

        result.push('\n');
    }

    Ok(result)
}

pub fn deserialize(string: String) -> RisResult<RisYaml> {
    let mut entries = Vec::new();

    for (i, line) in string.lines().enumerate() {
        let trimmed = line.trim();

        let mut comment_splits = trimmed.splitn(2, '#').map(|s| s.trim().to_string());

        let key_value_str = comment_splits.next();
        let comment = comment_splits.next();

        let key_value = match key_value_str {
            Some(key_value) => {
                if key_value.is_empty() {
                    None
                } else {
                    let mut splits = key_value.splitn(2, ':').map(|s| s.trim().to_string());

                    let key = splits.next();
                    let value = splits.next();

                    match (key, value) {
                        (Some(key), Some(value)) => {
                            assert_valid_key_value(&key, i)?;
                            assert_valid_key_value(&value, i)?;

                            if key.is_empty() {
                                return error_on_line(i, "key may not be empty");
                            } else {
                                Some((key, value))
                            }
                        },
                        (None, None) => None,
                        _ => return error_on_line(i, &format!("invalid syntax \"{}\"", key_value)),
                    }
                }
            },
            None => None,
        };

        let entry = RisYamlEntry {
            key_value,
            comment,
            line: i,
        };

        entries.push(entry);
    }

    Ok(RisYaml{entries})
}

fn assert_valid_key_value(string: &str, line: usize) -> RisResult<()> {
    assert_no_newline(string, line)?;

    if string.contains(':') {
        error_on_line(line, &format!("string may not contain ':'. string: \"{}\"", string))
    } else {
        Ok(())
    }
}

fn assert_no_newline(string: &str, line: usize) -> RisResult<()> {
    if string.contains('\n') {
        error_on_line(line, &format!("string may not contain '\\n'. string: \"{}\"", string))
    } else {
        Ok(())
    }
}

fn error_on_line<T>(line: usize, message: &str) -> RisResult<T> {
    ris_util::result_err!("error on line {}: {}", line, message)
}
