use crate::workspace::{EnvironmentKey, Workspace};
use anyhow::Result;

pub struct DescribeEnvironmentInput {
    pub key: u64,
}

pub struct DescribeEnvironmentOutput {
    // pub variables:
    // pub environment: EnvironmentInfo,
}

impl Workspace {
    pub async fn describe_environment(
        &self,
        input: DescribeEnvironmentInput,
    ) -> Result<DescribeEnvironmentOutput> {
        let environments = self.environments().await?;
        let environments_lock = environments.read().await;

        let environment_key = EnvironmentKey::from(input.key);
        let (environment, environment_cache) = environments_lock.read(environment_key)?;

        // Ok(DescribeEnvironmentOutput {
        //     environment: environment.into(),
        // })

        todo!()
    }
}
