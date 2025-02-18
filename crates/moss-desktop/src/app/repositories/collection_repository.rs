// use std::sync::Arc;

// use anyhow::Result;

// use crate::app::models::collection::{CollectionEntity, PutCollectionInput, RemoveCollectionInput};

// pub trait CollectionRepository: Send + Sync + 'static {
//     fn put_collection_item(&self, input: PutCollectionInput) -> Result<()>;
//     fn remove_collection_item(&self, input: RemoveCollectionInput) -> Result<()>;
// }

// pub trait CollectionRequestRepository: Send + Sync + 'static {}

// pub struct SledCollectionRepository {
//     tree: Arc<sled::Tree>,
// }

// impl SledCollectionRepository {
//     pub fn new(tree: Arc<sled::Tree>) -> Self {
//         Self { tree }
//     }
// }

// impl CollectionRepository for SledCollectionRepository {
//     fn put_collection_item(&self, input: PutCollectionInput) -> Result<()> {
//         let value = bincode::serialize(&CollectionEntity {
//             kind: input.kind,
//             order: input.order,
//         })?;
//         self.tree.insert(&input.source, value)?;

//         Ok(())
//     }

//     fn remove_collection_item(&self, input: RemoveCollectionInput) -> Result<()> {
//         self.tree.remove(input.source)?;

//         Ok(())
//     }
// }

// impl CollectionRequestRepository for SledCollectionRepository {}
