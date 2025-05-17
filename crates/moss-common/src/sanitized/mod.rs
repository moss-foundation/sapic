pub mod sanitized_name;

/// Set of characters that must be percent-encoded.
const FORBIDDEN: [char; 11] = ['.', '%', '<', '>', ':', '"', '/', '\\', '|', '?', '*'];

/// Lookup table for converting a half-byte (0–15) to its hexadecimal digit.
const HEX_DIGITS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

#[inline]
fn is_forbidden(ch: char) -> bool {
    FORBIDDEN.contains(&ch)
}

/// Converts a single hex digit (0–9, A–F, a–f) to its numeric value 0–15.
/// Returns `None` if `c` is not a hex digit.
#[inline]
fn hex_value(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some((c as u8) - b'0'),
        'A'..='F' => Some((c as u8) - b'A' + 10),
        'a'..='f' => Some((c as u8) - b'a' + 10),
        _ => None,
    }
}

/// Percent-encodes every forbidden character in `input` as `%XX` (where XX is its ASCII code in hex),
/// and leaves all other (including any Unicode) characters unchanged.
pub fn sanitize(input: &str) -> String {
    // Count how many characters will be escaped, to reserve capacity: each becomes 3 chars ("%XY")
    let escape_count = input.chars().filter(|&ch| is_forbidden(ch)).count();
    let mut result = String::with_capacity(input.len() + escape_count * 2);

    for ch in input.chars() {
        if is_forbidden(ch) {
            let code = ch as u32;
            result.push('%');
            // High hex digit
            result.push(HEX_DIGITS[(code >> 4) as usize]);
            // Low hex digit
            result.push(HEX_DIGITS[(code & 0xF) as usize]);
        } else {
            // Leave all other characters (including non-ASCII) intact
            result.push(ch);
        }
    }

    result
}

/// Decodes `%XX` sequences back into their single-byte characters. If a `%`
/// is not followed by two valid hex digits, it is left in place.
///
/// Leaves any other characters (including Unicode) unchanged.
pub fn desanitize(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            let mut lookahead = chars.clone();

            // Peek ahead to see if there are two hex digits
            if let (Some(hi), Some(lo)) = (lookahead.next(), lookahead.next()) {
                if let (Some(h), Some(l)) = (hex_value(hi), hex_value(lo)) {
                    // Consume those two chars from the *original* iterator
                    chars.next();
                    chars.next();

                    // Push the decoded byte as a char
                    result.push((h << 4 | l) as char);
                    continue;
                }
            }

            // Not a valid escape, leave `%` as-is
            result.push('%');
        } else {
            result.push(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_leaves_clean_strings_unchanged() {
        let cases = [
            "",
            "hello_world",
            "Русский текст",
            "こんにちは",
            "1234567890-_=+[]{};'", // punctuation that’s not forbidden
        ];
        for &input in &cases {
            assert_eq!(sanitize(input), input);
        }
    }

    #[test]
    fn sanitize_mixed_string() {
        let input = "file name.txt%backup";
        let out = sanitize(input);
        // spaces and '-' are untouched; '.' → %2E, '%' → %25
        assert_eq!(out, "file name%2Etxt%25backup");
    }

    #[test]
    fn sanitize_preserves_unicode() {
        let input = "路径/到/文件.txt";
        let out = sanitize(input);
        // '/' and '.' get encoded, but Chinese stays intact
        assert_eq!(out, "路径%2F到%2F文件%2Etxt");
    }

    #[test]
    fn desanitize_decodes_valid_sequences() {
        let input = "%41%42%43"; // "A", "B", "C"
        assert_eq!(desanitize(input), "ABC");
    }

    #[test]
    fn desanitize_leaves_invalid_sequences_intact() {
        let cases = [
            ("%", "%"),           // lone percent
            ("%A", "%A"),         // only one hex digit
            ("%AG", "%AG"),       // 'G' not hex
            ("%1G", "%1G"),       // 'G' not hex
            ("%ZZfoo", "%ZZfoo"), // none hex
        ];
        for &(input, expected) in &cases {
            assert_eq!(desanitize(input), expected);
        }
    }

    #[test]
    fn desanitize_mixed_string() {
        let input = "foo%20bar%21baz%ZZ"; // %20=" ", %21="!", %ZZ invalid
        let out = desanitize(input);
        assert_eq!(out, "foo bar!baz%ZZ");
    }

    #[test]
    fn desanitize_preserves_unicode_and_other_chars() {
        let input = "路径%2F到%2F文件%2Etxt";
        assert_eq!(desanitize(input), "路径/到/文件.txt");
    }

    #[test]
    fn roundtrip_identity_for_safe_strings() {
        let cases = ["simple-name", "混合Mixed文字123", "no_forbidden_here!"];
        for &orig in &cases {
            let san = sanitize(orig);
            let desan = desanitize(&san);
            assert_eq!(desan, orig);
        }
    }

    #[test]
    fn roundtrip_with_invalid_percent_sequences() {
        let orig = "%G1%BAD%";
        // sanitize will escape '%' → %25, so input becomes "%25G1%25BAD%25"
        let san = sanitize(orig);
        // desanitize then decodes only valid %25 back to '%'
        let desan = desanitize(&san);
        assert_eq!(desan, orig);
    }

    #[test]
    fn consecutive_forbidden_chars() {
        let input = "...***";
        let out = sanitize(input);
        assert_eq!(out, "%2E%2E%2E%2A%2A%2A");
        assert_eq!(desanitize(&out), input);
    }
}
