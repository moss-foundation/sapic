use moss_collection::collection::CollectionKind;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionEntity {
    pub kind: CollectionKind,
    pub order: usize,
    // data: Vec<u8>,
}

pub struct PutCollectionInput {
    pub source: String,
    pub kind: CollectionKind,
    pub order: usize,
}

pub struct RemoveCollectionInput<'a> {
    pub source: &'a str,
}
