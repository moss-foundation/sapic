use hcl::{
    Identifier, Value,
    expr::{Expression, Object as HclObject, ObjectKey},
};
use serde::{Deserialize, Serialize};
use std::{fmt, ops};

/// A transparent wrapper type which creates HCL objects with unquoted keys.
///
/// When serialized with HCL, this produces objects where keys appear without quotes
/// in the HCL output, similar to how Terraform configuration blocks work.
///
/// # Example
///
/// ```rust
/// use serde::Serialize;
/// use indexmap::IndexMap;
///
/// #[derive(Serialize)]
/// struct Config {
///     #[serde(serialize_with = "object_with_unquoted_keys")]
///     options: HeaderParameterOptionsObject,
/// }
///
/// #[derive(Serialize)]
/// struct HeaderParameterOptionsObject {
///     propagate: bool,
/// }
///
/// // This will serialize as:
/// // options = {
/// //   propagate = true
/// // }
/// ```
pub struct Object<T>(T);

impl<T> Object<T> {
    /// Create a new `Object<T>` from a `T`.
    pub fn new(value: T) -> Object<T> {
        Object(value)
    }

    /// Consume the `Object` and return the wrapped `T`.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> ops::Deref for Object<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for Object<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Clone for Object<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Object(self.0.clone())
    }
}

impl<T> fmt::Debug for Object<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Object").field(&self.0).finish()
    }
}

impl<T> Serialize for Object<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Convert to HCL Expression with unquoted keys
        let expression = object_to_hcl_expression(&self.0).map_err(serde::ser::Error::custom)?;
        expression.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Object<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Object)
    }
}

/// Convert a serializable value to HCL Expression with unquoted keys
fn object_to_hcl_expression<T>(value: &T) -> Result<Expression, hcl::Error>
where
    T: Serialize,
{
    // First convert to HCL Value
    let hcl_value = hcl::to_value(value)?;

    // Then convert to Expression with unquoted keys
    match hcl_value {
        Value::Object(map) => {
            let mut object_items = Vec::new();
            for (key, value) in map {
                // Create unquoted key using Identifier
                let object_key = ObjectKey::Identifier(Identifier::new(key)?);
                let expr = hcl::to_expression(&value)?;
                object_items.push((object_key, expr));
            }
            Ok(Expression::Object(HclObject::from(object_items)))
        }
        _ => {
            // For non-object values, convert directly to expression
            hcl::to_expression(&hcl_value)
        }
    }
}

/// Serialize `T` as an HCL object with unquoted keys.
///
/// This function is intended to be used in the `#[serde(serialize_with)]` attribute.
///
/// # Example
///
/// ```rust
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Config {
///     #[serde(serialize_with = "object_with_unquoted_keys")]
///     options: HeaderParameterOptionsObject,
/// }
///
/// #[derive(Serialize)]
/// struct HeaderParameterOptionsObject {
///     propagate: bool,
/// }
/// ```
pub fn object_with_unquoted_keys<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: serde::Serializer,
{
    let expression = object_to_hcl_expression(value).map_err(serde::ser::Error::custom)?;
    expression.serialize(serializer)
}

/// Convenience function that wraps value in Object<T> and serializes it.
pub fn object<T, S>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: serde::Serializer,
{
    Object::new(value).serialize(serializer)
}
