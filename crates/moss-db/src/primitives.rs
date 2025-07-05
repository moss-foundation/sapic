use redb::{Key, TypeName, Value};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{fmt::Debug, hash::Hash};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AnyKey(Vec<u8>);

impl AnyKey {
    pub fn new(key: &str) -> Self {
        Self(key.as_bytes().to_vec())
    }
}

impl From<Vec<u8>> for AnyKey {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl std::borrow::Borrow<[u8]> for AnyKey {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Display for AnyKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl Value for AnyKey {
    type SelfType<'a>
        = AnyKey
    where
        Self: 'a;

    type AsBytes<'a>
        = &'a [u8]
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn type_name() -> TypeName {
        TypeName::new("AnyKey")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
        &value.0
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        AnyKey(data.to_vec())
    }
}

impl Key for AnyKey {
    fn compare(data1: &[u8], data2: &[u8]) -> std::cmp::Ordering {
        data1.cmp(data2)
    }
}

pub trait IsPrimitive {}

impl IsPrimitive for isize {}
impl IsPrimitive for usize {}
impl IsPrimitive for i8 {}
impl IsPrimitive for i16 {}
impl IsPrimitive for i32 {}
impl IsPrimitive for i64 {}
impl IsPrimitive for i128 {}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct AnyValue(Vec<u8>);

// ### Architecture-dependent integer types ###

impl From<isize> for AnyValue {
    fn from(value: isize) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for isize {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let size = std::mem::size_of::<isize>();

        // Ensure we have enough bytes
        if bytes.len() < size {
            // Pad with zeros if we have fewer bytes than expected
            let mut buf = vec![0u8; size];
            buf[..bytes.len()].copy_from_slice(bytes);
            let array: [u8; std::mem::size_of::<isize>()] = buf.try_into().unwrap();
            isize::from_le_bytes(array)
        } else {
            let mut buf = [0; std::mem::size_of::<isize>()];
            buf.copy_from_slice(&bytes[..size]);
            isize::from_le_bytes(buf)
        }
    }
}

impl From<usize> for AnyValue {
    fn from(value: usize) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for usize {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let size = std::mem::size_of::<usize>();

        // Ensure we have enough bytes
        if bytes.len() < size {
            // Pad with zeros if we have fewer bytes than expected
            let mut buf = vec![0u8; size];
            buf[..bytes.len()].copy_from_slice(bytes);
            let array: [u8; std::mem::size_of::<usize>()] = buf.try_into().unwrap();
            usize::from_le_bytes(array)
        } else {
            let mut buf = [0; std::mem::size_of::<usize>()];
            buf.copy_from_slice(&bytes[..size]);
            usize::from_le_bytes(buf)
        }
    }
}

// ### Signed integer types ###

impl From<i8> for AnyValue {
    fn from(value: i8) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for i8 {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let mut buf = [0; 1];
        buf.copy_from_slice(&bytes[..1]);
        i8::from_le_bytes(buf)
    }
}

impl From<i16> for AnyValue {
    fn from(value: i16) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for i16 {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let mut buf = [0; 2];
        buf.copy_from_slice(&bytes[..2]);
        i16::from_le_bytes(buf)
    }
}

impl From<i32> for AnyValue {
    fn from(value: i32) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for i32 {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let mut buf = [0; 4];
        buf.copy_from_slice(&bytes[..4]);
        i32::from_le_bytes(buf)
    }
}

impl From<i64> for AnyValue {
    fn from(value: i64) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for i64 {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let mut buf = [0; 8];
        buf.copy_from_slice(&bytes[..8]);
        i64::from_le_bytes(buf)
    }
}

impl From<i128> for AnyValue {
    fn from(value: i128) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for i128 {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let mut buf = [0; 16];
        buf.copy_from_slice(&bytes[..16]);
        i128::from_le_bytes(buf)
    }
}

// ### Unsigned integer types ###

impl From<u8> for AnyValue {
    fn from(value: u8) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for u8 {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let mut buf = [0; 1];
        buf.copy_from_slice(&bytes[..1]);
        u8::from_le_bytes(buf)
    }
}

impl From<u16> for AnyValue {
    fn from(value: u16) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for u16 {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let mut buf = [0; 2];
        buf.copy_from_slice(&bytes[..2]);
        u16::from_le_bytes(buf)
    }
}

impl From<u32> for AnyValue {
    fn from(value: u32) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for u32 {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let mut buf = [0; 4];
        buf.copy_from_slice(&bytes[..4]);
        u32::from_le_bytes(buf)
    }
}

impl From<u64> for AnyValue {
    fn from(value: u64) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for u64 {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let mut buf = [0; 8];
        buf.copy_from_slice(&bytes[..8]);
        u64::from_le_bytes(buf)
    }
}

impl From<u128> for AnyValue {
    fn from(value: u128) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for u128 {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let mut buf = [0; 16];
        buf.copy_from_slice(&bytes[..16]);
        u128::from_le_bytes(buf)
    }
}

// ### Floating point types ###

impl From<f32> for AnyValue {
    fn from(value: f32) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for f32 {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let mut buf = [0; 4];
        buf.copy_from_slice(&bytes[..4]);
        f32::from_le_bytes(buf)
    }
}

impl From<f64> for AnyValue {
    fn from(value: f64) -> Self {
        AnyValue(value.to_le_bytes().to_vec())
    }
}

impl From<AnyValue> for f64 {
    fn from(value: AnyValue) -> Self {
        let bytes = value.as_bytes();
        let mut buf = [0; 8];
        buf.copy_from_slice(&bytes[..8]);
        f64::from_le_bytes(buf)
    }
}

// ### Boolean type ###

impl From<bool> for AnyValue {
    fn from(value: bool) -> Self {
        AnyValue(vec![if value { 1 } else { 0 }])
    }
}

impl From<AnyValue> for bool {
    fn from(value: AnyValue) -> Self {
        value.as_bytes()[0] != 0
    }
}

// ### String types ###

impl From<String> for AnyValue {
    fn from(value: String) -> Self {
        AnyValue(value.into_bytes())
    }
}

impl From<AnyValue> for String {
    fn from(value: AnyValue) -> Self {
        String::from_utf8_lossy(value.as_bytes()).into_owned()
    }
}

impl From<&str> for AnyValue {
    fn from(value: &str) -> Self {
        AnyValue(value.as_bytes().to_vec())
    }
}

impl AnyValue {
    pub fn new(value: impl Into<Vec<u8>>) -> Self {
        Self(value.into())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn serialize<T: Serialize>(value: &T) -> Result<Self, serde_json::Error> {
        serde_json::to_vec(value).map(AnyValue)
    }

    pub fn deserialize<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.0)
    }
}

impl std::borrow::Borrow<[u8]> for AnyValue {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Display for AnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl Value for AnyValue {
    type SelfType<'a>
        = AnyValue
    where
        Self: 'a;

    type AsBytes<'a>
        = &'a [u8]
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn type_name() -> TypeName {
        TypeName::new("AnyValue")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
        &value.0
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        AnyValue(data.to_vec())
    }
}

impl Key for AnyValue {
    fn compare(data1: &[u8], data2: &[u8]) -> std::cmp::Ordering {
        data1.cmp(data2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ### Architecture-dependent integer types ###

    #[test]
    fn test_isize_conversion() {
        let values = [isize::MIN, -1000, 0, 1000, isize::MAX];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: isize = any_value.into();
            assert_eq!(converted, original);
        }
    }

    #[test]
    fn test_usize_conversion() {
        let values = [usize::MIN, 123, 1000, 50000, usize::MAX];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: usize = any_value.into();
            assert_eq!(converted, original);
        }
    }

    // ### Signed integer types ###

    #[test]
    fn test_i8_conversion() {
        let values = [i8::MIN, -42, 0, 42, i8::MAX];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: i8 = any_value.into();
            assert_eq!(converted, original);
        }
    }

    #[test]
    fn test_i16_conversion() {
        let values = [i16::MIN, -1000, 0, 1000, i16::MAX];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: i16 = any_value.into();
            assert_eq!(converted, original);
        }
    }

    #[test]
    fn test_i32_conversion() {
        let values = [i32::MIN, -100000, 0, 100000, i32::MAX];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: i32 = any_value.into();
            assert_eq!(converted, original);
        }
    }

    #[test]
    fn test_i64_conversion() {
        let values = [i64::MIN, -1000000000, 0, 1000000000, i64::MAX];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: i64 = any_value.into();
            assert_eq!(converted, original);
        }
    }

    #[test]
    fn test_i128_conversion() {
        let values = [
            i128::MIN,
            -1000000000000000000,
            0,
            1000000000000000000,
            i128::MAX,
        ];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: i128 = any_value.into();
            assert_eq!(converted, original);
        }
    }

    // ### Unsigned integer types ###

    #[test]
    fn test_u8_conversion() {
        let values = [u8::MIN, 42, 128, 200, u8::MAX];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: u8 = any_value.into();
            assert_eq!(converted, original);
        }
    }

    #[test]
    fn test_u16_conversion() {
        let values = [u16::MIN, 1000, 32768, 50000, u16::MAX];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: u16 = any_value.into();
            assert_eq!(converted, original);
        }
    }

    #[test]
    fn test_u32_conversion() {
        let values = [u32::MIN, 100000, 2147483648, 3000000000, u32::MAX];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: u32 = any_value.into();
            assert_eq!(converted, original);
        }
    }

    #[test]
    fn test_u64_conversion() {
        let values = [
            u64::MIN,
            1000000000,
            9223372036854775808,
            15000000000000000000,
            u64::MAX,
        ];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: u64 = any_value.into();
            assert_eq!(converted, original);
        }
    }

    #[test]
    fn test_u128_conversion() {
        let values = [
            u128::MIN,
            1000000000000000000,
            u128::MAX / 2,
            u128::MAX - 1,
            u128::MAX,
        ];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: u128 = any_value.into();
            assert_eq!(converted, original);
        }
    }

    // ### Floating point types ###

    #[test]
    fn test_f32_conversion() {
        let values = [f32::MIN, -3.14159, -0.0, 0.0, 3.14159, f32::MAX];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: f32 = any_value.into();
            assert_eq!(converted, original);
        }

        // Test special float values
        let special_values = [f32::INFINITY, f32::NEG_INFINITY, f32::NAN];
        for original in special_values {
            let any_value = AnyValue::from(original);
            let converted: f32 = any_value.into();
            if original.is_nan() {
                assert!(converted.is_nan());
            } else {
                assert_eq!(converted, original);
            }
        }
    }

    #[test]
    fn test_f64_conversion() {
        let values = [
            f64::MIN,
            -3.141592653589793,
            -0.0,
            0.0,
            3.141592653589793,
            f64::MAX,
        ];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: f64 = any_value.into();
            assert_eq!(converted, original);
        }

        // Test special float values
        let special_values = [f64::INFINITY, f64::NEG_INFINITY, f64::NAN];
        for original in special_values {
            let any_value = AnyValue::from(original);
            let converted: f64 = any_value.into();
            if original.is_nan() {
                assert!(converted.is_nan());
            } else {
                assert_eq!(converted, original);
            }
        }
    }

    // ### Boolean type ###

    #[test]
    fn test_bool_conversion() {
        let values = [true, false];
        for original in values {
            let any_value = AnyValue::from(original);
            let converted: bool = any_value.into();
            assert_eq!(converted, original);
        }
    }

    // ### String types ###

    #[test]
    fn test_string_conversion() {
        let test_strings = [
            "Hello, World!",
            "",
            "Привет, мир!",
            "🎉 Unicode test 🚀",
            "Multi\nLine\nString",
            "String with special chars: !@#$%^&*()",
        ];

        for original in test_strings {
            // Test &str -> AnyValue -> String round trip
            let any_value = AnyValue::from(original);
            let converted: String = any_value.into();
            assert_eq!(converted, original);

            // Test String -> AnyValue -> String round trip
            let string_original = original.to_string();
            let any_value = AnyValue::from(string_original.clone());
            let converted: String = any_value.into();
            assert_eq!(converted, string_original);
        }
    }

    // ### Type differentiation tests ###

    #[test]
    fn test_mixed_type_serialization() {
        // Test that different types produce different byte representations
        let int_value = AnyValue::from(42i32);
        let float_value = AnyValue::from(42.0f32);
        let string_value = AnyValue::from("42");

        // These should have different byte representations
        assert_ne!(int_value.as_bytes(), float_value.as_bytes());
        assert_ne!(int_value.as_bytes(), string_value.as_bytes());
        assert_ne!(float_value.as_bytes(), string_value.as_bytes());

        // Verify round trip conversion still works correctly
        let int_converted: i32 = int_value.into();
        let float_converted: f32 = float_value.into();
        let string_converted: String = string_value.into();

        assert_eq!(int_converted, 42i32);
        assert_eq!(float_converted, 42.0f32);
        assert_eq!(string_converted, "42");
    }

    #[test]
    fn test_multiple_roundtrip_consistency() {
        // Test that multiple roundtrips don't corrupt data for various types

        // Test with i64
        let original_i64: i64 = 12345678901234567;
        let mut value = AnyValue::from(original_i64);
        for _ in 0..10 {
            let intermediate: i64 = value.clone().into();
            value = AnyValue::from(intermediate);
        }
        let final_i64: i64 = value.into();
        assert_eq!(final_i64, original_i64);

        // Test with f64
        let original_f64: f64 = 3.141592653589793;
        let mut value = AnyValue::from(original_f64);
        for _ in 0..10 {
            let intermediate: f64 = value.clone().into();
            value = AnyValue::from(intermediate);
        }
        let final_f64: f64 = value.into();
        assert_eq!(final_f64, original_f64);

        // Test with String
        let original_string = "Hello, multiple roundtrips!".to_string();
        let mut value = AnyValue::from(original_string.clone());
        for _ in 0..10 {
            let intermediate: String = value.clone().into();
            value = AnyValue::from(intermediate);
        }
        let final_string: String = value.into();
        assert_eq!(final_string, original_string);
    }
}
