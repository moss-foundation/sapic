#[allow(warnings)]
mod bindings;

use crate::bindings::Value;
use bindings::Guest;

struct Component;

impl Guest for Component {
    fn execute(input: Value) -> Value {
        input
    }
}

bindings::export!(Component with_types_in bindings);
