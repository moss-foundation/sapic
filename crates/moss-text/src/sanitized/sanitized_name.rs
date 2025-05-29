use std::{
    ffi::OsStr,
    fmt::{Display, Formatter},
    ops::Deref,
    path::Path,
};

use super::sanitize;

/// A filename (or directory name) that’s been percent-escaped for safety,
/// but still remembers its original form.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SanitizedName {
    sanitized: String,
    original: String,
}

impl Deref for SanitizedName {
    type Target = str;

    fn deref(&self) -> &str {
        &self.sanitized
    }
}

impl SanitizedName {
    pub fn new<S>(original: S) -> Self
    where
        S: Into<String>,
    {
        let original: String = original.into();
        let sanitized = sanitize(&original);

        Self {
            sanitized,
            original,
        }
    }

    /// Borrow the original, un-escaped name.
    pub fn original(&self) -> &str {
        &self.original
    }

    /// Consume this and return the original `String`.
    pub fn into_original(self) -> String {
        self.original
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

impl<S> From<S> for SanitizedName
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        SanitizedName::new(s)
    }
}

#[cfg(test)]
mod tests {
    use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_string};
    use std::{ffi::OsStr, path::Path};

    use crate::sanitized::desanitize;

    use super::*;

    #[test]
    fn sanitized_name_normal_name() {
        let normal = random_string(10);
        let sanitized = SanitizedName::new(&normal);
        assert_eq!(sanitized.to_string(), normal);
        assert_eq!(sanitized.original(), normal);
    }

    #[test]
    fn sanitized_name_special_chars() {
        for char in FILENAME_SPECIAL_CHARS {
            let normal = format!("special{char}name");
            let sanitized = SanitizedName::new(&normal);
            assert_ne!(sanitized.to_string(), normal);
            assert_eq!(sanitized.original(), normal);
        }
    }

    #[test]
    fn new_from_str_and_string() {
        let s_str = SanitizedName::new("hello/world.txt");
        let s_string = SanitizedName::new("hello/world.txt".to_string());
        assert_eq!(s_str.sanitized, s_string.sanitized);
        assert_eq!(s_str.original(), s_string.original());
    }

    #[test]
    fn original_and_into_original() {
        let orig = "my:file?.txt";
        let san = SanitizedName::new(orig);
        assert_eq!(san.original(), orig);
        let recovered: String = san.clone().into_original();
        assert_eq!(recovered, orig);
    }

    #[test]
    fn as_ref_osstr_and_path() {
        let san = SanitizedName::new("foo/bar");
        let os: &OsStr = san.as_ref();
        assert_eq!(os.to_str().unwrap(), "foo%2Fbar");
        let p: &Path = san.as_ref();
        assert_eq!(p, Path::new("foo%2Fbar"));
    }

    #[test]
    fn display_shows_sanitized() {
        let san = SanitizedName::new("01?02");
        let s = format!("{}", san);
        assert_eq!(s, "01%3F02");
    }

    #[test]
    fn from_trait() {
        let san1: SanitizedName = "a*b".into();
        let san2 = SanitizedName::new("a*b");
        assert_eq!(san1, san2);
    }

    #[test]
    fn roundtrip_sanitize_desanitize_via_sanitizedname() {
        let names = ["simple", "mix/and?match*.", "unicode-路径"];
        for &orig in &names {
            let san = SanitizedName::new(orig);
            let back = desanitize(san.as_ref());
            assert_eq!(back, orig);
        }
    }
}
