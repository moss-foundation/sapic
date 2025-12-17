use chrono::{DateTime, Utc};
use git2::{BranchType, IndexAddOption, Signature};
use joinerror::{Error, OptionExt, ResultExt};
use json_patch::{PatchOperation, ReplaceOperation, jsonptr::PointerBuf};
use moss_applib::AppRuntime;
use moss_bindingutils::primitives::{ChangePath, ChangeString};
use moss_edit::json::EditOptions;
use moss_fs::{CreateOptions, FileSystem};
use moss_git::{repository::Repository, url::GitUrl};
use moss_storage2::KvStorage;
use moss_text::sanitized::sanitize;
use sapic_base::{
    project::{
        manifest::{MANIFEST_FILE_NAME, ManifestVcs, ProjectManifest},
        types::primitives::ProjectId,
    },
    user::types::primitives::AccountId,
};
use sapic_core::{
    context::AnyAsyncContext,
    subscription::{Event, EventEmitter, EventMarker},
};
use sapic_system::ports::GitProviderKind;
use serde_json::Value as JsonValue;
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::sync::OnceCell;

use crate::{
    config::{CONFIG_FILE_NAME, ConfigFile},
    dirs,
    edit::ProjectEdit,
    git::GitClient,
    set_icon::SetIconService,
    vcs::{ProjectVcs, Vcs},
    worktree::Worktree,
};

const ARCHIVE_EXCLUDED_ENTRIES: [&'static str; 3] = ["config.json", "state.db", ".git"];

#[derive(Debug, Clone)]
pub enum OnDidChangeEvent {
    Toggled(bool),
}

impl EventMarker for OnDidChangeEvent {}

pub struct ProjectModifyParams {
    pub name: Option<String>,
    pub repository: Option<ChangeString>,
    pub icon_path: Option<ChangePath>,
}

pub struct ProjectConfigModifyParams {
    pub archived: Option<bool>,
    pub account_id: Option<AccountId>,
}

pub struct ProjectDetails {
    pub name: String,
    pub created_at: String, // File created time
    pub vcs: Option<VcsDetails>,
    pub archived: bool,
    pub account_id: Option<AccountId>,
}

pub struct VcsDetails {
    pub kind: GitProviderKind,
    pub repository: String,
}

impl From<ManifestVcs> for VcsDetails {
    fn from(value: ManifestVcs) -> Self {
        match value {
            ManifestVcs::GitHub { repository } => Self {
                kind: GitProviderKind::GitHub,
                repository,
            },
            ManifestVcs::GitLab { repository } => Self {
                kind: GitProviderKind::GitLab,
                repository,
            },
        }
    }
}

pub struct Project {
    pub(super) id: ProjectId,
    #[allow(dead_code)]
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) storage: Arc<dyn KvStorage>,
    pub(super) internal_abs_path: Arc<Path>,
    pub(super) external_abs_path: Option<Arc<Path>>,
    pub(super) edit: ProjectEdit,
    pub(super) worktree: OnceCell<Arc<Worktree>>,
    pub(super) set_icon_service: SetIconService,
    pub(super) vcs: OnceCell<Vcs>,
    pub(super) on_did_change: EventEmitter<OnDidChangeEvent>,
    pub(super) archived: AtomicBool,
}

unsafe impl Send for Project {}

#[rustfmt::skip]
impl Project {
    pub fn on_did_change(&self) -> Event<OnDidChangeEvent> { self.on_did_change.event() }
}

impl Project {
    pub(crate) async fn worktree(&self) -> &Arc<Worktree> {
        self.worktree
            .get_or_init(|| async {
                Arc::new(Worktree::new(
                    self.storage.clone(),
                    self.id.clone(),
                    self.abs_path(),
                    self.fs.clone(),
                ))
            })
            .await
    }

    pub fn id(&self) -> &ProjectId {
        &self.id
    }

    pub fn is_archived(&self) -> bool {
        self.archived.load(Ordering::SeqCst)
    }

    pub fn internal_abs_path(&self) -> &Arc<Path> {
        &self.internal_abs_path
    }

    pub fn external_abs_path(&self) -> Option<&Arc<Path>> {
        self.external_abs_path.as_ref()
    }

    // Actual path where resources, manifest and other committable files are stored
    // It's the internal_abs_path for normal project, external_abs_path for external project
    pub fn abs_path(&self) -> Arc<Path> {
        self.external_abs_path
            .clone()
            .unwrap_or(self.internal_abs_path.clone())
    }

    pub fn icon_path(&self) -> Option<PathBuf> {
        self.set_icon_service.icon_path()
    }

    pub fn environments_path(&self) -> PathBuf {
        self.abs_path().join(dirs::ENVIRONMENTS_DIR)
    }

    pub fn vcs<R: AppRuntime>(&self) -> Option<&dyn ProjectVcs<R>> {
        self.vcs.get().map(|vcs| vcs as &dyn ProjectVcs<R>)
    }
    pub async fn init_vcs(
        &self,
        ctx: &dyn AnyAsyncContext,
        client: GitClient,
        url: GitUrl,
        default_branch: String,
    ) -> joinerror::Result<()> {
        // HACK: It's impossible to set the config after git operations inside this method
        // Since it will trigger spurious git2 type errors
        {
            let account_id = client.account_id();
            self.modify_config(
                ctx,
                ProjectConfigModifyParams {
                    archived: None,
                    account_id: Some(account_id),
                },
            )
            .await?;
        }
        let (access_token, username) = match &client {
            GitClient::GitHub { account, .. } => {
                (account.session().token(ctx).await?, account.username())
            }
            GitClient::GitLab { account, .. } => {
                (account.session().token(ctx).await?, account.username())
            }
        };

        let mut cb = git2::RemoteCallbacks::new();
        let username_clone = username.clone();
        let access_token_clone = access_token.clone();
        cb.credentials(move |_url, username_from_url, _allowed| {
            git2::Cred::userpass_plaintext(
                username_from_url.unwrap_or(&username_clone),
                &access_token_clone,
            )
        });

        let repository = Repository::init(&self.abs_path())?;
        repository.add_remote(None, &url.to_string_with_suffix()?)?;
        repository.fetch(None, cb)?;

        let remote_branches = repository.list_branches(Some(BranchType::Remote))?;

        // We will push a default branch to the remote, if no remote branches exist
        // TODO: Support connecting with a remote repo that already has branches?
        if !remote_branches.is_empty() {
            return Err(Error::new::<()>(
                "connecting with a non-empty repo is unimplemented",
            ));
        }

        repository.add_all(["."].iter(), IndexAddOption::DEFAULT)?;
        let author = Signature::now(
            &username,
            // FIXME: This is a temporary solution to avoid the error
            format!("{}@git.noreply.com", username).as_str(),
        )
        .map_err(|e| {
            Error::new::<()>(format!(
                "failed to generate commit signature: {}",
                e.to_string()
            ))
        })?;
        repository.commit("Initial Commit", author)?;

        let old_default_branch_name = repository
            .list_branches(Some(BranchType::Local))?
            .first()
            .cloned()
            .ok_or_join_err::<()>("no local branch exists")?;
        repository.rename_branch(&old_default_branch_name, &default_branch, false)?;

        // We don't want to push during integration tests
        #[cfg(not(any(test, feature = "integration-tests")))]
        {
            let mut cb = git2::RemoteCallbacks::new();
            let username_clone = username.clone();
            cb.credentials(move |_url, username_from_url, _allowed| {
                git2::Cred::userpass_plaintext(
                    username_from_url.unwrap_or(&username_clone),
                    &access_token,
                )
            });
            repository.push(None, Some(&default_branch), Some(&default_branch), true, cb)?;
        }

        self.vcs
            .set(Vcs::new(url, repository, client))
            .map_err(|e| Error::new::<()>(e.to_string()))
            .join_err::<()>("failed to set git service")?;

        Ok(())
    }

    pub async fn load_vcs(
        &self,
        ctx: &dyn AnyAsyncContext,
        client: GitClient,
    ) -> joinerror::Result<()> {
        let abs_path = self.abs_path();
        let repository = Repository::open(&abs_path)?;

        let url = {
            let manifest_path = abs_path.join(MANIFEST_FILE_NAME);
            let rdr = self
                .fs
                .open_file(ctx, &manifest_path)
                .await
                .join_err_with::<()>(|| {
                    format!("failed to open manifest file: {}", manifest_path.display())
                })?;
            let manifest: ProjectManifest =
                serde_json::from_reader(rdr).join_err_with::<()>(|| {
                    format!("failed to parse manifest file: {}", manifest_path.display())
                })?;

            let url = manifest
                .vcs
                .map(|vcs| vcs.repository().to_string())
                .ok_or_join_err::<()>("no repository in manifest")?;

            GitUrl::parse(&url)?
        }; // HACK: This is a hack to get the URL from the manifest file. We should come up with a better solution.

        self.vcs
            .set(Vcs::new(url, repository, client))
            .map_err(|e| Error::new::<()>(e.to_string()))
            .join_err::<()>("failed to set git service")?;

        Ok(())
    }

    pub async fn details(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<ProjectDetails> {
        let manifest_path = self.abs_path().join(MANIFEST_FILE_NAME);
        let rdr = self
            .fs
            .open_file(ctx, &manifest_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to open manifest file: {}", manifest_path.display())
            })?;
        let manifest: ProjectManifest = serde_json::from_reader(rdr).join_err_with::<()>(|| {
            format!("failed to parse manifest file: {}", manifest_path.display())
        })?;

        let config_path = self.internal_abs_path.join(CONFIG_FILE_NAME);

        let rdr = self
            .fs
            .open_file(ctx, &config_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to open config file: {}", config_path.display())
            })?;

        let config: ConfigFile = serde_json::from_reader(rdr).join_err_with::<()>(|| {
            format!("failed to parse config file: {}", config_path.display())
        })?;

        let created_at: DateTime<Utc> = std::fs::metadata(&manifest_path)?.created()?.into();

        Ok(ProjectDetails {
            name: manifest.name,
            created_at: created_at.to_rfc3339(),
            vcs: manifest.vcs.map(|vcs| vcs.into()),
            archived: config.archived,
            account_id: config.account_id,
        })
    }

    pub async fn modify(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ProjectModifyParams,
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

        match params.icon_path {
            None => {}
            Some(ChangePath::Update(new_icon_path)) => {
                self.set_icon_service.set_icon(&new_icon_path)?;
            }
            Some(ChangePath::Remove) => {
                self.set_icon_service.remove_icon(ctx).await?;
            }
        }
        self.edit
            .edit(ctx, &patches)
            .await
            .join_err::<()>("failed to edit project")?;

        Ok(())
    }

    pub(crate) async fn modify_config(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ProjectConfigModifyParams,
    ) -> joinerror::Result<()> {
        let config_path = self.internal_abs_path.join(CONFIG_FILE_NAME);
        let rdr = self
            .fs
            .open_file(ctx, &config_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to open config file: {}", config_path.display())
            })?;

        let mut config: ConfigFile = serde_json::from_reader(rdr).join_err_with::<()>(|| {
            format!("failed to parse config file: {}", config_path.display())
        })?;

        match params.account_id {
            None => {}
            Some(account_id) => {
                config.account_id = Some(account_id);
            }
        }

        match params.archived {
            None => {}
            Some(archived) => {
                config.archived = archived;
            }
        }

        self.fs
            .create_file_with(
                ctx,
                &config_path,
                serde_json::to_string(&config)?.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await?;
        Ok(())
    }

    pub async fn dispose(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<()> {
        if let Some(vcs) = self.vcs.get() {
            vcs.dispose(ctx, self.fs.clone()).await?;
        }

        Ok(())
    }

    pub async fn archive(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<()> {
        let updated = self
            .archived
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |archived| {
                if archived { None } else { Some(true) }
            })
            .is_ok();

        if !updated {
            return Ok(());
        }
        self.archived.store(true, Ordering::Relaxed);

        self.modify_config(
            ctx,
            ProjectConfigModifyParams {
                archived: Some(true),
                account_id: None,
            },
        )
        .await?;
        // TODO: Dropping worktree and vcs?
        // Right now it's impossible since OnceCell requires &mut self

        Ok(())
    }

    pub async fn unarchive(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<()> {
        let updated = self
            .archived
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |archived| {
                if !archived { None } else { Some(false) }
            })
            .is_ok();

        if !updated {
            return Ok(());
        }

        self.modify_config(
            ctx,
            ProjectConfigModifyParams {
                archived: Some(false),
                account_id: None,
            },
        )
        .await?;

        let _ = self
            .worktree
            .get_or_init(|| async {
                Arc::new(Worktree::new(
                    self.storage.clone(),
                    self.id.clone(),
                    self.internal_abs_path.clone(),
                    self.fs.clone(),
                ))
            })
            .await;

        // TODO: Read account info from config and reload vcs

        Ok(())
    }

    /// Export the project to {destination}/{project_name}.zip
    /// Returns the path to the output archive file
    pub async fn export_archive(
        &self,
        ctx: &dyn AnyAsyncContext,
        destination: &Path,
    ) -> joinerror::Result<PathBuf> {
        // If the output is inside the collection folder, it will also be bundled, which we don't want
        let abs_path = self.abs_path();

        if destination.starts_with(&abs_path) {
            return Err(Error::new::<()>(
                "cannot export archive file into the project folder",
            ));
        }
        // Project name can contain special chars that need sanitizing
        let raw_name = format!("{}", self.details(ctx).await?.name);
        let sanitized_name = sanitize(&raw_name);
        let archive_path = destination.join(format!("{}.zip", sanitized_name));

        self.fs
            .zip(ctx, &abs_path, &archive_path, &ARCHIVE_EXCLUDED_ENTRIES)
            .await?;

        Ok(archive_path)
    }
}
