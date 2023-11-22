use ris_util::error::RisResult;

#[derive(Default, Debug)]
pub struct RisYamlEntry {
    pub key_value: Option<(String, String)>,
    pub comment: Option<String>,
    pub line: usize,
}

#[derive(Default)]
pub struct RisYaml {
    pub entries: Vec<RisYamlEntry>,
}

impl RisYaml {
    pub fn add_empty(&mut self) {
        let entry = RisYamlEntry::default();
        self.entries.push(entry);
    }

    pub fn add_key_value(&mut self, key: &str, value: String) {
        let mut entry = RisYamlEntry::default();
        entry.key_value = Some((key.to_owned(), value));
        self.entries.push(entry);
    }

    pub fn add_comment(&mut self, comment: &str) {
        let mut entry = RisYamlEntry::default();
        entry.comment = Some(comment.to_owned());
        self.entries.push(entry);
    }

    pub fn add_key_value_and_comment(&mut self, key: &str, value: &str, comment: &str) {
        let mut entry = RisYamlEntry::default();
        entry.key_value = Some((key.to_owned(), value.to_owned()));
        entry.comment = Some(comment.to_owned());
        self.entries.push(entry);
    }

    pub fn from(string: &str) -> RisResult<Self> {
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
                                assert_valid_key(&key, i)?;
                                assert_valid_value(&value, i)?;

                                if key.is_empty() {
                                    return error_on_line(i, "key may not be empty");
                                } else {
                                    Some((key, value))
                                }
                            }
                            (None, None) => None,
                            _ => {
                                return error_on_line(
                                    i,
                                    &format!("invalid syntax \"{}\"", key_value),
                                )
                            }
                        }
                    }
                }
                None => None,
            };

            let entry = RisYamlEntry {
                key_value,
                comment,
                line: i + 1,
            };

            entries.push(entry);
        }

        Ok(RisYaml { entries })
    }

    pub fn to_string(&self) -> RisResult<String> {
        let mut result = String::new();

        for (i, entry) in self.entries.iter().enumerate() {
            if let Some((key, value)) = &entry.key_value {
                assert_valid_key(key, i)?;
                assert_valid_value(value, i)?;

                result.push_str(&format!("{}: {} ", key, value));
            }

            if let Some(comment) = &entry.comment {
                assert_valid_comment(comment, i)?;

                result.push_str(&format!("# {}", comment));
            }

            result.push('\n');
        }

        Ok(result)
    }
}

pub fn error_on_line<T>(line: usize, message: &str) -> RisResult<T> {
    ris_util::result_err!("error on line {}: {}", line, message)
}

fn assert_valid_value(value: &str, line: usize) -> RisResult<()> {
    if value.contains('\n') {
        return error_on_line(
            line,
            &format!("value may not contain '\\n'. value: \"{}\"", value),
        );
    }

    if value.contains(':') {
        return error_on_line(
            line,
            &format!("value may not contain ':'. value: \"{}\"", value),
        );
    }

    Ok(())
}

fn assert_valid_key(key: &str, line: usize) -> RisResult<()> {
    if key.is_empty() {
        return error_on_line(line, "key may not be empty");
    }

    if key.contains('\n') {
        return error_on_line(
            line,
            &format!("key may not contain '\\n'. key: \"{}\"", key),
        );
    }

    if key.contains(':') {
        return error_on_line(line, &format!("key may not contain ':'. key: \"{}\"", key));
    }

    Ok(())
}

fn assert_valid_comment(comment: &str, line: usize) -> RisResult<()> {
    if comment.contains('\n') {
        error_on_line(
            line,
            &format!("comment may not contain '\\n'. comment: \"{}\"", comment),
        )
    } else {
        Ok(())
    }
}
