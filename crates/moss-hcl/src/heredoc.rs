use hcl::{HeredocStripMode, Identifier};
use serde::{Serialize, Serializer};
use serde_json::Value as JsonValue;

const INDENT: &'static str = "  ";
const DELIMITER: &'static str = "EOT";

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

pub fn serialize_jsonvalue_as_heredoc<S>(expr: &JsonValue, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let json_text = serde_json::to_string_pretty(expr).map_err(serde::ser::Error::custom)?;
    let indented = indent(&json_text);
    let heredoc = hcl::expr::Heredoc::new(Identifier::from(DELIMITER), indented)
        .with_strip_mode(HeredocStripMode::Indent);
    heredoc.serialize(serializer)
}

/// Custom serializer for Option<JsonValue> that uses heredoc when Some
pub fn serialize_option_jsonvalue_as_heredoc<S>(
    option: &Option<JsonValue>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match option {
        Some(value) => serialize_jsonvalue_as_heredoc(value, serializer),
        None => serializer.serialize_none(),
    }
}
