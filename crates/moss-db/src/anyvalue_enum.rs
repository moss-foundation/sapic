use anyhow::{Result, anyhow};
use redb::{Key, TypeName, Value};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::cmp::Ordering;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum AnyValueEnum {
    Null,
    Bool(bool),
    Int(i64),
    UnsizedInt(u64),
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
            AnyValueEnum::UnsizedInt(_) => "UnsizedInt",
            AnyValueEnum::Double(_) => "Double",
            AnyValueEnum::Text(_) => "Text",
            AnyValueEnum::Bytes(_) => "Bytes",
        }
    }
    pub fn is_null(&self) -> bool {
        match self {
            AnyValueEnum::Null => true,
            _ => false,
        }
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
            _ => None,
        }
    }

    pub fn as_unsized_int(&self) -> Option<u64> {
        match self {
            AnyValueEnum::UnsizedInt(u) => Some(*u),
            _ => None,
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        match self {
            AnyValueEnum::Double(d) => Some(*d),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            AnyValueEnum::Text(t) => Some(t.as_str()),
            _ => None,
        }
    }

    pub fn serialize<T: Serialize>(value: &T) -> Result<Self> {
        serde_json::to_vec(value)
            .map(|bytes| Self::Bytes(bytes))
            .map_err(|e| e.into())
    }

    pub fn deserialize<T: DeserializeOwned>(&self) -> Result<T> {
        match self {
            AnyValueEnum::Bytes(bytes) => serde_json::from_slice(bytes).map_err(|e| e.into()),
            _ => Err(anyhow!("cannot deserialize non-bytes value")),
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
        // FIXME: Is this always true?
        serde_json::from_slice(data)
            .expect("Should be able to deserialize bytes back to AnyValueEnum")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        // FIXME: Is this always true?
        serde_json::to_vec(value).expect("Should be able to serialize AnyValueEnum into bytes")
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
            table.insert(&mut write_txn, 3, &AnyValueEnum::UnsizedInt(42))?;
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
            assert_eq!(result3, AnyValueEnum::UnsizedInt(42));
            let result4 = table.read(&read_txn, 4)?;
            // We can't directly compare two floats
            let float = result4.as_double().unwrap();
            assert!((float - 3.14).abs() < f64::EPSILON);
            let result5 = table.read(&read_txn, 5)?;
            assert_eq!(result5, AnyValueEnum::Text("hello world".to_string()));
        }
        std::fs::remove_file(db_path)?;

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

        Ok(())
    }
}
