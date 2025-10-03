use hcl::{HeredocStripMode, Identifier};
use serde::{Serialize, Serializer};

const INDENT: &'static str = "  ";
const DELIMITER: &'static str = "EOF";

/// We must manually indent the content to the correct level during serialization
fn indent(content: &str) -> String {
    content
        .lines()
        .map(|l| format!("{INDENT}{l}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Serialize String as an HCL heredoc String
/// Heredoc is the only way to store multiline string in hcl
/// Example:
/// block {
///   value = <<-EOT
///   hello
///     world
///   EOT
/// }
pub fn serialize_string_as_heredoc<S>(expr: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let indented = indent(expr);
    let heredoc = hcl::expr::Heredoc::new(Identifier::from(DELIMITER), indented)
        .with_strip_mode(HeredocStripMode::Indent);
    heredoc.serialize(serializer)
}

/// Custom serializer for Option<String> that uses heredoc when Some
pub fn serialize_option_string_as_heredoc<S>(
    option: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match option {
        Some(value) => serialize_string_as_heredoc(value, serializer),
        None => serializer.serialize_none(),
    }
}
