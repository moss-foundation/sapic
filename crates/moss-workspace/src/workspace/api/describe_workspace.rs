use anyhow::Result;

use crate::{
    models::{
        operations::DescribeWorkspaceOutput,
        types::{CollectionInfo, EnvironmentInfo},
    },
    workspace::Workspace,
};

impl Workspace {
    pub async fn describe(&self) -> Result<DescribeWorkspaceOutput> {
        let collections_info = self.list_collections().await?;
        let environments_info = self.list_environments().await?;

        Ok(DescribeWorkspaceOutput {
            collections: collections_info,
            environments: environments_info,
        })
    }

    pub(crate) async fn list_environments(&self) -> Result<Vec<EnvironmentInfo>> {
        let environments = self.environments().await?;
        let environments_lock = environments.read().await;

        let global_environments_info_iter = environments_lock
            .iter()
            .filter(|(_, iter_slot)| !iter_slot.is_leased())
            .map(|(key, iter_slot)| {
                let (_, cache) = iter_slot.value();
                EnvironmentInfo {
                    key: key.as_u64(),
                    collection_key: None,
                    name: cache.decoded_name.clone(),
                    order: cache.order,
                }
            });

        // TODO: get environments from collections

        let mut environments_info = Vec::new();
        environments_info.extend(global_environments_info_iter);

        Ok(environments_info)
    }

    pub(crate) async fn list_collections(&self) -> Result<Vec<CollectionInfo>> {
        let collections = self.collections().await?;
        let collections_lock = collections.read().await;
        let collections_info = collections_lock
            .iter()
            .filter(|(_, iter_slot)| !iter_slot.is_leased())
            .map(|(key, iter_slot)| {
                let (_, metadata) = iter_slot.value();
                CollectionInfo {
                    key: key.as_u64(),
                    name: metadata.name.clone(),
                    order: metadata.order,
                }
            })
            .collect();

        Ok(collections_info)
    }
}
