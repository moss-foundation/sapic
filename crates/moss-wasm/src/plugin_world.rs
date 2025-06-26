/// See moss-wasm/wit
wasmtime::component::bindgen!("plugin-world");

use anyhow::{Result, anyhow};

use crate::plugin_world::plugin::base::types::SimpleValue;

impl From<SimpleValue> for Value {
    fn from(value: SimpleValue) -> Self {
        match value {
            SimpleValue::Null => Value::Null,
            SimpleValue::Boolean(b) => Value::Boolean(b),
            SimpleValue::Num(number) => Value::Num(number),
            SimpleValue::Str(s) => Value::Str(s),
        }
    }
}

impl TryFrom<Value> for SimpleValue {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(SimpleValue::Null),
            Value::Boolean(b) => Ok(SimpleValue::Boolean(b)),
            Value::Num(number) => Ok(SimpleValue::Num(number)),
            Value::Str(s) => Ok(SimpleValue::Str(s)),
            Value::Arr(_) => Err(anyhow!("Cannot convert composite type to simple value")),
            Value::Obj(_) => Err(anyhow!("Cannot convert composite type to simple value")),
        }
    }
}
