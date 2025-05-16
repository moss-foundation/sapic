use regex::Regex;
use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};
use std::path::{Component, Path, PathBuf};
use std::sync::LazyLock;

/// Regex to match forbidden characters in a directory/file name
static FORBIDDEN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"[.%<>:"/\\|?*]"#).unwrap());
/// Function to encode forbidden characters and '%' in a directory/file name
fn encode(original: &str) -> String {
    FORBIDDEN_RE
        .replace_all(original, |caps: &regex::Captures| {
            // Replace each forbidden character with its hex representation (e.g., ':' -> %3A)
            format!("%{:02X}", caps[0].chars().next().unwrap() as u32)
        })
        .to_string()
}

/// Function to decode an encoded directory/file name back to its original form
fn decode(encoded: &str) -> anyhow::Result<String, std::num::ParseIntError> {
    let mut result = String::new();
    let mut chars = encoded.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            // Read the next two characters as a hex code
            let hex: String = chars.by_ref().take(2).collect();
            let value = u8::from_str_radix(&hex, 16)?;
            result.push(value as char);
        } else {
            result.push(ch);
        }
    }

    Ok(result)
}

pub struct SanitizedName(String);
impl SanitizedName {
    pub fn new(raw: &str) -> Self {
        Self(encode(raw))
    }

    pub fn to_original(&self) -> String {
        decode(&self.0).expect("SanitizedName should be correctly encoded")
    }
}

impl AsRef<str> for SanitizedName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<OsStr> for SanitizedName {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

impl AsRef<Path> for SanitizedName {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl Display for SanitizedName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use moss_testutils::fs_specific::FILENAME_SPECIAL_CHARS;
    use moss_testutils::random_name::random_string;

    #[test]
    fn test_sanitized_name_normal_name() {
        let normal = random_string(10);
        let sanitized = SanitizedName::new(&normal);
        assert_eq!(sanitized.to_string(), normal);
        assert_eq!(sanitized.to_original(), normal);
    }

    #[test]
    fn test_sanitized_name_special_chars() {
        for char in FILENAME_SPECIAL_CHARS {
            let normal = format!("special{char}name");
            let sanitized = SanitizedName::new(&normal);
            assert_ne!(sanitized.to_string(), normal);
            assert_eq!(sanitized.to_original(), normal);
        }
    }
}
