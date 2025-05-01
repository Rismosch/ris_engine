pub enum SanitizeInfo {
    RemoveInvalidChars,
    RemoveInvalidCharsAndReplaceSlashes,
    RemoveInvalidCharsAndSlashes,
}

pub fn sanitize(value: impl AsRef<str>, info: SanitizeInfo) -> String {
    const INVALID_CHARS: [char; 7] = [':', '*', '?', '"', '<', '>', '|'];
    const WINDOWS_SLASH: char = '\\';
    const LINUX_SLASH: char = '/';
    const REPLACEMENT: &str = "_";

    let mut value = value.as_ref().to_string();

    for invalid_char in INVALID_CHARS {
        value = value.replace(invalid_char, REPLACEMENT);
    }

    match info {
        SanitizeInfo::RemoveInvalidChars => (),
        SanitizeInfo::RemoveInvalidCharsAndReplaceSlashes => {
            value = value.replace(WINDOWS_SLASH, &LINUX_SLASH.to_string());
        }
        SanitizeInfo::RemoveInvalidCharsAndSlashes => {
            value = value.replace(WINDOWS_SLASH, REPLACEMENT);
            value = value.replace(LINUX_SLASH, REPLACEMENT);
        }
    }

    value
}
