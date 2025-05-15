use regex::Regex;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::sync::LazyLock;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use ts_rs::TS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[serde(transparent)]
#[ts(export, export_to = "primitives.ts")]
pub struct Identifier(usize);

impl Identifier {
    pub fn new(counter: &AtomicUsize) -> Self {
        Self(counter.fetch_add(1, SeqCst))
    }

    pub fn to_usize(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Regex to match forbidden characters in a directory/file name
static FORBIDDEN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"[.%<>:"/\\|?*]"#).unwrap());

pub struct SanitizedName(String);
impl SanitizedName {
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

    pub fn new(raw: &str) -> Self {
        Self(Self::encode(raw))
    }

    pub fn to_original(&self) -> String {
        Self::decode(&self.0).expect("SanitizedName should be correctly encoded")
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

impl Display for SanitizedName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
