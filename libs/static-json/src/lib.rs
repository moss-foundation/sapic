use core::ops::Index;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Value<'a> {
    Null(()),
    Bool(bool),
    Float(f64),
    Int(i64),
    Str(&'a str),
    Array(&'a [Value<'a>]),
    Object(&'a [(&'a str, Value<'a>)]),
}

impl Value<'_> {
    pub const fn get_value(&self, key: &str) -> Option<&Self> {
        match self {
            Self::Object(obj) => {
                let mut i = 0;
                while i < obj.len() {
                    let (k, v) = &obj[i];
                    if Self::string_eq(k, key) {
                        return Some(v);
                    }
                    i += 1;
                }
                None
            }
            _ => None,
        }
    }

    pub const fn get_idx(&self, index: usize) -> Option<&Self> {
        match self {
            Self::Array(arr) => Some(&arr[index]),
            _ => None,
        }
    }

    pub const fn as_null(&self) -> Option<()> {
        match *self {
            Self::Null(inner) => Some(inner),
            _ => None,
        }
    }

    pub const fn as_bool(&self) -> Option<bool> {
        match *self {
            Self::Bool(inner) => Some(inner),
            _ => None,
        }
    }

    pub const fn as_float(&self) -> Option<f64> {
        match *self {
            Self::Float(inner) => Some(inner),
            Self::Int(inner) => Some(inner as f64),
            _ => None,
        }
    }

    pub const fn as_int(&self) -> Option<i64> {
        match *self {
            Self::Int(inner) => Some(inner),
            _ => None,
        }
    }

    pub const fn as_str(&self) -> Option<&str> {
        match *self {
            Self::Str(inner) => Some(inner),
            _ => None,
        }
    }

    const fn string_eq(l: &str, r: &str) -> bool {
        if l.len() != r.len() {
            return false;
        }
        let mut idx = 0;
        while idx < l.len() {
            if l.as_bytes()[idx] != r.as_bytes()[idx] {
                return false;
            }
            idx += 1;
        }
        return true;
    }
}

impl<'a> Index<usize> for Value<'a> {
    type Output = Value<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_idx(index).expect("must be an array")
    }
}

impl<'a> Index<&'a str> for Value<'a> {
    type Output = Value<'a>;

    fn index(&self, index: &'a str) -> &Self::Output {
        self.get_value(index).expect("must be an object")
    }
}

impl core::fmt::Debug for Value<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Value::Null(()) => f.write_str("null"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Float(fl) => write!(f, "{fl}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Str(s) => write!(f, "{s:?}"),
            Value::Array(a) => write!(f, "{a:?}"),

            Value::Object(o) => {
                f.write_str("{")?;
                let mut idx = 0;
                while idx < o.len() {
                    let (k, v) = o[idx];
                    write!(f, " {k:?}: {v:?}")?;
                    if idx < o.len() - 1 {
                        f.write_str(",")?;
                    }

                    idx += 1;
                }

                f.write_str(" }")
            }
        }
    }
}

impl From<Value<'_>> for serde_json::Value {
    fn from(value: Value<'_>) -> Self {
        match value {
            Value::Null(()) => serde_json::Value::Null,
            Value::Bool(b) => serde_json::Value::Bool(b),
            Value::Float(f) => serde_json::Value::Number(
                serde_json::Number::from_f64(f).unwrap_or_else(|| serde_json::Number::from(0)),
            ),
            Value::Int(i) => serde_json::Value::Number(serde_json::Number::from(i)),
            Value::Str(s) => serde_json::Value::String(s.to_string()),
            Value::Array(arr) => {
                let vec: Vec<serde_json::Value> =
                    arr.iter().map(|v| serde_json::Value::from(*v)).collect();
                serde_json::Value::Array(vec)
            }
            Value::Object(obj) => {
                let mut map = serde_json::Map::new();
                for (key, val) in obj.iter() {
                    map.insert(key.to_string(), serde_json::Value::from(*val));
                }
                serde_json::Value::Object(map)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_conversion() {
        let value = Value::Null(());
        let json_value: serde_json::Value = value.into();
        assert_eq!(json_value, serde_json::Value::Null);
    }

    #[test]
    fn test_bool_conversion() {
        let value_true = Value::Bool(true);
        let json_value_true: serde_json::Value = value_true.into();
        assert_eq!(json_value_true, serde_json::Value::Bool(true));

        let value_false = Value::Bool(false);
        let json_value_false: serde_json::Value = value_false.into();
        assert_eq!(json_value_false, serde_json::Value::Bool(false));
    }

    #[test]
    fn test_int_conversion() {
        let value = Value::Int(42);
        let json_value: serde_json::Value = value.into();
        assert_eq!(
            json_value,
            serde_json::Value::Number(serde_json::Number::from(42))
        );

        let value_negative = Value::Int(-123);
        let json_value_negative: serde_json::Value = value_negative.into();
        assert_eq!(
            json_value_negative,
            serde_json::Value::Number(serde_json::Number::from(-123))
        );

        let value_zero = Value::Int(0);
        let json_value_zero: serde_json::Value = value_zero.into();
        assert_eq!(
            json_value_zero,
            serde_json::Value::Number(serde_json::Number::from(0))
        );
    }

    #[test]
    fn test_float_conversion() {
        let value = Value::Float(3.14);
        let json_value: serde_json::Value = value.into();
        assert_eq!(
            json_value,
            serde_json::Value::Number(serde_json::Number::from_f64(3.14).unwrap())
        );

        let value_negative = Value::Float(-2.718);
        let json_value_negative: serde_json::Value = value_negative.into();
        assert_eq!(
            json_value_negative,
            serde_json::Value::Number(serde_json::Number::from_f64(-2.718).unwrap())
        );

        let value_zero = Value::Float(0.0);
        let json_value_zero: serde_json::Value = value_zero.into();
        assert_eq!(
            json_value_zero,
            serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap())
        );
    }

    #[test]
    fn test_float_infinity_conversion() {
        // Test handling of infinity and NaN - should fallback to 0
        let value_inf = Value::Float(f64::INFINITY);
        let json_value_inf: serde_json::Value = value_inf.into();
        assert_eq!(
            json_value_inf,
            serde_json::Value::Number(serde_json::Number::from(0))
        );

        let value_neg_inf = Value::Float(f64::NEG_INFINITY);
        let json_value_neg_inf: serde_json::Value = value_neg_inf.into();
        assert_eq!(
            json_value_neg_inf,
            serde_json::Value::Number(serde_json::Number::from(0))
        );

        let value_nan = Value::Float(f64::NAN);
        let json_value_nan: serde_json::Value = value_nan.into();
        assert_eq!(
            json_value_nan,
            serde_json::Value::Number(serde_json::Number::from(0))
        );
    }

    #[test]
    fn test_string_conversion() {
        let value = Value::Str("hello world");
        let json_value: serde_json::Value = value.into();
        assert_eq!(
            json_value,
            serde_json::Value::String("hello world".to_string())
        );

        let value_empty = Value::Str("");
        let json_value_empty: serde_json::Value = value_empty.into();
        assert_eq!(json_value_empty, serde_json::Value::String("".to_string()));

        let value_unicode = Value::Str("Rust");
        let json_value_unicode: serde_json::Value = value_unicode.into();
        assert_eq!(
            json_value_unicode,
            serde_json::Value::String("Rust".to_string())
        );
    }

    #[test]
    fn test_array_conversion() {
        let array_data = [
            Value::Int(1),
            Value::Str("hello"),
            Value::Bool(true),
            Value::Null(()),
        ];
        let value = Value::Array(&array_data);
        let json_value: serde_json::Value = value.into();

        let expected = serde_json::Value::Array(vec![
            serde_json::Value::Number(serde_json::Number::from(1)),
            serde_json::Value::String("hello".to_string()),
            serde_json::Value::Bool(true),
            serde_json::Value::Null,
        ]);

        assert_eq!(json_value, expected);
    }

    #[test]
    fn test_empty_array_conversion() {
        let array_data: [Value; 0] = [];
        let value = Value::Array(&array_data);
        let json_value: serde_json::Value = value.into();

        let expected = serde_json::Value::Array(vec![]);
        assert_eq!(json_value, expected);
    }

    #[test]
    fn test_object_conversion() {
        let object_data = [
            ("name", Value::Str("John")),
            ("age", Value::Int(30)),
            ("active", Value::Bool(true)),
            ("balance", Value::Float(123.45)),
        ];
        let value = Value::Object(&object_data);
        let json_value: serde_json::Value = value.into();

        let mut expected_map = serde_json::Map::new();
        expected_map.insert(
            "name".to_string(),
            serde_json::Value::String("John".to_string()),
        );
        expected_map.insert(
            "age".to_string(),
            serde_json::Value::Number(serde_json::Number::from(30)),
        );
        expected_map.insert("active".to_string(), serde_json::Value::Bool(true));
        expected_map.insert(
            "balance".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(123.45).unwrap()),
        );
        let expected = serde_json::Value::Object(expected_map);

        assert_eq!(json_value, expected);
    }

    #[test]
    fn test_empty_object_conversion() {
        let object_data: [(&str, Value); 0] = [];
        let value = Value::Object(&object_data);
        let json_value: serde_json::Value = value.into();

        let expected = serde_json::Value::Object(serde_json::Map::new());
        assert_eq!(json_value, expected);
    }

    #[test]
    fn test_nested_structure_conversion() {
        let inner_array = [Value::Int(1), Value::Int(2), Value::Int(3)];
        let inner_object = [("x", Value::Int(10)), ("y", Value::Int(20))];

        let outer_object = [
            ("numbers", Value::Array(&inner_array)),
            ("point", Value::Object(&inner_object)),
            ("metadata", Value::Null(())),
        ];

        let value = Value::Object(&outer_object);
        let json_value: serde_json::Value = value.into();

        let mut expected_map = serde_json::Map::new();
        expected_map.insert(
            "numbers".to_string(),
            serde_json::Value::Array(vec![
                serde_json::Value::Number(serde_json::Number::from(1)),
                serde_json::Value::Number(serde_json::Number::from(2)),
                serde_json::Value::Number(serde_json::Number::from(3)),
            ]),
        );

        let mut inner_expected_map = serde_json::Map::new();
        inner_expected_map.insert(
            "x".to_string(),
            serde_json::Value::Number(serde_json::Number::from(10)),
        );
        inner_expected_map.insert(
            "y".to_string(),
            serde_json::Value::Number(serde_json::Number::from(20)),
        );
        expected_map.insert(
            "point".to_string(),
            serde_json::Value::Object(inner_expected_map),
        );

        expected_map.insert("metadata".to_string(), serde_json::Value::Null);

        let expected = serde_json::Value::Object(expected_map);
        assert_eq!(json_value, expected);
    }

    #[test]
    fn test_deeply_nested_arrays() {
        let level2 = [Value::Int(42)];
        let level1 = [Value::Array(&level2)];
        let level0 = Value::Array(&level1);

        let json_value: serde_json::Value = level0.into();

        let expected = serde_json::Value::Array(vec![serde_json::Value::Array(vec![
            serde_json::Value::Number(serde_json::Number::from(42)),
        ])]);

        assert_eq!(json_value, expected);
    }
}
