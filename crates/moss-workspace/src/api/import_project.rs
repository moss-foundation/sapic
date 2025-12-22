use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use sapic_base::{other::GitProviderKind, project::types::primitives::ProjectId};
use sapic_ipc::ValidationResultExt;
use validator::Validate;

use crate::{
    Workspace,
    models::{
        operations::{ImportProjectInput, ImportProjectOutput},
        types::ImportProjectSource,
    },
    project::{
        ProjectItemCloneParams, ProjectItemImportFromArchiveParams, ProjectItemImportFromDiskParams,
    },
};

impl Workspace {
    pub async fn import_project<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: &ImportProjectInput,
    ) -> joinerror::Result<ImportProjectOutput> {
        unimplemented!()
    }
    //     input.validate().join_err_bare()?;
    //
    //     let params = &input.inner;
    //     let id = ProjectId::new();
    //
    //     let description = match &params.source {
    //         ImportProjectSource::GitHub(git_params) => {
    //             let session = self
    //                 .active_profile
    //                 .account(&git_params.account_id)
    //                 .await
    //                 .ok_or_join_err::<()>("account not found")?;
    //
    //             self.project_service
    //                 .clone_project(
    //                     ctx,
    //                     app_delegate,
    //                     &id,
    //                     session,
    //                     ProjectItemCloneParams {
    //                         order: params.order,
    //                         account_id: git_params.account_id.to_owned(),
    //                         repository: git_params.repository.clone(),
    //                         git_provider_type: GitProviderKind::GitHub,
    //                         branch: git_params.branch.clone(),
    //                     },
    //                 )
    //                 .await?
    //         }
    //         ImportProjectSource::GitLab(git_params) => {
    //             let session = self
    //                 .active_profile
    //                 .account(&git_params.account_id)
    //                 .await
    //                 .ok_or_join_err::<()>("account not found")?;
    //
    //             self.project_service
    //                 .clone_project(
    //                     ctx,
    //                     app_delegate,
    //                     &id,
    //                     session,
    //                     ProjectItemCloneParams {
    //                         order: params.order,
    //                         account_id: git_params.account_id.to_owned(),
    //                         repository: git_params.repository.clone(),
    //                         git_provider_type: GitProviderKind::GitLab,
    //                         branch: git_params.branch.clone(),
    //                     },
    //                 )
    //                 .await?
    //         }
    //         ImportProjectSource::Archive(archive_params) => {
    //             self.project_service
    //                 .import_archived_project(
    //                     ctx,
    //                     &id,
    //                     ProjectItemImportFromArchiveParams {
    //                         name: params.name.clone(),
    //                         order: params.order,
    //                         archive_path: archive_params.archive_path.clone(),
    //                     },
    //                 )
    //                 .await?
    //         }
    //         ImportProjectSource::Disk(external_params) => {
    //             self.project_service
    //                 .import_external_project::<R>(
    //                     ctx,
    //                     &id,
    //                     ProjectItemImportFromDiskParams {
    //                         order: params.order,
    //                         external_path: external_params.external_path.clone(),
    //                     },
    //                 )
    //                 .await?
    //         } // TODO: Support importing from other apps
    //     };
    //
    //     Ok(ImportProjectOutput {
    //         id: description.id,
    //         name: description.name,
    //         order: description.order,
    //         expanded: description.expanded,
    //         icon_path: description.icon_path,
    //         abs_path: description.internal_abs_path,
    //         external_path: description.external_path,
    //     })
    // }
}
