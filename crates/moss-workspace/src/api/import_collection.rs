use moss_api::ext::ValidationResultExt;
use moss_applib::AppRuntime;
use moss_git_hosting_provider::models::primitives::GitProviderType;
use validator::Validate;

use crate::{
    Workspace,
    models::{
        operations::{ImportCollectionInput, ImportCollectionOutput},
        types::ImportCollectionParams,
    },
    services::collection_service::{CollectionItemCloneParams, CollectionItemGitCloneParams},
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn import_collection(
        &self,
        ctx: &R::AsyncContext,
        input: &ImportCollectionInput,
    ) -> joinerror::Result<ImportCollectionOutput> {
        input.validate().join_err_bare()?;
        let (repository, git_provider_type, branch) = match &input.params {
            ImportCollectionParams::GitHub(params) => {
                params.validate().join_err_bare()?;
                (
                    params.repository.clone(),
                    GitProviderType::GitHub,
                    params.branch.clone(),
                )
            }
            ImportCollectionParams::GitLab(params) => {
                params.validate().join_err_bare()?;
                (
                    params.repository.clone(),
                    GitProviderType::GitLab,
                    params.branch.clone(),
                )
            }
        };

        let description = self
            .collection_service
            .clone_collection(
                ctx,
                CollectionItemCloneParams {
                    name: input.name.clone(),
                    order: input.order,
                    icon_path: input.icon_path.clone(),
                    git_params: CollectionItemGitCloneParams {
                        repository,
                        git_provider_type,
                        branch,
                    },
                },
            )
            .await?;

        Ok(ImportCollectionOutput {
            id: description.id,
            name: description.name,
            order: description.order,
            expanded: description.expanded,
            icon_path: description.icon_path,
            abs_path: description.abs_path,
            external_path: description.external_path,
        })
    }
}
