use hcl::Expression as HclExpression;
use serde_json::Value as JsonValue;

/// Convert a JSON value to an HCL expression.
///
/// This function uses our custom deserialization logic to properly handle
/// JSON template strings like `"${variable}"` and convert them to appropriate
/// HCL expressions.
pub fn json_to_hcl(json_value: &JsonValue) -> Result<HclExpression, String> {
    // Use our custom deserialize_expression function with JsonValue as deserializer
    match crate::expression::deserialize_expression(json_value) {
        Ok(expr) => Ok(expr),
        Err(err) => Err(format!(
            "Failed to convert JSON to HCL: {}",
            err.to_string()
        )),
    }
}

/// Convert an HCL expression to a JSON value.
///
/// This function uses our custom serialization logic to properly convert
/// HCL expressions to JSON, including template string format for complex expressions.
pub fn hcl_to_json(hcl_expr: &HclExpression) -> Result<JsonValue, String> {
    // Create a dummy serializer to JSON
    match crate::expression::serialize_expression(&hcl_expr, serde_json::value::Serializer) {
        Ok(json_val) => Ok(json_val),
        Err(err) => Err(format!(
            "Failed to convert HCL to JSON: {}",
            err.to_string()
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hcl::expr::Variable;
    use serde_json::json;

    #[test]
    fn test_json_to_hcl_simple_variable() {
        let json_val = json!("${test}");
        let hcl_expr = json_to_hcl(&json_val).unwrap();

        match hcl_expr {
            HclExpression::Variable(var) => {
                assert_eq!(var.to_string(), "test");
            }
            _ => panic!("Expected Variable expression, got {:?}", hcl_expr),
        }
    }

    #[test]
    fn test_json_to_hcl_function_call() {
        let json_val = json!("${try(coalesce(var.ami, null))}");
        let hcl_expr = json_to_hcl(&json_val).unwrap();

        // Function calls are parsed as FuncCall expressions
        // We can't easily check the internal structure, but we can serialize it back
        let json_result = hcl_to_json(&hcl_expr).unwrap();
        assert!(json_result.as_str().unwrap().contains("try"));
        assert!(json_result.as_str().unwrap().contains("coalesce"));
    }

    #[test]
    fn test_json_to_hcl_traversal() {
        let json_val = json!("${local.create}");
        let hcl_expr = json_to_hcl(&json_val).unwrap();

        match hcl_expr {
            HclExpression::Traversal(_) => {
                // Success - we got a traversal expression
            }
            _ => panic!("Expected Traversal expression, got {:?}", hcl_expr),
        }
    }

    #[test]
    fn test_json_to_hcl_conditional() {
        let json_val = json!("${local.create ? 1 : 0}");
        let hcl_expr = json_to_hcl(&json_val).unwrap();

        // Conditional expressions are complex and parsed by the HCL parser
        let json_result = hcl_to_json(&hcl_expr).unwrap();
        assert!(json_result.as_str().unwrap().contains("?"));
        assert!(json_result.as_str().unwrap().contains(":"));
    }

    #[test]
    fn test_json_to_hcl_primitive_values() {
        // Test null
        let json_null = json!(null);
        let hcl_null = json_to_hcl(&json_null).unwrap();
        assert!(matches!(hcl_null, HclExpression::Null));

        // Test boolean
        let json_bool = json!(true);
        let hcl_bool = json_to_hcl(&json_bool).unwrap();
        assert!(matches!(hcl_bool, HclExpression::Bool(true)));

        // Test number
        let json_num = json!(42);
        let hcl_num = json_to_hcl(&json_num).unwrap();
        match hcl_num {
            HclExpression::Number(n) => {
                assert_eq!(n.as_i64().unwrap(), 42);
            }
            _ => panic!("Expected Number expression"),
        }

        // Test string (not template)
        let json_str = json!("hello world");
        let hcl_str = json_to_hcl(&json_str).unwrap();
        match hcl_str {
            HclExpression::String(s) => {
                assert_eq!(s, "hello world");
            }
            _ => panic!("Expected String expression"),
        }
    }

    #[test]
    fn test_hcl_to_json_variable() {
        let var = HclExpression::Variable(Variable::new("test").unwrap());
        let json_val = hcl_to_json(&var).unwrap();

        assert_eq!(json_val.as_str().unwrap(), "${test}");
    }

    #[test]
    fn test_hcl_to_json_primitive_values() {
        // Test null
        let hcl_null = HclExpression::Null;
        let json_null = hcl_to_json(&hcl_null).unwrap();
        assert!(json_null.is_null());

        // Test boolean
        let hcl_bool = HclExpression::Bool(true);
        let json_bool = hcl_to_json(&hcl_bool).unwrap();
        assert_eq!(json_bool.as_bool().unwrap(), true);

        // Test number
        let hcl_num = HclExpression::Number(42.into());
        let json_num = hcl_to_json(&hcl_num).unwrap();
        assert_eq!(json_num.as_i64().unwrap(), 42);

        // Test string
        let hcl_str = HclExpression::String("hello world".to_string());
        let json_str = hcl_to_json(&hcl_str).unwrap();
        assert_eq!(json_str.as_str().unwrap(), "hello world");
    }

    #[test]
    fn test_round_trip_conversion() {
        let test_cases = vec![
            json!("${test}"),
            json!("${local.create}"),
            json!("${try(coalesce(var.ami, null))}"),
            json!("${condition ? 1 : 0}"),
            json!(null),
            json!(true),
            json!(false),
            json!(42),
            json!(3.14),
            json!("plain string"),
        ];

        for json_input in test_cases {
            let hcl_expr = json_to_hcl(&json_input).unwrap();
            let json_output = hcl_to_json(&hcl_expr).unwrap();

            // For template strings, the round-trip should preserve the content
            if let Some(input_str) = json_input.as_str() {
                if input_str.starts_with("${") {
                    let output_str = json_output.as_str().unwrap();
                    assert!(
                        output_str.starts_with("${"),
                        "Round-trip failed for template: {} -> {}",
                        input_str,
                        output_str
                    );
                }
            } else {
                // For primitive values, they should be identical
                assert_eq!(
                    json_input, json_output,
                    "Round-trip failed for primitive value"
                );
            }
        }
    }
}
