use anyhow::{Result, anyhow};
use redb::{Key, TypeName, Value};
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use std::cmp::Ordering;

/// Macro for easily creating `AnyValueEnum` for primitive types
/// For complex types, use `AnyValueEnum::serialize()`
#[macro_export]
macro_rules! any_value {
    (null) => {
        $crate::anyvalue_enum::AnyValueEnum::Null
    };
    ($value:expr) => {
        $crate::anyvalue_enum::AnyValueEnum::from($value)
    };
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum AnyValueEnum {
    Null,
    Bool(bool),
    Int(i64),
    Uint(u64),
    #[serde(deserialize_with = "deserialize_f64_null_as_nan")]
    Double(f64),
    Text(String),
    Bytes(Vec<u8>),
}

/// A helper to deserialize `f64`, treating JSON null as f64::NAN.
/// See https://github.com/serde-rs/json/issues/202
fn deserialize_f64_null_as_nan<'de, D: Deserializer<'de>>(des: D) -> Result<f64, D::Error> {
    let optional = Option::<f64>::deserialize(des)?;
    Ok(optional.unwrap_or(f64::NAN))
}

impl AnyValueEnum {
    pub fn type_name(&self) -> &'static str {
        match self {
            AnyValueEnum::Null => "Null",
            AnyValueEnum::Bool(_) => "Bool",
            AnyValueEnum::Int(_) => "Int",
            AnyValueEnum::Uint(_) => "UnsizedInt",
            AnyValueEnum::Double(_) => "Double",
            AnyValueEnum::Text(_) => "Text",
            AnyValueEnum::Bytes(_) => "Bytes",
        }
    }
    pub fn is_null(&self) -> bool {
        matches!(self, AnyValueEnum::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            AnyValueEnum::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            AnyValueEnum::Int(i) => Some(*i),
            AnyValueEnum::Uint(u) => Some(*u as i64),
            _ => None,
        }
    }

    pub fn as_uint(&self) -> Option<u64> {
        match self {
            AnyValueEnum::Uint(u) => Some(*u),
            AnyValueEnum::Int(i) => Some(*i as u64),
            _ => None,
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        match self {
            AnyValueEnum::Double(d) => Some(*d),
            AnyValueEnum::Int(i) => Some(*i as f64),
            AnyValueEnum::Uint(u) => Some(*u as f64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            AnyValueEnum::Text(t) => Some(t.as_str()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            AnyValueEnum::Bytes(b) => Some(b.as_slice()),
            _ => None,
        }
    }

    pub fn serialize<T: Serialize>(value: &T) -> Result<Self> {
        serde_json::to_vec(value)
            .map(|bytes| Self::Bytes(bytes))
            .map_err(|e| anyhow!("Serialization failed: {}", e))
    }

    pub fn deserialize<T: DeserializeOwned>(&self) -> Result<T> {
        match self {
            AnyValueEnum::Bytes(bytes) => serde_json::from_slice(bytes).map_err(|e| e.into()),
            _ => Err(anyhow!(
                "Cannot deserialize non-bytes value: expected Bytes variant, got {}",
                self.type_name()
            )),
        }
    }
}

impl Value for AnyValueEnum {
    type SelfType<'a>
        = AnyValueEnum
    where
        Self: 'a;
    type AsBytes<'a>
        = Vec<u8>
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        serde_json::from_slice(data).unwrap_or_else(|_| {
            // JSON deserialization fails, store as raw bytes
            AnyValueEnum::Bytes(data.to_vec())
        })
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        serde_json::to_vec(value).unwrap_or_else(|_| {
            // Fall back to empty vec when serialization fails
            vec![]
        })
    }

    fn type_name() -> TypeName {
        TypeName::new("AnyValueEnum")
    }
}

impl From<bool> for AnyValueEnum {
    fn from(b: bool) -> AnyValueEnum {
        AnyValueEnum::Bool(b)
    }
}
impl From<i32> for AnyValueEnum {
    fn from(i: i32) -> AnyValueEnum {
        AnyValueEnum::Int(i as i64)
    }
}
impl From<i64> for AnyValueEnum {
    fn from(i: i64) -> AnyValueEnum {
        AnyValueEnum::Int(i)
    }
}
impl From<isize> for AnyValueEnum {
    fn from(i: isize) -> Self {
        AnyValueEnum::Int(i as i64)
    }
}

impl From<u32> for AnyValueEnum {
    fn from(u: u32) -> AnyValueEnum {
        AnyValueEnum::Uint(u as u64)
    }
}
impl From<u64> for AnyValueEnum {
    fn from(u: u64) -> AnyValueEnum {
        AnyValueEnum::Uint(u)
    }
}
impl From<usize> for AnyValueEnum {
    fn from(u: usize) -> AnyValueEnum {
        AnyValueEnum::Uint(u as u64)
    }
}
impl From<f32> for AnyValueEnum {
    fn from(f: f32) -> AnyValueEnum {
        AnyValueEnum::Double(f as f64)
    }
}
impl From<f64> for AnyValueEnum {
    fn from(f: f64) -> AnyValueEnum {
        AnyValueEnum::Double(f)
    }
}

impl From<String> for AnyValueEnum {
    fn from(s: String) -> AnyValueEnum {
        AnyValueEnum::Text(s)
    }
}

impl From<&str> for AnyValueEnum {
    fn from(value: &str) -> Self {
        AnyValueEnum::Text(value.to_string())
    }
}

impl From<Vec<u8>> for AnyValueEnum {
    fn from(v: Vec<u8>) -> AnyValueEnum {
        AnyValueEnum::Bytes(v)
    }
}

impl<const N: usize> From<&[u8; N]> for AnyValueEnum {
    fn from(v: &[u8; N]) -> AnyValueEnum {
        AnyValueEnum::Bytes(v.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DatabaseClient, ReDbClient, bincode_table::BincodeTable};
    use std::path::PathBuf;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        pub name: String,
        pub id: i32,
    }
    #[test]
    fn test_anyvalue_enum_basic() {
        let db_path = PathBuf::from("tests").join("anyvalue_enum.db");
        let table: BincodeTable<i32, AnyValueEnum> = BincodeTable::new("any_value");
        let client = ReDbClient::new(&db_path)
            .unwrap()
            .with_table(&table)
            .unwrap();
        let test_data = TestData {
            name: "secret".to_string(),
            id: 42,
        };

        {
            // Test writing basic types
            let mut write_txn = client.begin_write().unwrap();
            table
                .insert(&mut write_txn, 0, &AnyValueEnum::Null)
                .unwrap();
            table
                .insert(&mut write_txn, 1, &AnyValueEnum::Bool(true))
                .unwrap();
            table
                .insert(&mut write_txn, 2, &AnyValueEnum::Int(-42))
                .unwrap();
            table
                .insert(&mut write_txn, 3, &AnyValueEnum::Uint(42))
                .unwrap();
            table
                .insert(&mut write_txn, 4, &AnyValueEnum::Double(3.14))
                .unwrap();
            // Test writing NAN
            table
                .insert(&mut write_txn, 5, &AnyValueEnum::Double(f64::NAN))
                .unwrap();
            table
                .insert(
                    &mut write_txn,
                    6,
                    &AnyValueEnum::Text(String::from("hello world")),
                )
                .unwrap();
            write_txn.commit().unwrap();
        }

        {
            // Test reading basic types
            let read_txn = client.begin_read().unwrap();
            let result0 = table.read(&read_txn, 0).unwrap();
            assert_eq!(result0, AnyValueEnum::Null);
            let result1 = table.read(&read_txn, 1).unwrap();
            assert_eq!(result1, AnyValueEnum::Bool(true));
            let result2 = table.read(&read_txn, 2).unwrap();
            assert_eq!(result2, AnyValueEnum::Int(-42));
            let result3 = table.read(&read_txn, 3).unwrap();
            assert_eq!(result3, AnyValueEnum::Uint(42));
            let result4 = table.read(&read_txn, 4).unwrap();
            // We can't directly compare two floats
            let float = result4.as_double().unwrap();
            assert!((float - 3.14).abs() < f64::EPSILON);
            // Check reading NAN from the database
            let result5 = table.read(&read_txn, 5).unwrap();
            let nan = result5.as_double().unwrap();
            assert!(nan.is_nan());
            let result6 = table.read(&read_txn, 6).unwrap();
            assert_eq!(result6, AnyValueEnum::Text("hello world".to_string()));
        }

        {
            // Test writing custom types
            let mut write_txn = client.begin_write().unwrap();
            table
                .insert(
                    &mut write_txn,
                    42,
                    &AnyValueEnum::serialize(&test_data).unwrap(),
                )
                .unwrap();
            write_txn.commit().unwrap();
        }

        {
            // Test reading custom types
            let read_txn = client.begin_read().unwrap();
            let result = table.read(&read_txn, 42).unwrap();
            let data_retrieved = result.deserialize::<TestData>().unwrap();
            assert_eq!(test_data, data_retrieved);
        }

        std::fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_anyvalue_enum_macro() {
        assert_eq!(any_value!(null), AnyValueEnum::Null);
        assert_eq!(any_value!(true), AnyValueEnum::Bool(true));
        assert_eq!(any_value!(1i32), AnyValueEnum::Int(1));
        assert_eq!(any_value!(1i64), AnyValueEnum::Int(1));
        assert_eq!(any_value!(1isize), AnyValueEnum::Int(1));
        assert_eq!(any_value!(1u32), AnyValueEnum::Uint(1));
        assert_eq!(any_value!(1u64), AnyValueEnum::Uint(1));
        assert_eq!(any_value!(1usize), AnyValueEnum::Uint(1));
        assert_eq!(any_value!(1.0f32), AnyValueEnum::Double(1.0));
        assert_eq!(any_value!(1.0f64), AnyValueEnum::Double(1.0));
        assert_eq!(
            any_value!("hello world"),
            AnyValueEnum::Text(String::from("hello world"))
        );
        assert_eq!(
            any_value!("hello world".to_string()),
            AnyValueEnum::Text(String::from("hello world"))
        );
        assert_eq!(
            any_value!(b"hello world"),
            AnyValueEnum::Bytes(b"hello world".to_vec())
        );
        let test_data = TestData {
            name: "name".to_string(),
            id: 42,
        };
        let bytes = serde_json::to_vec(&test_data).unwrap();
        let any_data = any_value!(bytes.clone());
        // Check that macro works as intended
        assert_eq!(any_data, AnyValueEnum::Bytes(bytes));
        // Check that this is equivalent to AnyValueEnum::serialize()
        assert_eq!(any_data, AnyValueEnum::serialize(&test_data).unwrap());
    }

    #[test]
    fn test_anyvalue_enum_type_conversion() {
        let i = AnyValueEnum::Int(1);
        assert_eq!(i.as_int().unwrap(), 1);
        assert_eq!(i.as_uint().unwrap(), 1);
        assert_eq!(i.as_double().unwrap(), 1.0);

        let u = AnyValueEnum::Uint(1);
        assert_eq!(u.as_int().unwrap(), 1);
        assert_eq!(u.as_uint().unwrap(), 1);
        assert_eq!(u.as_double().unwrap(), 1.0);

        let f = AnyValueEnum::Double(1.0);
        assert_eq!(f.as_double().unwrap(), 1.0);
    }
}
