#[allow(warnings)]
mod bindings;

use bindings::Guest;

use crate::bindings::plugin::base::host_functions::greet;

struct Component;

impl Guest for Component {
    fn execute(input: bindings::Value) -> bindings::Value {
        greet(&input);
        bindings::Value::Str("Success".to_string())
    }
}

bindings::export!(Component with_types_in bindings);
