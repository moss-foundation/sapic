use hcl::{HeredocStripMode, Identifier};
use serde::{Serialize, Serializer, ser::Error};
use serde_json::Value as JsonValue;

const INDENT: &'static str = "  ";
const DELIMITER: &'static str = "EOF";

// TODO: Use the correct indentation

/// Heredoc is the only way to store multiline string in hcl
/// Example:
/// block {
///   value = <<-EOT
///   hello
///     world
///   EOT
/// }
/// We must manually indent the content to the correct level during serialization

fn indent(content: &str) -> String {
    content
        .lines()
        .map(|l| format!("{INDENT}{l}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Serialize String as an HCL heredoc String
pub fn serialize_string_as_heredoc<S>(expr: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let indented = indent(expr);
    let heredoc = hcl::expr::Heredoc::new(Identifier::from(DELIMITER), indented)
        .with_strip_mode(HeredocStripMode::Indent);
    heredoc.serialize(serializer)
}

/// Serialize Json Value as an HCL heredoc string
pub fn serialize_jsonvalue_as_heredoc<S>(expr: &JsonValue, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let json_str = serde_json::to_string_pretty(expr).map_err(S::Error::custom)?;
    let indented = indent(&json_str);
    let heredoc = hcl::expr::Heredoc::new(Identifier::from(DELIMITER), indented)
        .with_strip_mode(HeredocStripMode::Indent);
    heredoc.serialize(serializer)
}
