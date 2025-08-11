use moss_api::ext::ValidationResultExt;
use moss_applib::AppRuntime;
use moss_git_hosting_provider::common::GitProviderType;
use validator::Validate;

use crate::{
    Workspace,
    models::{
        operations::{ImportCollectionInput, ImportCollectionOutput},
        primitives::CollectionId,
    },
    services::collection_service::CollectionItemCloneParams,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn import_collection(
        &self,
        ctx: &R::AsyncContext,
        input: &ImportCollectionInput,
    ) -> joinerror::Result<ImportCollectionOutput> {
        let id = CollectionId::new();

        let description = match input {
            ImportCollectionInput::GitHub(params) => {
                params.validate().join_err_bare()?;
                self.collection_service
                    .clone_collection(
                        ctx,
                        &id,
                        CollectionItemCloneParams {
                            git_provider_type: GitProviderType::GitHub,
                            order: params.order,
                            repository: params.repository.clone(),
                        },
                    )
                    .await?
            }
            ImportCollectionInput::GitLab(params) => {
                params.validate().join_err_bare()?;
                self.collection_service
                    .clone_collection(
                        ctx,
                        &id,
                        CollectionItemCloneParams {
                            git_provider_type: GitProviderType::GitLab,
                            order: params.order,
                            repository: params.repository.clone(),
                        },
                    )
                    .await?
            }
        };

        Ok(ImportCollectionOutput {
            id,
            name: description.name,
            order: description.order,
            expanded: description.expanded,
            icon_path: description.icon_path,
            abs_path: description.abs_path,
            external_path: description.external_path,
        })
    }
}
