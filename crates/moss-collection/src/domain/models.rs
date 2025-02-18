use serde::{Deserialize, Serialize};
use std::{any::Any, borrow::Cow, path::PathBuf, sync::Arc};

// #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
// pub enum CollectionKind {
//     Local,
//     Remote,
// }

pub struct LocalCollection {
    pub path: PathBuf,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub enum CollectionSource {
    Local(PathBuf),
    Remote(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionDetails {
    pub order: usize,
    pub source: CollectionSource,
}

impl CollectionDetails {
    pub fn source(&self) -> Cow<'_, str> {
        match &self.source {
            CollectionSource::Local(path) => path
                .to_str()
                .map(Cow::Borrowed)
                .unwrap_or_else(|| Cow::Owned(path.to_string_lossy().into_owned())),
            CollectionSource::Remote(url) => Cow::Borrowed(url),
        }
    }
}
