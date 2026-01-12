use async_trait::async_trait;
use indexmap::IndexMap;
use moss_common::continue_if_err;
use moss_environment::configuration::VariableDecl;
use moss_hcl::json_to_hcl;
use sapic_base::{
    environment::types::primitives::VariableId, workspace::types::primitives::WorkspaceId,
};
use sapic_core::context::AnyAsyncContext;
use std::{path::PathBuf, sync::Arc};

use crate::environment::{
    AppEnvironmentServiceFs, CreateEnvironmentFsParams, EnvironmentCreateOp,
    environment_service::CreateEnvironmentItemParams,
};

// This struct is used for creating workspace predefined environment after workspace creation
pub struct AppEnvironmentService {
    backend: Arc<dyn AppEnvironmentServiceFs>,
}

impl AppEnvironmentService {
    pub fn new(backend: Arc<dyn AppEnvironmentServiceFs>) -> Self {
        Self { backend }
    }

    pub async fn create_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        params: CreateEnvironmentItemParams,
    ) -> joinerror::Result<PathBuf> {
        let mut variable_decls = IndexMap::new();
        for param in params.variables {
            let id = VariableId::new();
            let global_value = continue_if_err!(json_to_hcl(&param.global_value), |err| {
                println!("failed to convert global value expression: {}", err); // TODO: log error
            });
            let decl = VariableDecl {
                name: param.name,
                value: global_value,
                description: param.desc,
                options: param.options,
            };
            variable_decls.insert(id, decl);
        }

        let abs_path = self
            .backend
            .create_environment(
                ctx,
                workspace_id,
                &params.env_id,
                &CreateEnvironmentFsParams {
                    project_id: params.project_id,
                    name: params.name,
                    color: params.color,
                    variables: variable_decls,
                },
            )
            .await?;

        Ok(abs_path)
    }
}

#[async_trait]
impl EnvironmentCreateOp for AppEnvironmentService {
    async fn create(
        &self,
        ctx: &dyn AnyAsyncContext,
        workspace_id: &WorkspaceId,
        params: CreateEnvironmentItemParams,
    ) -> joinerror::Result<PathBuf> {
        self.create_environment(ctx, workspace_id, params).await
    }
}
