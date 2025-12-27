use moss_applib::AppRuntime;
use sapic_base::other::GitProviderKind;
use sapic_ipc::{
    ValidationResultExt,
    contracts::main::project::{ImportProjectInput, ImportProjectOutput, ImportProjectSource},
};
use validator::Validate;

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn import_project(
        &self,
        ctx: &R::AsyncContext,
        input: &ImportProjectInput,
    ) -> joinerror::Result<ImportProjectOutput> {
        input.validate().join_err_bare()?;

        let workspace = self.workspace.load();
        let params = &input.inner;

        let project = match &params.source {
            ImportProjectSource::GitHub(git_params) => {
                workspace
                    .clone_project(
                        ctx,
                        &git_params.account_id,
                        GitProviderKind::GitHub,
                        &git_params.repository,
                        git_params.branch.clone(),
                    )
                    .await?
            }
            ImportProjectSource::GitLab(git_params) => {
                workspace
                    .clone_project(
                        ctx,
                        &git_params.account_id,
                        GitProviderKind::GitLab,
                        &git_params.repository,
                        git_params.branch.clone(),
                    )
                    .await?
            }
            ImportProjectSource::Archive(archive_params) => {
                workspace
                    .import_archived_project(ctx, archive_params)
                    .await?
            }
            ImportProjectSource::Disk(external_params) => {
                workspace
                    .import_external_project(ctx, external_params)
                    .await?
            }
        };

        let details = project.handle.details(ctx).await?;
        Ok(ImportProjectOutput {
            id: project.id,
            name: details.name,
            order: project.order,
            expanded: true, // HACK: hardcoded value
            icon_path: project.handle.icon_path(),
            abs_path: project.handle.internal_abs_path().to_owned(),
            external_path: project.handle.external_abs_path().map(|p| p.to_path_buf()),
        })
    }
}
