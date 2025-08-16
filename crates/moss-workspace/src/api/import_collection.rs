use moss_api::ext::ValidationResultExt;
use moss_applib::AppRuntime;
use moss_git_hosting_provider::models::primitives::GitProviderType;
use validator::Validate;

use crate::{
    Workspace,
    models::{
        operations::{ImportCollectionInput, ImportCollectionOutput},
        types::ImportCollectionSource,
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
        let params = &input.inner;
        let description = match &params.source {
            ImportCollectionSource::GitHub(git_params) => self.collection_service.clone_collection(
                ctx,
                CollectionItemCloneParams {
                    _name: params.name.clone(),
                    order: params.order,
                    _icon_path: params.icon_path.clone(),
                    git_params: CollectionItemGitCloneParams {
                        repository: git_params.repository.clone(),
                        git_provider_type: GitProviderType::GitHub,
                        branch: git_params.branch.clone(),
                    },
                },
            ),
            ImportCollectionSource::GitLab(git_params) => self.collection_service.clone_collection(
                ctx,
                CollectionItemCloneParams {
                    _name: params.name.clone(),
                    order: params.order,
                    _icon_path: params.icon_path.clone(),
                    git_params: CollectionItemGitCloneParams {
                        repository: git_params.repository.clone(),
                        git_provider_type: GitProviderType::GitLab,
                        branch: git_params.branch.clone(),
                    },
                },
            ), // TODO: Support importing from other apps
        }
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
