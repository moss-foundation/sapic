pub mod collection;
pub mod indexing;
pub mod storage;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

pub struct LocalCollection {
    pub path: PathBuf,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub enum CollectionSource {
    Local(PathBuf),
    Remote(String),
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct RequestVariant {
//     pub order: usize,
// }
// #[derive(Serialize, Deserialize, Debug)]
// pub struct RequestMetadata {
//     pub order: usize,
//     pub variants: HashMap<String, RequestVariant>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct CollectionMetadata {
//     pub order: usize,
//     pub requests: HashMap<Vec<u8>, RequestMetadata>,
//     // pub source: CollectionSource,
// }

// impl CollectionMetadata {
//     pub fn source(&self) -> Cow<'_, str> {
//         match &self.source {
//             CollectionSource::Local(path) => path
//                 .to_str()
//                 .map(Cow::Borrowed)
//                 .unwrap_or_else(|| Cow::Owned(path.to_string_lossy().into_owned())),
//             CollectionSource::Remote(url) => Cow::Borrowed(url),
//         }
//     }
// }
