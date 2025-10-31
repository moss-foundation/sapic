// use serde_json::Value as JsonValue;
// use std::collections::HashMap;

// /// Binary Blob Type
// ///
// /// Blobs represent protocol-agnostic binary content.
// #[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
// pub struct Blob {
//     inner: Vec<u8>,
// }

// impl Blob {
//     /// Creates a new blob from the given `input`.
//     pub fn new<T: Into<Vec<u8>>>(input: T) -> Self {
//         Blob {
//             inner: input.into(),
//         }
//     }

//     /// Consumes the `Blob` and returns a `Vec<u8>` with its contents.
//     pub fn into_inner(self) -> Vec<u8> {
//         self.inner
//     }
// }

// impl AsRef<[u8]> for Blob {
//     fn as_ref(&self) -> &[u8] {
//         &self.inner
//     }
// }

// impl From<Vec<u8>> for Blob {
//     fn from(value: Vec<u8>) -> Self {
//         Blob::new(value)
//     }
// }

// impl From<Blob> for Vec<u8> {
//     fn from(value: Blob) -> Self {
//         value.into_inner()
//     }
// }

// impl From<&[u8]> for Blob {
//     fn from(value: &[u8]) -> Self {
//         Blob::new(value)
//     }
// }

// pub enum AttributeValue {
//     Blob(Blob),
//     Bool(bool),
//     BlobSet(Vec<Blob>),
//     List(Vec<AttributeValue>),
//     Map(HashMap<String, AttributeValue>),
//     Number(f64),
//     NumberSet(Vec<f64>),
//     Null(bool),
//     String(String),
//     StringSet(Vec<String>),
// }
