use anyhow::Result;
use moss_global_env::models::types::VariableInfo;

use crate::{
    models::{
        operations::DescribeWorkspaceOutput,
        types::{CollectionInfo, EnvironmentInfoFull},
    },
    workspace::Workspace,
};

impl Workspace {
    pub async fn describe(&self) -> Result<DescribeWorkspaceOutput> {
        let collections_info = self.list_collections().await?;
        // let environments_info = self.describe_environments().await?;

        Ok(DescribeWorkspaceOutput {
            collections: collections_info,
            // environments: environments_info,
        })
    }

    // async fn list_environments(&self) -> Result<Vec<EnvironmentInfo>> {
    //     todo!()
    // }

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

    // async fn describe_environments(&self) -> Result<Vec<EnvironmentInfo>> {
    //     let environments = self.environments().await?;
    //     let environments_lock = environments.read().await;

    //     let mut environments_info = Vec::new();
    //     for (key, iter_slot) in environments_lock
    //         .iter()
    //         .filter(|(_, iter_slot)| !iter_slot.is_leased())
    //     {
    //         let (env, cache) = iter_slot.value();
    //         let variables_lock = env.variables().read().await;
    //         let variables = variables_lock
    //             .iter()
    //             .map(|(name, var)| {
    //                 let variable_cache =
    //                     cache.variables_cache.get(name).cloned().unwrap_or_default();

    //                 (
    //                     name.clone(),
    //                     VariableInfo {
    //                         global_value: var.value.clone(),
    //                         local_value: variable_cache.local_value,
    //                         desc: var.desc.clone(),
    //                         disabled: variable_cache.disabled,
    //                         kind: var.kind.clone(),
    //                         order: variable_cache.order,
    //                     },
    //                 )
    //             })
    //             .collect();

    //         environments_info.push(EnvironmentInfo {
    //             key: key.as_u64(),
    //             name: cache.decoded_name.clone(),
    //             order: cache.order,
    //             variables,
    //         });
    //     }

    //     Ok(environments_info)
    // }
}
