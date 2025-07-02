use moss_common::api::OperationResult;

use crate::{Collection, models::types::EnvironmentInfo};

impl Collection {
    pub async fn list_environments(&self) -> OperationResult<Vec<EnvironmentInfo>> {
        let environments = self.environments().await?;

        // TODO: restore order from cache

        let environments = environments
            .values()
            .map(|item| EnvironmentInfo {
                id: item.id.clone(),
                name: item.name.clone(),
                order: None, // TODO: restore order from cache
            })
            .collect();

        Ok(environments)
    }
}
