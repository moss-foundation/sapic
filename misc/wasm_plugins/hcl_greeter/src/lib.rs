#[allow(warnings)]
mod bindings;

use bindings::{Guest, Value};

struct Component;

impl Guest for Component {
    fn execute(input: Value) -> Value {
        match input {
            Value::Str(s) => Value::Str(format!("Hello, {s}!")),
            _ => Value::Str("We only support string values".to_string()),
        }
    }
}

bindings::export!(Component with_types_in bindings);
