use moss_text::sanitized::{desanitize, sanitize};

use crate::constants;

pub(super) fn format_file_name(name: &str) -> String {
    format!(
        "{}.{}",
        sanitize(name),
        constants::ENVIRONMENT_FILE_EXTENSION
    )
}

pub(super) fn parse_file_name(filename: &str) -> Result<String, String> {
    let name = filename
        .split('.')
        .next()
        .ok_or_else(|| format!("invalid file name: {}", filename))?;

    Ok(desanitize(name))
}
