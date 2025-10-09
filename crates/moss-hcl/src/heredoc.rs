use hcl::{Heredoc, HeredocStripMode, Identifier};
use serde::{Serialize, Serializer};

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

pub fn convert_string_to_heredoc(expr: &str) -> Heredoc {
    let indented = indent(expr);
    Heredoc::new(Identifier::from(DELIMITER), indented).with_strip_mode(HeredocStripMode::Indent)
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
    let heredoc = convert_string_to_heredoc(expr);
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

// Properly handling patching of heredoc strings
// We need to convert them into json value

pub trait ToJsonValue {
    fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error>;
}

impl ToJsonValue for Heredoc {
    fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
        let mut buffer = Vec::new();
        self.serialize(&mut serde_json::Serializer::new(&mut buffer))?;
        serde_json::from_slice(buffer.as_slice())
    }
}
