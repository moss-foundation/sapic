use async_trait::async_trait;
use joinerror::ResultExt;
use json_patch::{PatchOperation, RemoveOperation, ReplaceOperation};
use jsonptr::PointerBuf;
use moss_bindingutils::primitives::ChangeString;
use moss_edit::json::{EditOptions, JsonEdit};
use moss_fs::{CreateOptions, FileSystem};
use sapic_base::project::{
    config::CONFIG_FILE_NAME, manifest::MANIFEST_FILE_NAME, types::primitives::ProjectId,
};
use sapic_core::context::AnyAsyncContext;
use sapic_system::project::{ProjectConfigEditParams, ProjectEditBackend, ProjectEditParams};
use serde_json::Value as JsonValue;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

pub struct ProjectFsEditBackend {
    projects_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    edits: RwLock<JsonEdit>,
}

impl ProjectFsEditBackend {
    pub fn new(fs: Arc<dyn FileSystem>, projects_dir: PathBuf) -> Arc<Self> {
        Arc::new(Self {
            projects_dir,
            fs,
            edits: RwLock::new(JsonEdit::new()),
        })
    }
}

#[async_trait]
impl ProjectEditBackend for ProjectFsEditBackend {
    async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: ProjectEditParams,
    ) -> joinerror::Result<()> {
        let mut patches = Vec::new();

        if let Some(new_name) = params.name {
            patches.push((
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked("/name") },
                    value: JsonValue::String(new_name),
                }),
                EditOptions {
                    create_missing_segments: false,
                    ignore_if_not_exists: false,
                },
            ));
        }

        if patches.is_empty() {
            return Ok(());
        }

        let abs_path = self
            .projects_dir
            .join(id.to_string())
            .join(MANIFEST_FILE_NAME);
        let rdr = self
            .fs
            .open_file(ctx, &abs_path)
            .await
            .join_err_with::<()>(|| format!("failed to open file: {}", abs_path.display()))?;

        let mut value: JsonValue =
            serde_json::from_reader(rdr).join_err::<()>("failed to parse json")?;

        let mut edits_lock = self.edits.write().await;
        edits_lock
            .apply(&mut value, &patches)
            .join_err::<()>("failed to apply patches")?;

        let content = serde_json::to_string(&value).join_err::<()>("failed to serialize json")?;

        self.fs
            .create_file_with(
                ctx,
                &abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<()>(|| format!("failed to write file: {}", abs_path.display()))?;

        Ok(())
    }

    async fn edit_config(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: ProjectConfigEditParams,
    ) -> joinerror::Result<()> {
        let mut patches = Vec::new();

        if let Some(archived) = params.archived {
            patches.push((
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked("/archived") },
                    value: JsonValue::Bool(archived),
                }),
                EditOptions {
                    create_missing_segments: false,
                    ignore_if_not_exists: false,
                },
            ));
        }

        if patches.is_empty() {
            return Ok(());
        }

        let abs_path = self
            .projects_dir
            .join(id.to_string())
            .join(CONFIG_FILE_NAME);
        let rdr = self
            .fs
            .open_file(ctx, &abs_path)
            .await
            .join_err_with::<()>(|| format!("failed to open file: {}", abs_path.display()))?;

        let mut value: JsonValue =
            serde_json::from_reader(rdr).join_err::<()>("failed to parse json")?;

        let mut edits_lock = self.edits.write().await;
        edits_lock
            .apply(&mut value, &patches)
            .join_err::<()>("failed to apply patches")?;

        let content = serde_json::to_string(&value).join_err::<()>("failed to serialize json")?;

        self.fs
            .create_file_with(
                ctx,
                &abs_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await
            .join_err_with::<()>(|| format!("failed to write file: {}", abs_path.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use moss_fs::RealFileSystem;
    use moss_testutils::random_name::random_string;
    use sapic_core::context::ArcContext;
    use sapic_system::project::{CreateProjectParams, ProjectServiceFs as ProjectServiceFsPort};

    use crate::project::project_service_fs::ProjectServiceFs;

    use super::*;

    async fn setup_project_edit_test() -> (
        ArcContext,
        Arc<ProjectServiceFs>,
        Arc<ProjectFsEditBackend>,
        PathBuf,
    ) {
        let ctx = ArcContext::background();
        let test_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("data")
            .join(random_string(10));
        let tmp_path = test_path.join("tmp");
        let projects_dir = tmp_path.join("projects");

        tokio::fs::create_dir_all(&tmp_path).await.unwrap();
        tokio::fs::create_dir_all(&projects_dir).await.unwrap();

        let fs = Arc::new(RealFileSystem::new(&tmp_path));
        let project_fs = ProjectServiceFs::new(fs.clone(), projects_dir.clone());
        let edit = ProjectFsEditBackend::new(fs.clone(), projects_dir.clone());
        (ctx, project_fs, edit, test_path)
    }

    #[tokio::test]
    async fn test_edit_rename() {
        let (ctx, project_fs, edit, test_path) = setup_project_edit_test().await;

        let id = ProjectId::new();
        let old_name = random_string(10);
        let new_name = random_string(10);

        project_fs
            .create_project(
                &ctx,
                &id,
                CreateProjectParams {
                    name: Some(old_name.clone()),
                    external_abs_path: None,
                    git_params: None,
                    icon_path: None,
                },
            )
            .await
            .unwrap();

        edit.edit(
            &ctx,
            &id,
            ProjectEditParams {
                name: Some(new_name.clone()),
            },
        )
        .await
        .unwrap();

        let projects = project_fs.lookup_projects(&ctx).await.unwrap();
        assert_eq!(projects[0].manifest.name, new_name);

        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_edit_nonexistent() {
        let (ctx, _project_fs, edit, test_path) = setup_project_edit_test().await;

        let id = ProjectId::new();
        let new_name = random_string(10);
        let result = edit
            .edit(
                &ctx,
                &id,
                ProjectEditParams {
                    name: Some(new_name.clone()),
                },
            )
            .await;

        assert!(result.is_err());

        tokio::fs::remove_dir_all(test_path).await.unwrap();
    }
}
