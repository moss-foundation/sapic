use derive_more::Deref;
use futures::Stream;
use joinerror::OptionExt;
use moss_applib::AppRuntime;
use moss_common::continue_if_err;
use moss_db::primitives::AnyValue;
use moss_environment::{
    AnyEnvironment, DescribeEnvironment, Environment, ModifyEnvironmentParams,
    builder::{EnvironmentBuilder, EnvironmentLoadParams},
    constants::ENVIRONMENT_FILE_EXTENSION,
    errors::ErrorIo,
    models::{primitives::EnvironmentId, types::AddVariableParams},
    segments::{SEGKEY_VARIABLE_LOCALVALUE, SEGKEY_VARIABLE_ORDER},
};
use moss_fs::{FileSystem, FsResultExt, RemoveOptions};
use moss_logging::session;
use moss_storage::{
    WorkspaceStorage,
    common::VariableStore,
    primitives::segkey::SegKeyBuf,
    storage::operations::{ListByPrefix, TransactionalRemoveByPrefix, TransactionalRemoveItem},
};
use rustc_hash::{FxHashMap, FxHashSet};
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
    models::{
        primitives::ProjectId,
        types::{EnvironmentGroup, UpdateEnvironmentGroupParams, UpdateEnvironmentParams},
    },
    services::storage_service::StorageService,
    storage::segments,
};

const GLOBAL_ACTIVE_ENVIRONMENT_KEY: &'static str = "";

pub struct ActivateEnvironmentItemParams {
    pub environment_id: EnvironmentId,
}

pub struct CreateEnvironmentItemParams {
    pub collection_id: Option<ProjectId>,
    pub name: String,
    pub order: isize,
    pub color: Option<String>,
    pub variables: Vec<AddVariableParams>,
}

#[derive(Clone, Deref)]
struct EnvironmentItem<R>
where
    R: AppRuntime,
{
    pub id: EnvironmentId,
    pub collection_id: Option<Arc<String>>,
    pub order: Option<isize>,

    #[deref]
    pub handle: Arc<Environment<R>>,
}

pub struct EnvironmentItemDescription {
    pub id: EnvironmentId,
    pub collection_id: Option<Arc<String>>,
    pub is_active: bool,
    pub display_name: String,
    pub order: Option<isize>,
    pub color: Option<String>,
    pub abs_path: Arc<Path>,
    pub total_variables: usize,
}

type EnvironmentMap<R> = HashMap<EnvironmentId, EnvironmentItem<R>>;

struct ServiceState<R>
where
    R: AppRuntime,
{
    environments: EnvironmentMap<R>,
    active_environments: HashMap<Arc<String>, EnvironmentId>,
    groups: FxHashSet<Arc<String>>,
    expanded_groups: HashSet<Arc<String>>,
    sources: FxHashMap<Arc<String>, PathBuf>,
}

pub struct EnvironmentService<R>
where
    R: AppRuntime,
{
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
    state: Arc<RwLock<ServiceState<R>>>,
    storage: Arc<StorageService<R>>,
}

impl<R> EnvironmentService<R>
where
    R: AppRuntime,
{
    /// `abs_path` is the absolute path to the workspace directory
    pub async fn new(
        abs_path: &Path,
        fs: Arc<dyn FileSystem>,
        storage: Arc<StorageService<R>>,
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
        ctx: &R::AsyncContext,
        params: UpdateEnvironmentGroupParams,
    ) -> joinerror::Result<()> {
        let mut state = self.state.write().await;

        // TODO: Make database errors not fail the operation
        let mut txn = self.storage.storage.begin_write_with_context(ctx).await?;

        let collection_id_inner = params.collection_id.inner();
        if let Some(expanded) = params.expanded {
            if expanded {
                state.expanded_groups.insert(collection_id_inner.clone());
            } else {
                state.expanded_groups.remove(&collection_id_inner);
            }

            self.storage
                .put_expanded_groups_txn(ctx, &mut txn, &state.expanded_groups)
                .await?;
        }

        if let Some(order) = params.order {
            self.storage
                .put_environment_group_order_txn(ctx, &mut txn, collection_id_inner, order)
                .await?;
        }

        txn.commit()?;
        Ok(())
    }

    pub async fn environment(&self, id: &EnvironmentId) -> Option<Arc<Environment<R>>> {
        let state = self.state.read().await;
        state.environments.get(id).map(|item| item.handle.clone())
    }

    pub async fn list_environment_groups(
        &self,
        ctx: &R::AsyncContext,
    ) -> joinerror::Result<Vec<EnvironmentGroup>> {
        let expanded_groups = match self.storage.get_expanded_groups(ctx).await {
            Ok(expanded_groups) => expanded_groups,
            Err(e) => {
                session::warn!(
                    "failed to get expanded groups from the db: {}",
                    e.to_string()
                );
                HashSet::new()
            }
        };

        let mut state = self.state.write().await;
        state.expanded_groups = expanded_groups;

        let data: FxHashMap<String, AnyValue> = self
            .storage
            .list_environment_groups_metadata(ctx)
            .await
            .unwrap_or_else(|e| {
                session::warn!(format!("failed to get environment group metadata: {}", e));
                HashMap::new()
            })
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        let mut groups = Vec::with_capacity(state.groups.len());
        let base_key = segments::SEGKEY_ENVIRONMENT_GROUP
            .to_segkey_buf()
            .to_string();

        for group_id in state.groups.iter() {
            let group_id_str = group_id.as_str();
            let order = data
                .get(&format!("{base_key}:{}:order", group_id_str))
                .map(|v| v.deserialize::<isize>())
                .transpose()
                .unwrap_or_else(|e| {
                    session::warn!(format!(
                        "failed to deserialize order for environment group `{}`: {}",
                        group_id_str, e
                    ));
                    None
                });

            groups.push(EnvironmentGroup {
                collection_id: group_id.clone(),
                expanded: state.expanded_groups.contains(group_id),
                order,
            });
        }

        Ok(groups)
    }

    pub async fn list_environments(
        &self,
        ctx: &R::AsyncContext,
    ) -> Pin<Box<dyn Stream<Item = EnvironmentItemDescription> + Send + '_>> {
        let ctx = ctx.clone();
        let state_clone = self.state.clone();
        let storage_clone = self.storage.storage.clone();
        let sources_clone = state_clone.read().await.sources.clone();

        Box::pin(async_stream::stream! {
            let (tx, mut rx) = mpsc::unbounded_channel::<(EnvironmentItem<R>, DescribeEnvironment)>();
            let scanner = EnvironmentSourceScanner {
                fs: self.fs.clone(),
                sources: sources_clone,
                storage: storage_clone,
                tx,
            };

            let ctx_clone = ctx.clone();
            let scan_task = {
                tokio::spawn(async move {
                    if let Err(e) = scanner.scan(&ctx_clone).await {
                        session::error!(format!("environment scan failed: {}", e));
                    }
                })
            };

            let active_environments = self.storage.get_active_environments(&ctx).await.unwrap_or_else(|e| {
                session::warn!(format!("failed to get activated environments from the db: {}", e));
                HashMap::new()
            });

            let mut state_lock = state_clone.write().await;
            (*state_lock).active_environments = active_environments;

            // Ensure that environments and groups that are not found during the scan will be removed from map
            (*state_lock).environments = HashMap::new();
            (*state_lock).groups = FxHashSet::default();

            while let Some((item, desc)) = rx.recv().await {
                let id = item.id.clone();
                let group_key = item.collection_id.clone().unwrap_or_else(|| {
                    GLOBAL_ACTIVE_ENVIRONMENT_KEY.to_string().into()
                });

                let desc = EnvironmentItemDescription {
                    id: id.clone(),
                    collection_id: item.collection_id.clone(),
                    is_active: state_lock.active_environments.get(&group_key) == Some(&desc.id),
                    display_name: desc.name,
                    order: item.order.clone(),
                    color: desc.color,
                    abs_path: desc.abs_path,
                    total_variables: desc.variables.len(),
                };

                {
                    let group_id = item.collection_id.clone();
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
        ctx: &R::AsyncContext,
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
                .put_environment_order(ctx, &params.id, order)
                .await
            {
                session::warn!(format!("failed to put environment order in the db: {}", e));
            }
        }

        Ok(())
    }

    pub async fn create_environment(
        &self,
        ctx: &R::AsyncContext,
        params: CreateEnvironmentItemParams,
    ) -> joinerror::Result<EnvironmentItemDescription> {
        let abs_path = if let Some(collection_id) = params.collection_id.clone() {
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

        let environment = EnvironmentBuilder::new(self.fs.clone())
            .create::<R>(
                ctx,
                self.storage.variable_store(),
                moss_environment::builder::CreateEnvironmentParams {
                    name: params.name.clone(),
                    abs_path: &abs_path,
                    color: params.color,
                    variables: params.variables,
                },
            )
            .await?;

        let abs_path = environment.abs_path().await;
        let collection_id_inner = params.collection_id.as_ref().map(|id| id.inner());
        let desc = environment.describe(ctx).await?;

        let mut state = self.state.write().await;
        state.environments.insert(
            desc.id.clone(),
            EnvironmentItem {
                id: desc.id.clone(),
                collection_id: collection_id_inner.clone(),
                order: Some(params.order),
                handle: Arc::new(environment),
            },
        );

        let output = EnvironmentItemDescription {
            id: desc.id.clone(),
            collection_id: collection_id_inner,
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
            .put_environment_order(ctx, &desc.id, params.order)
            .await
        {
            session::warn!(format!(
                "failed to put environment order in the db: {}",
                e.to_string()
            ));
            return Ok(output);
        }

        let collection_id = if let Some(collection_id) = &params.collection_id {
            collection_id.inner()
        } else {
            return Ok(output);
        };

        // Create a new environment group if it doesn't exist
        if state.groups.contains(&collection_id) {
            return Ok(output);
        }

        // FIXME: the order should be the current max group order + 1
        let group_order = (state.groups.len() + 1) as isize;

        state.groups.insert(collection_id.clone());
        state.expanded_groups.insert(collection_id.clone());

        {
            let mut txn = match self.storage.begin_write(&ctx).await {
                Ok(txn) => txn,
                Err(e) => {
                    session::warn!(format!("failed to begin write transaction: {}", e));
                    return Ok(output);
                }
            };

            if let Err(e) = self
                .storage
                .put_expanded_groups_txn(&ctx, &mut txn, &state.expanded_groups)
                .await
            {
                session::warn!(format!(
                    "failed to put expanded environment groups in the db: {}",
                    e
                ))
            }

            if let Err(e) = self
                .storage
                .put_environment_group_order_txn(&ctx, &mut txn, collection_id, group_order)
                .await
            {
                session::warn!(format!(
                    "failed to put environment group order in the db: {}",
                    e
                ));
            }

            if let Err(e) = txn.commit() {
                session::warn!(format!("failed to commit transaction: {}", e));
            }
        }

        Ok(output)
    }

    pub async fn delete_environment(
        &self,
        ctx: &R::AsyncContext,
        id: &EnvironmentId,
    ) -> joinerror::Result<()> {
        let mut state = self.state.write().await;
        let environment = state
            .environments
            .remove(id)
            .ok_or_join_err_with::<ErrorNotFound>(|| format!("environment {} not found", id))?;

        // If the environment is currently active, deactivate it
        let env_group_key = environment
            .collection_id
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
            let mut txn = match self.storage.begin_write(ctx).await {
                Ok(txn) => txn,
                Err(e) => {
                    session::warn!(format!("failed to begin transaction: {}", e.to_string()));
                    return Ok(());
                }
            };

            // Clean all the data related to the deleted environment
            if let Err(e) = TransactionalRemoveByPrefix::remove_by_prefix(
                self.storage.storage.item_store().as_ref(),
                ctx,
                &mut txn,
                format!("environment:{}", id).as_str(),
            )
            .await
            {
                session::warn!(format!(
                    "failed to remove environment cache from the db: {}",
                    e
                ));
            }

            // Remove all variables belonging to the deleted environment
            let store = self.storage.variable_store();
            for id in desc.variables.keys() {
                let segkey_localvalue =
                    SegKeyBuf::from(id.as_str()).join(SEGKEY_VARIABLE_LOCALVALUE);

                if let Err(e) = TransactionalRemoveItem::remove(
                    store.as_ref(),
                    ctx,
                    &mut txn,
                    segkey_localvalue,
                )
                .await
                {
                    session::warn!(format!(
                        "failed to remove variable local value in the db: {}",
                        e.to_string()
                    ));
                }

                let segkey_order = SegKeyBuf::from(id.as_str()).join(SEGKEY_VARIABLE_ORDER);
                if let Err(e) =
                    TransactionalRemoveItem::remove(store.as_ref(), ctx, &mut txn, segkey_order)
                        .await
                {
                    session::warn!(format!(
                        "failed to remove variable order in the db: {}",
                        e.to_string()
                    ));
                }
            }

            // Update active environments map
            if active_environments_updated {
                if let Err(e) = self
                    .storage
                    .put_active_environments_txn(&ctx, &mut txn, &state.active_environments)
                    .await
                {
                    session::warn!(format!(
                        "failed to put active environments in the db: {}",
                        e
                    ));
                }
            }

            if let Err(e) = txn.commit() {
                session::warn!(format!("failed to commit transaction: {}", e.to_string()));
            }
        }

        Ok(())
    }

    pub async fn activate_environment(
        &self,
        ctx: &R::AsyncContext,
        params: ActivateEnvironmentItemParams,
    ) -> joinerror::Result<()> {
        let mut state = self.state.write().await;

        let environment_item = state
            .environments
            .get(&params.environment_id)
            .ok_or_join_err_with::<ErrorNotFound>(|| {
                format!("environment {} not found", params.environment_id)
            })?;

        let env_group_key = if let Some(group_id) = &environment_item.collection_id {
            group_id.clone()
        } else {
            GLOBAL_ACTIVE_ENVIRONMENT_KEY.to_string().into()
        };

        state
            .active_environments
            .insert(env_group_key.clone(), params.environment_id);

        {
            let mut txn = match self.storage.begin_write(ctx).await {
                Ok(txn) => txn,
                Err(e) => {
                    session::warn!(format!("failed to start a write transaction: {}", e));
                    return Ok(());
                }
            };

            if let Err(e) = self
                .storage
                .put_active_environments_txn(ctx, &mut txn, &state.active_environments)
                .await
            {
                session::warn!(format!(
                    "failed to put active environments in the db: {}",
                    e
                ))
            }

            if let Err(e) = txn.commit() {
                session::warn!(format!("failed to commit transaction: {}", e.to_string()));
            }
        }
        Ok(())
    }
}

struct ScanSourceJob<R: AppRuntime> {
    source_id: Arc<String>,
    abs_path: PathBuf,
    tx: mpsc::UnboundedSender<(Option<Arc<String>>, Environment<R>)>,
}

struct EnvironmentSourceScanner<R: AppRuntime> {
    fs: Arc<dyn FileSystem>,
    sources: FxHashMap<Arc<String>, PathBuf>,
    storage: Arc<dyn WorkspaceStorage<R::AsyncContext>>,
    tx: mpsc::UnboundedSender<(EnvironmentItem<R>, DescribeEnvironment)>,
}

impl<R: AppRuntime> EnvironmentSourceScanner<R> {
    /// Scans environments from all registered providers in parallel.
    ///
    /// This function implements a multi-stage scanning process:
    /// 1. Loads cached metadata from the database (orders, configurations, etc.)
    /// 2. Spawns parallel scanning tasks for each registered environment provider
    /// 3. Collects environments from all providers through a unified channel
    /// 4. Enriches each environment with cached metadata and forwards to the output channel
    async fn scan(&self, ctx: &R::AsyncContext) -> joinerror::Result<()> {
        // TODO: make database errors not fail the operation

        let data =
            ListByPrefix::list_by_prefix(self.storage.item_store().as_ref(), ctx, "environment")
                .await?
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect::<FxHashMap<_, _>>();

        let (provider_tx, mut provider_rx) =
            mpsc::unbounded_channel::<(Option<Arc<String>>, Environment<R>)>();

        let mut scan_tasks = Vec::new();

        for (source_id, source) in self.sources.iter() {
            let provider_tx_clone = provider_tx.clone();
            let storage_clone = self.storage.variable_store();
            let source_id_clone = source_id.clone();
            let source_clone = source.clone();
            let fs_clone = self.fs.clone();

            let task = tokio::spawn(async move {
                let scan_task = tokio::spawn({
                    let source_id_for_scan = source_id_clone.clone();
                    let source_for_scan = source_clone.clone();
                    let fs_for_scan = fs_clone.clone();
                    let storage_for_scan = storage_clone.clone();
                    async move {
                        if let Err(e) = scan_source::<R>(
                            fs_for_scan,
                            storage_for_scan.clone(),
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

            let order = if let Ok(order) = data
                .get(format!("environment:{}:order", desc.id).as_str())
                .map(|v| v.deserialize::<isize>())
                .transpose()
            {
                order
            } else {
                session::warn!(format!("no order found for environment `{}`", desc.id));
                None
            };

            let environment_item = EnvironmentItem {
                id: desc.id.clone(),
                collection_id,
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

async fn scan_source<R: AppRuntime>(
    fs: Arc<dyn FileSystem>,
    store: Arc<dyn VariableStore<R::AsyncContext>>,
    job: ScanSourceJob<R>,
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

        let maybe_environment = EnvironmentBuilder::new(fs.clone())
            .load::<R>(
                store.clone(),
                EnvironmentLoadParams {
                    abs_path: entry.path(),
                },
            )
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
