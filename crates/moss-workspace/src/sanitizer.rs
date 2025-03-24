// TODO: Should we move this into a distinct crate
// since it's used by both moss-collection and moss-workspace?
use regex::Regex;

// Function to encode forbidden characters and '%' in a directory name
pub fn encode_directory_name(name: &str) -> String {
    // List of forbidden characters, including '%' to avoid ambiguity
    let re = Regex::new(r#"[.%<>:"/\\|?*]"#).unwrap();
    re.replace_all(name, |caps: &regex::Captures| {
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
}