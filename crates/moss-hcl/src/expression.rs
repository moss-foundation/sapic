//! Custom HCL Expression serialization and deserialization for JSON round-trips.
//!
//! This module provides specialized handling for `hcl::Expression` when working with JSON,
//! ensuring that complex HCL expressions can be properly serialized to JSON and then
//! deserialized back to their original HCL form without data loss.

use hcl::{
    Body, Expression as HclExpression, Identifier,
    expr::{Traversal, TraversalOperator, Variable},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as JsonValue;

/// Serialize an HCL expression using the standard HCL serialization.
///
/// This function delegates to the standard `hcl::Expression::serialize` method,
/// which already handles JSON serialization correctly by converting HCL expressions
/// to JSON template strings (e.g., `"${expression}"`).
pub fn serialize_expression<S>(expr: &HclExpression, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    expr.serialize(serializer)
}

/// Deserialize a JSON value into an HCL expression, handling template strings and various patterns.
///
/// ## Why Custom Deserialization is Needed
///
/// The standard `hcl::Expression::deserialize` method cannot properly handle JSON template strings
/// like `"${variable}"` or `"${function(arg1, arg2)}"` when deserializing from JSON back to HCL.
/// This creates problems for round-trip serialization (HCL → JSON → HCL) because:
///
/// ### Problem Examples:
///
/// **1. Simple Variables:**
/// ```json
/// // JSON representation
/// "value": "${test}"
/// ```
/// - **Standard deserialize**: Treats as plain string `"${test}"`
/// - **Expected result**: HCL variable `test` (without quotes)
/// - **Our solution**: Parse content inside `${}` as variable name
///
/// **2. Function Calls:**
/// ```json
/// // JSON representation  
/// "value": "${try(coalesce(var.ami, data.aws_ssm_parameter.this[0].value), null)}"
/// ```
/// - **Standard deserialize**: Treats as plain string with `${}` wrapper
/// - **Expected result**: HCL function call `try(coalesce(var.ami, data.aws_ssm_parameter.this[0].value), null)`
/// - **Our solution**: Parse inner content as complete HCL expression
///
/// **3. Traversal Expressions:**
/// ```json
/// // JSON representation
/// "value": "${local.create}"
/// ```
/// - **Standard deserialize**: Treats as plain string `"${local.create}"`
/// - **Expected result**: HCL traversal `local.create`
/// - **Our solution**: Manually construct traversal from dot-separated parts
///
/// **4. Conditional Expressions:**
/// ```json
/// // JSON representation
/// "value": "${condition ? value1 : value2}"
/// ```
pub fn deserialize_expression<'de, D>(deserializer: D) -> Result<HclExpression, D::Error>
where
    D: Deserializer<'de>,
{
    let value = JsonValue::deserialize(deserializer)?;
    match value {
        JsonValue::String(s) if looks_like_template(&s) => {
            parse_template_expr(s).map_err(serde::de::Error::custom)
        }
        other => deserialize_hcl_value(other),
    }
}

fn looks_like_template(s: &str) -> bool {
    s.starts_with("${") && s.ends_with('}')
}

/// Parse a template expression string into an HCL expression.
///
/// This function attempts multiple parsing strategies in order of specificity:
/// 1. Simple variable names (alphanumeric + underscore)
/// 2. Full HCL expression parsing via temporary attribute wrapper
/// 3. Manual traversal construction for dot-separated patterns
/// 4. Fallback to preserving original string
fn parse_template_expr(s: String) -> Result<HclExpression, String> {
    let inner = &s[2..s.len() - 1];

    if is_simple_var(inner) {
        if let Ok(var) = Variable::new(inner) {
            return Ok(HclExpression::Variable(var));
        }
    }

    if let Ok(expr) = parse_full_hcl(inner) {
        return Ok(expr);
    }

    if let Some(expr) = parse_traversal(inner) {
        return Ok(expr);
    }

    Ok(HclExpression::String(s))
}

/// Determine if the content is a simple variable name.
///
/// Simple variables contain only alphanumeric characters and underscores,
/// with no dots, parentheses, or other special characters.
fn is_simple_var(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// Parse a full HCL expression by wrapping it in a temporary attribute.
///
/// This technique works by creating a temporary HCL snippet like `test = expression`
/// and then extracting the parsed expression from the attribute. This allows us to
/// leverage the full HCL parser for complex expressions like function calls,
/// conditionals, and arithmetic operations.
fn parse_full_hcl(inner: &str) -> Result<HclExpression, String> {
    let wrapped = format!("_ = {}", inner);
    hcl::from_str::<Body>(&wrapped)
        .map_err(|e| e.to_string())?
        .attributes()
        .next()
        .map(|attr| attr.expr().clone())
        .ok_or_else(|| "Failed to extract expression".into())
}

/// Parse a traversal expression like `var.name` or `local.variable`.
///
/// This handles simple dot-notation access patterns that are common in HCL.
/// Only processes expressions with exactly one dot and no function call parentheses.
fn parse_traversal(inner: &str) -> Option<HclExpression> {
    let parts: Vec<&str> = inner.splitn(2, '.').collect();
    if parts.len() != 2 || inner.contains('(') {
        return None;
    }

    let root = Variable::new(parts[0]).ok()?;
    let key = Identifier::new(parts[1]).ok()?;
    let traversal = Traversal::new(
        HclExpression::Variable(root),
        vec![TraversalOperator::GetAttr(key)],
    );
    Some(HclExpression::Traversal(Box::new(traversal)))
}

/// Deserialize non-template JSON values into HCL expressions.
/// It first attempts to use the standard HCL deserialization, falling back to
/// manual conversion if that fails.
fn deserialize_hcl_value<E>(value: JsonValue) -> Result<HclExpression, E>
where
    E: serde::de::Error,
{
    if let Ok(expr) = HclExpression::deserialize(value.clone()) {
        return Ok(expr);
    }

    match value {
        JsonValue::String(s) => Ok(HclExpression::String(s)),
        JsonValue::Bool(b) => Ok(HclExpression::Bool(b)),
        JsonValue::Number(n) => parse_number(n).map_err(E::custom),
        JsonValue::Null => Ok(HclExpression::Null),
        _ => Ok(HclExpression::Null),
    }
}

fn parse_number(n: serde_json::Number) -> Result<HclExpression, String> {
    if let Some(i) = n.as_i64() {
        Ok(HclExpression::Number(i.into()))
    } else if let Some(f) = n.as_f64() {
        match hcl::Number::from_f64(f) {
            Some(hcl_num) => Ok(HclExpression::Number(hcl_num)),
            None => Err("Invalid floating point number".to_string()),
        }
    } else {
        Err("Unsupported number format".to_string())
    }
}
