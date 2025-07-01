use anyhow::Result;
use derive_more::{Deref, DerefMut};
use moss_text::sanitized;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct SanitizedPath(PathBuf);

impl From<PathBuf> for SanitizedPath {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

impl From<SanitizedPath> for PathBuf {
    fn from(path: SanitizedPath) -> Self {
        path.0
    }
}

/// Doing a basic normalization using Path::components()
/// All path separators will be normalized, and special components ignored
pub fn normalize_path(path: &Path) -> PathBuf {
    // On Windows, backlashes might not be properly parsed as separators
    // So first we convert the path to string and replace backslashes with forward slashes
    let path_str = path.to_string_lossy().replace('\\', "/");
    let path = Path::new(&path_str);

    let components: Vec<_> = path
        .components()
        .filter_map(|comp| {
            match comp {
                Component::Normal(name) => Some(name),
                _ => None, // Filter out special components (ParentDir, Prefix, RootDir, CurDir)
            }
        })
        .collect();

    if components.is_empty() {
        return PathBuf::new();
    }

    let mut result = PathBuf::new();
    for component in components {
        result.push(component);
    }

    result
}

// TODO: return SanitizedPath
/// Normalize the path and encode the segments after the prefix
pub fn sanitize_path(path: &Path, prefix: Option<&Path>) -> Result<PathBuf> {
    // Determine the relative part of the path to be encoded.
    let relative_path = match prefix {
        Some(prefix) => path.strip_prefix(prefix)?,
        None => path,
    };

    let normalized = normalize_path(&relative_path);

    // Encode the parts after the prefix
    let encoded: PathBuf = normalized
        .iter()
        .map(|os_str| sanitized::sanitize(&os_str.to_string_lossy()))
        .collect();

    // If a prefix was provided, join it back with the encoded path.
    Ok(match prefix {
        Some(prefix) => normalize_path(prefix).join(encoded),
        None => encoded,
    })
}

/// Reverse the path sanitization by decoding the segments after the prefix
pub fn desanitize_path(path: &Path, prefix: Option<&Path>) -> Result<PathBuf> {
    // Determine the relative part of the path to be decoded.
    let relative_path = match prefix {
        Some(prefix) => path.strip_prefix(prefix)?,
        None => path,
    };

    // Decode the parts after the prefix
    let decoded: PathBuf = relative_path
        .iter()
        .map(|os_str| sanitized::desanitize(&os_str.to_string_lossy()))
        .collect();

    // If a prefix was provided, join it back with the decoded path.
    Ok(match prefix {
        Some(prefix) => prefix.join(decoded),
        None => decoded,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_chars() {
        let path = PathBuf::from("pre.fix/colle*ction");
        dbg!(&sanitize_path(&path, Some(Path::new("pre.fix"))));
    }

    #[test]
    fn test_normalize_path() {
        let canonical = Path::new("1").join("1");

        let irregular_paths = vec![
            Path::new("1/1"),
            Path::new("1//1"),
            Path::new("1/1/"),
            Path::new("1\\1"),
            Path::new("1\\\\1"),
            Path::new("1\\1\\"),
        ];

        for path in irregular_paths {
            let normalized = normalize_path(path);
            assert_eq!(
                normalized.components().collect::<Vec<_>>(),
                canonical.components().collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn test_desanitize_path_without_prefix() {
        let sanitized_path = Path::new("file%20name%2Etxt");
        let result = desanitize_path(sanitized_path, None).unwrap();
        assert_eq!(result, PathBuf::from("file name.txt"));
    }

    #[test]
    fn test_desanitize_path_with_prefix() {
        let sanitized_path = Path::new("my/prefix/file%20name%2Etxt");
        let prefix = Path::new("my/prefix");
        let result = desanitize_path(sanitized_path, Some(prefix)).unwrap();
        assert_eq!(result, PathBuf::from("my/prefix/file name.txt"));
    }

    #[test]
    fn test_desanitize_path_multiple_segments() {
        let sanitized_path = Path::new("folder%2Ename/file%20with%2Aspaces%2Etxt");
        let result = desanitize_path(sanitized_path, None).unwrap();
        assert_eq!(result, PathBuf::from("folder.name/file with*spaces.txt"));
    }

    #[test]
    fn test_desanitize_path_unicode_preserved() {
        let sanitized_path = Path::new("路径%2F到%2F文件%2Etxt");
        let result = desanitize_path(sanitized_path, None).unwrap();
        assert_eq!(result, PathBuf::from("路径/到/文件.txt"));
    }

    #[test]
    fn test_desanitize_path_invalid_sequences_preserved() {
        let sanitized_path = Path::new("file%ZZname%2Etxt");
        let result = desanitize_path(sanitized_path, None).unwrap();
        assert_eq!(result, PathBuf::from("file%ZZname.txt"));
    }

    #[test]
    fn test_desanitize_path_no_encoded_chars() {
        let clean_path = Path::new("clean/path/without/encoding");
        let result = desanitize_path(clean_path, None).unwrap();
        assert_eq!(result, clean_path);
    }

    #[test]
    fn test_desanitize_path_empty_path() {
        let empty_path = Path::new("");
        let result = desanitize_path(empty_path, None).unwrap();
        assert_eq!(result, PathBuf::from(""));
    }

    #[test]
    fn test_sanitize_desanitize_roundtrip_without_prefix() {
        let original_paths = vec![
            "simple.txt",
            "file with spaces.txt",
            "file*with?special<chars>.txt",
            "folder/with.dots/file.txt",
            "路径/文件.txt",
        ];

        for original in original_paths {
            let path = PathBuf::from(original);
            let sanitized = sanitize_path(&path, None).unwrap();
            let desanitized = desanitize_path(&sanitized, None).unwrap();
            assert_eq!(desanitized, path, "Roundtrip failed for: {}", original);
        }
    }

    #[test]
    fn test_sanitize_desanitize_roundtrip_with_prefix() {
        let prefix = Path::new("my/safe/prefix");
        let test_cases = vec![
            "simple.txt",
            "file with spaces.txt",
            "folder.name/file*name.txt",
        ];

        for test_case in test_cases {
            let original = prefix.join(test_case);
            let sanitized = sanitize_path(&original, Some(prefix)).unwrap();
            let desanitized = desanitize_path(&sanitized, Some(prefix)).unwrap();
            assert_eq!(
                desanitized, original,
                "Roundtrip failed for: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_desanitize_path_consecutive_encoded_chars() {
        let sanitized_path = Path::new("file%2E%2E%2E%2A%2A%2A.txt");
        let result = desanitize_path(sanitized_path, None).unwrap();
        assert_eq!(result, PathBuf::from("file...***.txt"));
    }

    #[test]
    fn test_desanitize_path_prefix_not_found() {
        let path = Path::new("different/path/file.txt");
        let prefix = Path::new("non/existent/prefix");
        let result = desanitize_path(path, Some(prefix));
        assert!(
            result.is_err(),
            "Should return error when prefix is not found in path"
        );
    }
}
