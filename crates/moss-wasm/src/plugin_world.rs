// TODO: Right now there's only limited support for composite types
// Since WIT disallows recursive types, you can only put SimpleValue into composite Value
// This means that you cannot, for example, have an array of objects as a value
// In the future, we might consider using JSON string instead

// // See moss-wasm/wit
// wasmtime::component::bindgen!("plugin-world");

// use anyhow::anyhow;
// use plugin::base::types::{Number, SimpleValue};

// impl From<&str> for Value {
//     fn from(value: &str) -> Self {
//         Self::Str(value.into())
//     }
// }

// impl From<String> for Value {
//     fn from(value: String) -> Self {
//         Self::Str(value)
//     }
// }

// impl From<SimpleValue> for Value {
//     fn from(value: SimpleValue) -> Self {
//         match value {
//             SimpleValue::Null => Value::Null,
//             SimpleValue::Boolean(b) => Value::Boolean(b),
//             SimpleValue::Num(number) => Value::Num(number),
//             SimpleValue::Str(s) => Value::Str(s),
//         }
//     }
// }

// impl TryFrom<Value> for SimpleValue {
//     type Error = anyhow::Error;

//     fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
//         match value {
//             Value::Null => Ok(SimpleValue::Null),
//             Value::Boolean(b) => Ok(SimpleValue::Boolean(b)),
//             Value::Num(number) => Ok(SimpleValue::Num(number)),
//             Value::Str(s) => Ok(SimpleValue::Str(s)),
//             Value::Arr(_) => Err(anyhow!("Cannot convert composite type to simple value")),
//             Value::Obj(_) => Err(anyhow!("Cannot convert composite type to simple value")),
//         }
//     }
// }

// impl TryFrom<hcl::Number> for Number {
//     type Error = anyhow::Error;

//     fn try_from(value: hcl::Number) -> std::result::Result<Self, Self::Error> {
//         if value.is_u64() {
//             Ok(Number::Unsigned(value.as_u64().unwrap()))
//         } else if value.is_i64() {
//             Ok(Number::Signed(value.as_i64().unwrap()))
//         } else if value.is_f64() {
//             Ok(Number::Float(value.as_f64().unwrap()))
//         } else {
//             // This should never be reached
//             Err(anyhow!("Failed to convert hcl number to WASM number"))
//         }
//     }
// }

// impl TryFrom<Number> for hcl::Number {
//     type Error = anyhow::Error;

//     fn try_from(value: Number) -> std::result::Result<Self, Self::Error> {
//         let number = match value {
//             Number::Signed(i) => hcl::Number::from(i),
//             Number::Unsigned(u) => hcl::Number::from(u),
//             Number::Float(f) => hcl::Number::from_f64(f)
//                 .ok_or(anyhow!("Unable to convert float number {f} to hcl number"))?,
//         };
//         Ok(number)
//     }
// }

// impl TryFrom<hcl::Value> for Value {
//     type Error = anyhow::Error;

//     fn try_from(value: hcl::Value) -> std::result::Result<Self, Self::Error> {
//         let val = match value {
//             hcl::Value::Null => Value::Null,
//             hcl::Value::Bool(b) => Value::Boolean(b),
//             hcl::Value::Number(number) => Value::Num(Number::try_from(number)?),
//             hcl::Value::String(s) => Value::Str(s),
//             hcl::Value::Array(values) => {
//                 let mut elements = vec![];
//                 for ele in values {
//                     let wasm_val: Value = ele.try_into()?;
//                     // We don't support nested composite type now
//                     // Element can only be SimpleValue
//                     let wasm_simple_val: SimpleValue = wasm_val
//                         .try_into()
//                         .map_err(|_| anyhow!("Nested composite types are currently unsupported"))?;
//                     elements.push(wasm_simple_val);
//                 }
//                 Value::Arr(elements)
//             }
//             hcl::Value::Object(index_map) => {
//                 let mut entries = vec![];
//                 for (key, value) in index_map {
//                     let wasm_val: Value = value.try_into()?;
//                     // We don't support nested composite type now
//                     // Element can only be SimpleValue
//                     let wasm_simple_val: SimpleValue = wasm_val
//                         .try_into()
//                         .map_err(|_| anyhow!("Nested composite types are currently unsupported"))?;
//                     entries.push((key, wasm_simple_val));
//                 }
//                 Value::Obj(entries)
//             }
//         };
//         Ok(val)
//     }
// }

// impl TryFrom<Value> for hcl::Value {
//     type Error = anyhow::Error;

//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         let value = match value {
//             Value::Null => hcl::Value::Null,
//             Value::Boolean(b) => hcl::Value::Bool(b),
//             Value::Num(number) => hcl::Value::Number(number.try_into()?),
//             Value::Str(s) => hcl::Value::String(s),
//             Value::Arr(simple_values) => {
//                 let mut elements = vec![];
//                 for simple_val in simple_values {
//                     let wasm_val: Value = simple_val.into();
//                     let hcl_val: hcl::Value = wasm_val.try_into()?;
//                     elements.push(hcl_val);
//                 }
//                 hcl::Value::Array(elements)
//             }
//             Value::Obj(items) => {
//                 let mut map: hcl::Map<String, hcl::Value> = hcl::Map::new();
//                 for (key, simple_val) in items {
//                     let wasm_val: Value = simple_val.into();
//                     let hcl_val: hcl::Value = wasm_val.try_into()?;
//                     map.insert(key, hcl_val);
//                 }
//                 hcl::Value::Object(map)
//             }
//         };
//         Ok(value)
//     }
// }

// impl Value {
//     pub fn as_bool(&self) -> Option<bool> {
//         match self {
//             Value::Boolean(b) => Some(*b),
//             _ => None,
//         }
//     }

//     pub fn as_str(&self) -> Option<&str> {
//         match self {
//             Value::Str(s) => Some(s),
//             _ => None,
//         }
//     }
// }
