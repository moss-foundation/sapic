// use joinerror::OptionExt;
// use moss_text::sanitized::{desanitize, sanitize};
//
// use crate::constants;

// pub(super) fn format_file_name(name: &str) -> String {
//     format!(
//         "{}.{}",
//         sanitize(name),
//         constants::ENVIRONMENT_FILE_EXTENSION
//     )
// }

// pub(super) fn parse_file_name(filename: &str) -> joinerror::Result<String> {
//     let name = filename
//         .split('.')
//         .next()
//         .ok_or_join_err_with::<()>(|| format!("invalid file name: {}", filename))?;
//
//     Ok(desanitize(name))
// }
