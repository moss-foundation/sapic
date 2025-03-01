use moss_fs::ports::FileSystem;
use parking_lot::RwLock;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::models::collection::{CollectionRequestVariantEntry, RequestType};



pub struct RequestState {
    pub name: String,
    pub order: Option<usize>,
    pub typ: Option<RequestType>,
    pub variants: RwLock<HashMap<PathBuf, CollectionRequestVariantEntry>>,
}


pub(crate) struct RequestHandle {
    pub fs: Arc<dyn FileSystem>,
    pub state: RequestState,
}

impl RequestHandle {
    pub fn new(fs: Arc<dyn FileSystem>, state: RequestState) -> Self {
        Self { fs, state }
    }
}
