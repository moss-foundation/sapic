use std::path::Path;

use moss_db::{AnyEntity, ReDbClient, bincode_table::BincodeTable};

use crate::item_store::store_impl::ItemStoreImpl;

use super::NamespacedStore;

#[rustfmt::skip]
pub(in crate::workspace_storage) const NAMESPACED_STORE: BincodeTable<String, AnyEntity> = BincodeTable::new("namespaced");

pub struct NamespacedStoreImpl {
    inner: ItemStoreImpl,
}

impl NamespacedStoreImpl {
    pub fn new(client: ReDbClient) -> Self {
        Self {
            inner: ItemStoreImpl::new(client, NAMESPACED_STORE),
        }
    }

    // fn namespace(&self, namespace: &str) {
    //     Namespace { inner: &self.inner }
    // }
}

impl NamespacedStore for NamespacedStoreImpl {}
