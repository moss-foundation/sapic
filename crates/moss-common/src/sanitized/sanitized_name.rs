use std::{
    ffi::OsStr,
    fmt::{Display, Formatter},
    path::Path,
};

use super::sanitize;

pub struct SanitizedName {
    sanitized: String,
    original: String,
}

impl SanitizedName {
    pub fn new(original: &str) -> Self {
        Self {
            sanitized: sanitize(&original),
            original: original.to_string(),
        }
    }

    pub fn to_original(&self) -> &str {
        &self.original
    }
}

impl AsRef<str> for SanitizedName {
    fn as_ref(&self) -> &str {
        &self.sanitized
    }
}

impl AsRef<OsStr> for SanitizedName {
    fn as_ref(&self) -> &OsStr {
        self.sanitized.as_ref()
    }
}

impl AsRef<Path> for SanitizedName {
    fn as_ref(&self) -> &Path {
        self.sanitized.as_ref()
    }
}

impl Display for SanitizedName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sanitized)
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
