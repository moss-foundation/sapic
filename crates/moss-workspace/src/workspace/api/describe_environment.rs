use anyhow::Result;
use moss_types::environment::types::VariableInfo;
use tauri::Runtime as TauriRuntime;

use crate::{
    models::operations::{DescribeEnvironmentInput, DescribeEnvironmentOutput},
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn describe_environment(
        &self,
        input: DescribeEnvironmentInput,
    ) -> Result<DescribeEnvironmentOutput> {
        let environments = self.environments().await?;
        let environments_lock = environments.read().await;

        let (environment, environment_cache) = environments_lock.get(input.key)?;

        let variables_lock = environment.variables().read().await;
        let variables: Vec<VariableInfo> = variables_lock
            .iter()
            .map(|(name, var)| {
                let variable_cache = environment_cache
                    .variables_cache
                    .get(name)
                    .cloned()
                    .unwrap_or_default();

                VariableInfo {
                    name: name.clone(),
                    global_value: var.value.clone(),
                    local_value: variable_cache.local_value,
                    desc: var.desc.clone(),
                    disabled: variable_cache.disabled,
                    kind: var.kind.clone(),
                    order: variable_cache.order,
                }
            })
            .collect();

        Ok(DescribeEnvironmentOutput { variables })
    }
}
