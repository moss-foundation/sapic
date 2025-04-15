use std::cell::LazyCell;
use anyhow::Result;
use std::path::{Component, Path, PathBuf};
use std::sync::LazyLock;
use regex::Regex;

static FORBIDDEN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"[.%<>:"/\\|?*]"#).unwrap()
});

// Function to encode forbidden characters and '%' in a directory name
pub fn encode_directory_name(name: &str) -> String {
    // List of forbidden characters, including '%' to avoid ambiguity
    FORBIDDEN_RE.replace_all(name, |caps: &regex::Captures| {
        // Replace each forbidden character with its hex representation (e.g., ':' -> %3A)
        format!("%{:02X}", caps[0].chars().next().unwrap() as u32)
    }).to_string()
}

// Function to decode an encoded directory name back to its original form
pub fn decode_directory_name(encoded: &str) -> Result<String, std::num::ParseIntError> {
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



// FIXME: This process may need some refinement
/// Doing a basic normalization using Path::components() and encode the segments after the prefix
pub fn encode_path(path: &Path, prefix: Option<&Path>) -> Result<PathBuf> {
    // Determine the relative part of the path to be encoded.
    let relative_path = match prefix {
        Some(prefix) => path.strip_prefix(prefix)?,
        None => path,
    };

    // Encode only the normal components of the path.
    let encoded: PathBuf = relative_path
        .components()
        .filter_map(|comp| {
            if let Component::Normal(name) = comp {
                Some(encode_directory_name(&name.to_string_lossy()))
            } else {
                // Special components are ignored (ParentDir, Prefix, RootDir, CurDir)
                None
            }
        })
        .collect();

    // If a prefix was provided, join it back with the encoded path.
    Ok(prefix.map(|p| p.join(&encoded)).unwrap_or(encoded))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_directory_name() {
        let invalid_names = vec![
            "workspace.name",  // Contains dot
            "workspace/name",  // Contains path separator
            "workspace\\name", // Contains backslash
            "workspace:name",  // Contains colon
            "workspace*name",  // Contains wildcard
            "workspace?name",  // Contains question mark
            "workspace\"name", // Contains quotes
            "workspace<name",  // Contains angle brackets
            "workspace>name",  // Contains angle brackets
            "workspace|name",  // Contains pipe
        ];
        invalid_names.into_iter().for_each(|name| {
            dbg!(encode_directory_name(name));
        })
    }

    #[test]
    fn test_special_chars() {
        let path = PathBuf::from("pre.fix/colle*ction");
        dbg!(&encode_path(&path, Some(Path::new("pre.fix"))));
    }


}