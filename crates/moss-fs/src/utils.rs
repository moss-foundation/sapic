use anyhow::Result;
use moss_text::sanitized;
use std::path::{Component, Path, PathBuf};

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
    Ok(prefix.map(|p| normalize_path(p)).unwrap_or(encoded))
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
}
