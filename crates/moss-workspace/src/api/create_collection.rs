use moss_api::ext::ValidationResultExt;
use moss_applib::AppRuntime;
use moss_git_hosting_provider::models::primitives::GitProviderType;
use validator::Validate;

use crate::{
    models::{
        operations::{CreateCollectionInput, CreateCollectionOutput},
        types::CreateCollectionGitParams,
    },
    services::collection_service::{CollectionItemCreateParams, CollectionItemGitCreateParams},
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn create_collection(
        &self,
        ctx: &R::AsyncContext,
        input: &CreateCollectionInput,
    ) -> joinerror::Result<CreateCollectionOutput> {
        input.validate().join_err_bare()?;

        let git_params = if let Some(git_params) = &input.git_params {
            match git_params {
                CreateCollectionGitParams::GitHub(p) => Some(CollectionItemGitCreateParams {
                    repository: p.repository.clone(),
                    git_provider_type: GitProviderType::GitHub,
                    branch: p.branch.clone(),
                }),
                CreateCollectionGitParams::GitLab(p) => Some(CollectionItemGitCreateParams {
                    repository: p.repository.clone(),
                    git_provider_type: GitProviderType::GitLab,
                    branch: p.branch.clone(),
                }),
            }
        } else {
            None
        };

        debug_assert!(input.external_path.is_none(), "Is not implemented");

        let description = self
            .collection_service
            .create_collection(
                ctx,
                CollectionItemCreateParams {
                    name: input.name.to_owned(),
                    order: input.order.to_owned(),
                    external_path: input.external_path.to_owned(),
                    icon_path: input.icon_path.to_owned(),
                    git_params,
                },
            )
            .await?;

        Ok(CreateCollectionOutput {
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
