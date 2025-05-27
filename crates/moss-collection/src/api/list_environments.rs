use moss_common::api::OperationResult;

use crate::{Collection, models::types::EnvironmentInfo};

impl Collection {
    pub async fn list_environments(&self) -> OperationResult<Vec<EnvironmentInfo>> {
        let environments = self.environments().await?;
        let environments_lock = environments.read().await;

        // TODO: restore order from cache

        let environments = environments_lock
            .values()
            .map(|item| EnvironmentInfo {
                id: item.id,
                name: item.name.clone(),
                order: None, // TODO: restore order from cache
            })
            .collect();

        Ok(environments)
    }
}
