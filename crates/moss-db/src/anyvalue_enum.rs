use anyhow::{Result, anyhow};
use redb::{Key, TypeName, Value};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::cmp::Ordering;

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
    Double(f64),
    Text(String),
    Bytes(Vec<u8>),
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

    pub fn try_as_number<T>(&self) -> Option<T>
    where
        T: TryFrom<i64> + TryFrom<u64> + TryFrom<f64>,
    {
        match self {
            AnyValueEnum::Int(i) => TryFrom::try_from(*i).ok(),
            AnyValueEnum::Uint(u) => TryFrom::try_from(*u).ok(),
            AnyValueEnum::Double(d) => TryFrom::try_from(*d).ok(),
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

impl Key for AnyValueEnum {
    fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
        std::cmp::Ord::cmp(data1, data2)
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

impl From<&[u8]> for AnyValueEnum {
    fn from(v: &[u8]) -> AnyValueEnum {
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
    fn test_anyvalue_enum() -> Result<()> {
        let db_path = PathBuf::from("tests").join("anyvalue_enum.db");
        let table: BincodeTable<i32, AnyValueEnum> = BincodeTable::new("any_value");
        let client = ReDbClient::new(&db_path)?.with_table(&table)?;
        let test_data = TestData {
            name: "secret".to_string(),
            id: 42,
        };

        {
            // Test writing basic types
            let mut write_txn = client.begin_write()?;
            table.insert(&mut write_txn, 0, &AnyValueEnum::Null)?;
            table.insert(&mut write_txn, 1, &AnyValueEnum::Bool(true))?;
            table.insert(&mut write_txn, 2, &AnyValueEnum::Int(-42))?;
            table.insert(&mut write_txn, 3, &AnyValueEnum::Uint(42))?;
            table.insert(&mut write_txn, 4, &AnyValueEnum::Double(3.14))?;
            table.insert(
                &mut write_txn,
                5,
                &AnyValueEnum::Text(String::from("hello world")),
            )?;
            write_txn.commit()?;
        }

        {
            // Test reading basic types
            let read_txn = client.begin_read()?;
            let result0 = table.read(&read_txn, 0)?;
            assert_eq!(result0, AnyValueEnum::Null);
            let result1 = table.read(&read_txn, 1)?;
            assert_eq!(result1, AnyValueEnum::Bool(true));
            let result2 = table.read(&read_txn, 2)?;
            assert_eq!(result2, AnyValueEnum::Int(-42));
            let result3 = table.read(&read_txn, 3)?;
            assert_eq!(result3, AnyValueEnum::Uint(42));
            let result4 = table.read(&read_txn, 4)?;
            // We can't directly compare two floats
            let float = result4.as_double().unwrap();
            assert!((float - 3.14).abs() < f64::EPSILON);
            let result5 = table.read(&read_txn, 5)?;
            assert_eq!(result5, AnyValueEnum::Text("hello world".to_string()));
        }

        {
            // Test writing custom types
            let mut write_txn = client.begin_write()?;
            table.insert(&mut write_txn, 42, &AnyValueEnum::serialize(&test_data)?)?;
            write_txn.commit()?;
        }

        {
            // Test reading custom types
            let read_txn = client.begin_read()?;
            let result = table.read(&read_txn, 42)?;
            let data_retrieved = result.deserialize::<TestData>()?;
            assert_eq!(test_data, data_retrieved);
        }

        std::fs::remove_file(db_path)?;

        Ok(())
    }
}
