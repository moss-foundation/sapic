use joinerror::OptionExt;
use moss_applib::{AppHandle, AppRuntime, errors::ValidationResultExt};
use moss_git_hosting_provider::GitProviderKind;
use validator::Validate;

use crate::{
    Workspace,
    models::{
        operations::{ImportCollectionInput, ImportCollectionOutput},
        primitives::CollectionId,
        types::ImportCollectionSource,
    },
    services::collection_service::{CollectionItemCloneParams, CollectionItemImportParams},
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn import_collection(
        &self,
        ctx: &R::AsyncContext,
        app_handle: &AppHandle<R>,
        input: &ImportCollectionInput,
    ) -> joinerror::Result<ImportCollectionOutput> {
        input.validate().join_err_bare()?;

        let params = &input.inner;
        let id = CollectionId::new();

        let description = match &params.source {
            ImportCollectionSource::GitHub(git_params) => {
                let session = self
                    .active_profile
                    .account(&git_params.account_id)
                    .await
                    .ok_or_join_err::<()>("account not found")?;

                self.collection_service
                    .clone_collection(
                        ctx,
                        app_handle,
                        &id,
                        session,
                        CollectionItemCloneParams {
                            order: params.order,
                            account_id: git_params.account_id.to_owned(),
                            repository: git_params.repository.clone(),
                            git_provider_type: GitProviderKind::GitHub,
                            branch: git_params.branch.clone(),
                        },
                    )
                    .await?
            }
            ImportCollectionSource::GitLab(git_params) => {
                let session = self
                    .active_profile
                    .account(&git_params.account_id)
                    .await
                    .ok_or_join_err::<()>("account not found")?;

                self.collection_service
                    .clone_collection(
                        ctx,
                        app_handle,
                        &id,
                        session,
                        CollectionItemCloneParams {
                            order: params.order,
                            account_id: git_params.account_id.to_owned(),
                            repository: git_params.repository.clone(),
                            git_provider_type: GitProviderKind::GitLab,
                            branch: git_params.branch.clone(),
                        },
                    )
                    .await?
            }
            ImportCollectionSource::Archive(archive_params) => {
                self.collection_service
                    .import_collection(
                        ctx,
                        &id,
                        CollectionItemImportParams {
                            name: params.name.clone(),
                            order: params.order,
                            archive_path: archive_params.archive_path.clone(),
                        },
                    )
                    .await?
            } // TODO: Support importing from other apps
        };

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
