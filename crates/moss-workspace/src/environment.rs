use derive_more::Deref;
use futures::Stream;
use joinerror::OptionExt;
use moss_common::continue_if_err;
use moss_environment::{
    AnyEnvironment, DescribeEnvironment, Environment, ModifyEnvironmentParams,
    builder::{EnvironmentBuilder, EnvironmentLoadParams},
    constants::ENVIRONMENT_FILE_EXTENSION,
    errors::ErrorIo,
    models::types::AddVariableParams,
    storage::key_variable,
};
use moss_fs::{FileSystem, FsResultExt, RemoveOptions};
use moss_logging::session;
use moss_storage2::{KvStorage, models::primitives::StorageScope};
use rustc_hash::{FxHashMap, FxHashSet};
use sapic_base::{
    environment::types::primitives::EnvironmentId, project::types::primitives::ProjectId,
    workspace::types::primitives::WorkspaceId,
};
use sapic_core::context::AnyAsyncContext;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::{RwLock, mpsc};

use crate::{
    dirs,
    errors::ErrorNotFound,
    models::types::{EnvironmentGroup, UpdateEnvironmentGroupParams, UpdateEnvironmentParams},
    storage::{
        KEY_ACTIVE_ENVIRONMENTS, KEY_ENVIRONMENT_GROUP_PREFIX, KEY_ENVIRONMENT_PREFIX,
        KEY_EXPANDED_ENVIRONMENT_GROUPS, key_environment, key_environment_group_order,
        key_environment_order,
    },
};

const GLOBAL_ACTIVE_ENVIRONMENT_KEY: &'static str = "";

pub struct ActivateEnvironmentItemParams {
    pub environment_id: EnvironmentId,
}

pub struct CreateEnvironmentItemParams {
    pub project_id: Option<ProjectId>,
    pub name: String,
    pub order: isize,
    pub color: Option<String>,
    pub variables: Vec<AddVariableParams>,
}

#[derive(Clone, Deref)]
struct EnvironmentItem {
    pub id: EnvironmentId,
    pub project_id: Option<Arc<String>>,
    pub order: Option<isize>,

    #[deref]
    pub handle: Arc<Environment>,
}

pub struct EnvironmentItemDescription {
    pub id: EnvironmentId,
    pub project_id: Option<Arc<String>>,
    pub is_active: bool,
    pub display_name: String,
    pub order: Option<isize>,
    pub color: Option<String>,
    pub abs_path: Arc<Path>,
    pub total_variables: usize,
}

type EnvironmentMap = HashMap<EnvironmentId, EnvironmentItem>;

struct ServiceState {
    environments: EnvironmentMap,
    active_environments: HashMap<Arc<String>, EnvironmentId>,
    groups: FxHashSet<Arc<String>>,
    expanded_groups: HashSet<Arc<String>>,
    sources: FxHashMap<Arc<String>, PathBuf>,
}

pub struct EnvironmentService {
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
    state: Arc<RwLock<ServiceState>>,
    storage: Arc<dyn KvStorage>,
    workspace_id: WorkspaceId,
}

impl EnvironmentService {
    /// `abs_path` is the absolute path to the workspace directory
    pub async fn new(
        abs_path: &Path,
        fs: Arc<dyn FileSystem>,
        storage: Arc<dyn KvStorage>,
        workspace_id: WorkspaceId,
        sources: FxHashMap<Arc<String>, PathBuf>,
    ) -> joinerror::Result<Self> {
        let abs_path = abs_path.join(dirs::ENVIRONMENTS_DIR);
        let state = Arc::new(RwLock::new(ServiceState {
            environments: HashMap::new(),
            active_environments: HashMap::new(),
            groups: FxHashSet::default(),
            expanded_groups: HashSet::new(),
            sources,
        }));

        Ok(Self {
            fs,
            abs_path,
            state,
            storage,
            workspace_id,
        })
    }

    pub(crate) async fn add_source(&self, id: Arc<String>, abs_path: PathBuf) {
        let mut state = self.state.write().await;
        state.sources.insert(id, abs_path);
    }

    pub(crate) async fn remove_source(&self, id: &Arc<String>) {
        let mut state_lock = self.state.write().await;
        state_lock.sources.remove(id);

        state_lock.expanded_groups.remove(id);
        state_lock.groups.remove(id);

        // TODO: remove from the db
    }

    pub async fn update_environment_group(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: UpdateEnvironmentGroupParams,
    ) -> joinerror::Result<()> {
        let mut state = self.state.write().await;

        let mut batch_input = vec![];

        let project_id = params.project_id;
        let group_order_key = key_environment_group_order(&project_id);

        if let Some(expanded) = params.expanded {
            if expanded {
                state.expanded_groups.insert(project_id.inner());
            } else {
                state.expanded_groups.remove(&project_id.inner());
            }

            batch_input.push((
                KEY_EXPANDED_ENVIRONMENT_GROUPS,
                serde_json::to_value(state.expanded_groups.clone())?,
            ));
        }

        if let Some(order) = params.order {
            batch_input.push((&group_order_key, serde_json::to_value(order)?));
        }

        if batch_input.is_empty() {
            return Ok(());
        }

        if let Err(e) = self
            .storage
            .put_batch(
                ctx,
                StorageScope::Workspace(self.workspace_id.inner()),
                &batch_input,
            )
            .await
        {
            session::warn!(format!(
                "failed to update environment group `{}`: {}",
                project_id, e
            ));
        }

        Ok(())
    }

    pub async fn environment(&self, id: &EnvironmentId) -> Option<Arc<Environment>> {
        let state = self.state.read().await;
        state.environments.get(id).map(|item| item.handle.clone())
    }

    pub async fn list_environment_groups(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<EnvironmentGroup>> {
        let expanded_groups_result = self
            .storage
            .get(
                ctx,
                StorageScope::Workspace(self.workspace_id.inner()),
                KEY_EXPANDED_ENVIRONMENT_GROUPS,
            )
            .await;

        let expanded_groups: HashSet<_> = match expanded_groups_result {
            Ok(Some(expanded_groups)) => {
                serde_json::from_value(expanded_groups).unwrap_or_default()
            }
            Ok(None) => HashSet::new(),
            Err(e) => {
                session::warn!(format!(
                    "failed to get expanded environment groups from the database: {}",
                    e
                ));
                HashSet::new()
            }
        };

        let mut state = self.state.write().await;
        state.expanded_groups = expanded_groups;

        let metadata = self
            .storage
            .get_batch_by_prefix(
                ctx,
                StorageScope::Workspace(self.workspace_id.inner()),
                KEY_ENVIRONMENT_GROUP_PREFIX,
            )
            .await
            .unwrap_or_else(|e| {
                session::warn!(format!("failed to get environment group metadata: {}", e));
                Vec::new()
            })
            .into_iter()
            .collect::<FxHashMap<_, _>>();

        let mut groups = Vec::with_capacity(state.groups.len());

        for group_id in state.groups.iter() {
            let group_order_key = key_environment_group_order(&ProjectId::from(group_id.clone()));

            let order: Option<isize> = metadata
                .get(&group_order_key)
                .and_then(|v| serde_json::from_value(v.clone()).ok());

            groups.push(EnvironmentGroup {
                project_id: group_id.clone(),
                expanded: state.expanded_groups.contains(group_id),
                order,
            });
        }

        Ok(groups)
    }

    pub async fn list_environments(
        &self,
        ctx: Arc<dyn AnyAsyncContext>,
    ) -> Pin<Box<dyn Stream<Item = EnvironmentItemDescription> + Send + '_>> {
        let state_clone = self.state.clone();
        let storage = self.storage.clone();
        let sources_clone = state_clone.read().await.sources.clone();

        Box::pin(async_stream::stream! {
            let ctx_clone = ctx.clone();
            let storage_clone = storage.clone();
            let (tx, mut rx) = mpsc::unbounded_channel::<(EnvironmentItem, DescribeEnvironment)>();
            let scanner = EnvironmentSourceScanner {
                fs: self.fs.clone(),
                sources: sources_clone,
                storage: storage_clone.clone(),
                workspace_id: self.workspace_id.clone(),
                tx,
            };

            let scan_task = {
                tokio::spawn(async move {
                    let ctx_clone = ctx_clone.clone();
                    if let Err(e) = scanner.scan(ctx_clone.as_ref()).await {
                        session::error!(format!("environment scan failed: {}", e));
                    }
                })
            };

            let active_environments_result = storage.get(
                ctx.as_ref(),
                StorageScope::Workspace(self.workspace_id.inner()),
                KEY_ACTIVE_ENVIRONMENTS
            ).await;

            let active_environments: HashMap<_, _> = match active_environments_result {
                Ok(Some(active_environments)) => {
                    serde_json::from_value(active_environments).unwrap_or_default()
                },
                Ok(None) => HashMap::new(),
                Err(e) => {
                    session::warn!(format!("failed to get activated environments from the db: {}", e));
                    HashMap::new()
                }
            };

            let mut state_lock = state_clone.write().await;
            (*state_lock).active_environments = active_environments;

            // Ensure that environments and groups that are not found during the scan will be removed from map
            (*state_lock).environments = HashMap::new();
            (*state_lock).groups = FxHashSet::default();

            while let Some((item, desc)) = rx.recv().await {
                let id = item.id.clone();
                let group_key = item.project_id.clone().unwrap_or_else(|| {
                    GLOBAL_ACTIVE_ENVIRONMENT_KEY.to_string().into()
                });

                let desc = EnvironmentItemDescription {
                    id: id.clone(),
                    project_id: item.project_id.clone(),
                    is_active: state_lock.active_environments.get(&group_key) == Some(&desc.id),
                    display_name: desc.name,
                    order: item.order.clone(),
                    color: desc.color,
                    abs_path: desc.abs_path,
                    total_variables: desc.variables.len(),
                };

                {
                    let group_id = item.project_id.clone();
                    state_lock.environments.insert(id, item);

                    if let Some(group_id) = group_id {
                        state_lock.groups.insert(group_id);
                    }
                }

                yield desc;
            }


            let _ = scan_task.await;
        })
    }

    pub async fn update_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: UpdateEnvironmentParams,
    ) -> joinerror::Result<()> {
        let mut state = self.state.write().await;
        let environment_item = state
            .environments
            .get_mut(&params.id)
            .ok_or_join_err_with::<ErrorNotFound>(|| {
                format!("environment item not found: {}", params.id)
            })?;

        environment_item
            .modify(
                ctx,
                ModifyEnvironmentParams {
                    name: params.name.clone(),
                    color: params.color.clone(),
                    vars_to_add: params.vars_to_add,
                    vars_to_update: params.vars_to_update,
                    vars_to_delete: params.vars_to_delete,
                },
            )
            .await?;

        if let Some(order) = params.order {
            environment_item.order = Some(order);
            if let Err(e) = self
                .storage
                .put(
                    ctx,
                    StorageScope::Workspace(self.workspace_id.inner()),
                    &key_environment_order(&params.id),
                    serde_json::to_value(order)?,
                )
                .await
            {
                session::warn!(format!("failed to put environment order in the db: {}", e));
            }
        }

        Ok(())
    }

    pub async fn create_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: CreateEnvironmentItemParams,
    ) -> joinerror::Result<EnvironmentItemDescription> {
        let abs_path = if let Some(collection_id) = params.project_id.clone() {
            let state_lock = self.state.read().await;
            let collection_id_inner = collection_id.inner();

            let source = state_lock
                .sources
                .get(&collection_id_inner)
                .ok_or_join_err_with::<ErrorNotFound>(|| {
                    format!("source not found for collection {}", collection_id_inner)
                })?;

            source.clone()
        } else {
            self.abs_path.clone()
        };

        // FIXME: Switch to new db when refactoring moss-environment
        let environment = EnvironmentBuilder::new(
            self.workspace_id.inner(),
            self.fs.clone(),
            self.storage.clone(),
        )
        .create(
            ctx,
            moss_environment::builder::CreateEnvironmentParams {
                name: params.name.clone(),
                abs_path: &abs_path,
                color: params.color,
                variables: params.variables,
            },
        )
        .await?;

        let abs_path = environment.abs_path().await;
        let project_id_inner = params.project_id.as_ref().map(|id| id.inner());
        let desc = environment.describe(ctx).await?;

        let mut state = self.state.write().await;
        state.environments.insert(
            desc.id.clone(),
            EnvironmentItem {
                id: desc.id.clone(),
                project_id: project_id_inner.clone(),
                order: Some(params.order),
                handle: Arc::new(environment),
            },
        );

        let output = EnvironmentItemDescription {
            id: desc.id.clone(),
            project_id: project_id_inner,
            // FIXME: Should we provide option to activate an environment upon creation?
            is_active: false,
            display_name: params.name.clone(),
            order: Some(params.order),
            color: desc.color,
            abs_path,
            total_variables: desc.variables.len(),
        };

        if let Err(e) = self
            .storage
            .put(
                ctx,
                StorageScope::Workspace(self.workspace_id.inner()),
                &key_environment_order(&desc.id),
                serde_json::to_value(params.order)?,
            )
            .await
        {
            session::warn!(format!("failed to put environment order in the db: {}", e));
        }

        let project_id = if let Some(project_id) = &params.project_id {
            project_id.inner()
        } else {
            return Ok(output);
        };

        // Create a new environment group if it doesn't exist
        if state.groups.contains(&project_id) {
            return Ok(output);
        }

        // FIXME: the order should be the current max group order + 1
        let group_order = (state.groups.len() + 1) as isize;

        state.groups.insert(project_id.clone());
        state.expanded_groups.insert(project_id.clone());

        {
            let group_order_key = key_environment_group_order(&project_id.into());
            let batch_input = vec![
                (group_order_key.as_str(), serde_json::to_value(group_order)?),
                (
                    KEY_EXPANDED_ENVIRONMENT_GROUPS,
                    serde_json::to_value(&state.expanded_groups)?,
                ),
            ];

            if let Err(e) = self
                .storage
                .put_batch(
                    ctx,
                    StorageScope::Workspace(self.workspace_id.inner()),
                    &batch_input,
                )
                .await
            {
                session::warn!(format!(
                    "failed to update environment groups metadata in database after creating environment: {}",
                    e
                ));
            }
        }

        Ok(output)
    }

    pub async fn delete_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<()> {
        let mut state = self.state.write().await;
        let environment = state
            .environments
            .remove(id)
            .ok_or_join_err_with::<ErrorNotFound>(|| format!("environment {} not found", id))?;

        // If the environment is currently active, deactivate it
        let env_group_key = environment
            .project_id
            .clone()
            .unwrap_or_else(|| GLOBAL_ACTIVE_ENVIRONMENT_KEY.to_string().into());

        let active_environments_updated =
            if state.active_environments.get(&env_group_key) == Some(&environment.id) {
                state.active_environments.remove(&env_group_key);
                true
            } else {
                false
            };

        let desc = environment.describe(ctx).await?;
        self.fs
            .remove_file(
                &desc.abs_path,
                RemoveOptions {
                    recursive: false,
                    ignore_if_not_exists: true,
                },
            )
            .await
            .join_err_with::<ErrorIo>(|| {
                format!(
                    "failed to remove environment file at {}",
                    desc.abs_path.display()
                )
            })?;

        // Database errors should not fail the operation
        {
            let storage_scope = StorageScope::Workspace(self.workspace_id.inner());
            // Clean all the data related to the deleted environment
            if let Err(e) = self
                .storage
                .remove_batch_by_prefix(ctx, storage_scope.clone(), &key_environment(id))
                .await
            {
                session::warn!(format!(
                    "failed to remove environment cache from the db: {}",
                    e
                ));
            }
            // Update active environments map
            if active_environments_updated {
                if let Err(e) = self
                    .storage
                    .put(
                        ctx,
                        storage_scope.clone(),
                        KEY_ACTIVE_ENVIRONMENTS,
                        serde_json::to_value(&state.active_environments)?,
                    )
                    .await
                {
                    session::warn!(format!(
                        "failed to update active environments in the database: {}",
                        e
                    ));
                }
            }

            // Remove all variables belonging to the deleted environment

            for id in desc.variables.keys() {
                if let Err(e) = self
                    .storage
                    .remove_batch_by_prefix(
                        ctx,
                        StorageScope::Workspace(self.workspace_id.inner()),
                        &key_variable(id),
                    )
                    .await
                {
                    session::warn!(format!(
                        "failed to remove variable cache from the database: {}",
                        e
                    ));
                }
            }
        }
        Ok(())
    }

    pub async fn activate_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ActivateEnvironmentItemParams,
    ) -> joinerror::Result<()> {
        let mut state = self.state.write().await;

        let environment_item = state
            .environments
            .get(&params.environment_id)
            .ok_or_join_err_with::<ErrorNotFound>(|| {
                format!("environment {} not found", params.environment_id)
            })?;

        let env_group_key = if let Some(group_id) = &environment_item.project_id {
            group_id.clone()
        } else {
            GLOBAL_ACTIVE_ENVIRONMENT_KEY.to_string().into()
        };

        state
            .active_environments
            .insert(env_group_key.clone(), params.environment_id);

        if let Err(e) = self
            .storage
            .put(
                ctx,
                StorageScope::Workspace(self.workspace_id.inner()),
                KEY_ACTIVE_ENVIRONMENTS,
                serde_json::to_value(&state.active_environments)?,
            )
            .await
        {
            session::warn!(format!(
                "failed to put active environments in the database: {}",
                e
            ));
        }

        Ok(())
    }
}

struct ScanSourceJob {
    source_id: Arc<String>,
    abs_path: PathBuf,
    tx: mpsc::UnboundedSender<(Option<Arc<String>>, Environment)>,
}

struct EnvironmentSourceScanner {
    fs: Arc<dyn FileSystem>,
    sources: FxHashMap<Arc<String>, PathBuf>,
    storage: Arc<dyn KvStorage>,
    workspace_id: WorkspaceId,
    tx: mpsc::UnboundedSender<(EnvironmentItem, DescribeEnvironment)>,
}

impl EnvironmentSourceScanner {
    /// Scans environments from all registered providers in parallel.
    ///
    /// This function implements a multi-stage scanning process:
    /// 1. Loads cached metadata from the database (orders, configurations, etc.)
    /// 2. Spawns parallel scanning tasks for each registered environment provider
    /// 3. Collects environments from all providers through a unified channel
    /// 4. Enriches each environment with cached metadata and forwards to the output channel
    async fn scan(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<()> {
        let data = self
            .storage
            .get_batch_by_prefix(
                ctx,
                StorageScope::Workspace(self.workspace_id.inner()),
                KEY_ENVIRONMENT_PREFIX,
            )
            .await
            .unwrap_or_else(|e| {
                session::warn!(format!(
                    "failed to get environment cache from the database: {}",
                    e
                ));
                Vec::new()
            })
            .into_iter()
            .collect::<HashMap<_, _>>();

        let (provider_tx, mut provider_rx) =
            mpsc::unbounded_channel::<(Option<Arc<String>>, Environment)>();

        let mut scan_tasks = Vec::new();

        let workspace_id = self.workspace_id.clone();
        for (source_id, source) in self.sources.iter() {
            let provider_tx_clone = provider_tx.clone();
            let source_id_clone = source_id.clone();
            let source_clone = source.clone();
            let fs_clone = self.fs.clone();
            let storage_clone = self.storage.clone();
            let workspace_id_clone = workspace_id.clone();

            let task = tokio::spawn(async move {
                let scan_task = tokio::spawn({
                    let source_id_for_scan = source_id_clone.clone();
                    let source_for_scan = source_clone.clone();
                    let fs_for_scan = fs_clone.clone();
                    let storage_for_scan = storage_clone.clone();
                    let workspace_id_for_scan = workspace_id_clone.clone();

                    async move {
                        if let Err(e) = scan_source(
                            workspace_id_for_scan,
                            fs_for_scan,
                            storage_for_scan,
                            ScanSourceJob {
                                source_id: source_id_for_scan.clone(),
                                abs_path: source_for_scan,
                                tx: provider_tx_clone,
                            },
                        )
                        .await
                        {
                            session::error!(format!(
                                "provider `{}` scan failed: {}",
                                source_id_for_scan, e
                            ));
                        }
                    }
                });

                // Wait for scan to complete
                let _ = scan_task.await;
            });

            scan_tasks.push(task);
        }

        // Drop the original sender so the channel closes when all tasks complete
        drop(provider_tx);

        while let Some((collection_id, environment)) = provider_rx.recv().await {
            let desc = match environment.describe(ctx).await {
                Ok(desc) => desc,
                Err(e) => {
                    session::error!(format!("failed to describe environment: {}", e));
                    continue;
                }
            };

            let order: Option<isize> = if let Some(order) = data
                .get(&key_environment_order(&desc.id))
                .and_then(|v| serde_json::from_value(v.clone()).ok())
            {
                order
            } else {
                session::warn!(format!("no order found for environment `{}`", desc.id));
                None
            };

            let environment_item = EnvironmentItem {
                id: desc.id.clone(),
                project_id: collection_id,
                order,
                handle: Arc::new(environment),
            };

            if self.tx.send((environment_item, desc)).is_err() {
                break; // Receiver dropped, stop processing
            }
        }

        for task in scan_tasks {
            let _ = task.await;
        }

        Ok(())
    }
}

async fn scan_source(
    workspace_id: WorkspaceId,
    fs: Arc<dyn FileSystem>,
    storage: Arc<dyn KvStorage>,
    job: ScanSourceJob,
) -> joinerror::Result<()> {
    session::trace!(
        "scanning environment provider: {}",
        job.abs_path.to_string_lossy().to_string()
    );
    let mut read_dir = fs.read_dir(&job.abs_path).await.map_err(|err| {
        joinerror::Error::new::<()>(format!(
            "failed to read directory {} : {}",
            job.abs_path.display(),
            err
        ))
    })?;

    while let Some(entry) = read_dir.next_entry().await? {
        if entry.file_type().await?.is_dir() {
            continue;
        }

        if !entry
            .file_name()
            .to_string_lossy()
            .ends_with(ENVIRONMENT_FILE_EXTENSION)
        {
            continue;
        }

        let maybe_environment =
            EnvironmentBuilder::new(workspace_id.inner(), fs.clone(), storage.clone())
                .load(EnvironmentLoadParams {
                    abs_path: entry.path(),
                })
                .await;
        let environment = continue_if_err!(maybe_environment, |err| {
            session::error!(format!(
                "failed to load environment `{}`: {}",
                entry.path().display(),
                err
            ));
        });

        let collection_id = if job.source_id.as_str() == "" {
            None
        } else {
            Some(job.source_id.clone())
        };

        job.tx.send((collection_id, environment)).ok();
    }

    Ok(())
}
