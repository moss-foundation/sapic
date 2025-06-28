use hcl::{
    Identifier, Value,
    expr::{Expression, Object as HclObject, ObjectKey},
};
use serde::{Deserialize, Serialize};
use std::{fmt, ops};

/// A transparent wrapper type that serializes structs as HCL objects with unquoted keys.
///
/// This wrapper ensures that when serialized to HCL, object keys appear without quotes,
/// matching the style of Terraform configuration files and other HCL documents.
///
/// Without `Object<T>`, keys would be quoted:
/// ```hcl
/// options = {
///   "propagate" = true
///   "cache" = false
/// }
/// ```
///
/// With `Object<T>`, keys are unquoted (Terraform-style):
/// ```hcl
/// options = {
///   propagate = true
///   cache = false
/// }
/// ```
#[derive(PartialEq, Eq, Hash)]
pub struct Object<T>(T);

impl<T> Object<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Object(value)
    }
}

impl<T> ops::Deref for Object<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for Object<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Clone for Object<T>
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Object(self.0.clone())
    }
}

impl<T> Copy for Object<T> where T: Copy {}

impl<T> Default for Object<T>
where
    T: Default,
{
    #[inline]
    fn default() -> Self {
        Object(T::default())
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

impl<T> fmt::Display for Object<T>
where
    T: fmt::Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<T> for Object<T> {
    #[inline]
    fn from(value: T) -> Self {
        Object::new(value)
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
        convert_to_hcl_object_expression(&self.0)
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Object<T>
where
    T: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Object::new)
    }
}

/// Converts a serializable value to an HCL Expression with unquoted object keys.
///
/// This function handles the conversion from Rust values to HCL expressions,
/// ensuring that object keys appear without quotes in the final HCL output.
fn convert_to_hcl_object_expression<T>(value: &T) -> Result<Expression, hcl::Error>
where
    T: Serialize,
{
    let hcl_value = hcl::to_value(value)?;

    match hcl_value {
        Value::Object(map) => {
            // Convert map entries to HCL object items with unquoted keys
            let object_items: Result<Vec<_>, hcl::Error> = map
                .into_iter()
                .map(|(key, value)| {
                    let object_key = ObjectKey::Identifier(Identifier::new(key)?);
                    let expr = hcl::to_expression(&value)?;
                    Ok((object_key, expr))
                })
                .collect();

            Ok(Expression::Object(HclObject::from(object_items?)))
        }
        _ => {
            // For non-object values, convert directly to expression
            hcl::to_expression(&hcl_value)
        }
    }
}
