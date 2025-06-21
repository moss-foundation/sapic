use anyhow::Result;
use derive_more::{Deref, DerefMut};
use moss_activity_indicator::ActivityIndicator;
use moss_applib::{Service, context::Context};
use moss_fs::FileSystem;
use moss_storage::{
    GlobalStorage, global_storage::entities::WorkspaceInfoEntity, primitives::segkey::SegmentExt,
    storage::operations::ListByPrefix,
};
use moss_text::ReadOnlyStr;
use moss_workspace::{
    Workspace,
    context::{WorkspaceContext, WorkspaceContextState},
};
use rustc_hash::FxHashMap;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::{OnceCell, RwLock, RwLockMappedWriteGuard, RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;

use crate::{
    command::{CommandCallback, CommandDecl},
    dirs,
    models::types::{ColorThemeInfo, LocaleInfo},
    storage::segments::WORKSPACE_SEGKEY,
};

pub struct AppPreferences {
    pub theme: RwLock<Option<ColorThemeInfo>>,
    pub locale: RwLock<Option<LocaleInfo>>,
}

pub struct AppDefaults {
    pub theme: ColorThemeInfo,
    pub locale: LocaleInfo,
}

type AnyService = Arc<dyn Any + Send + Sync>;

#[derive(Default)]
pub struct AppServices(FxHashMap<TypeId, AnyService>);

impl Deref for AppServices {
    type Target = FxHashMap<TypeId, AnyService>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AppServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct AppCommands<R: TauriRuntime>(FxHashMap<ReadOnlyStr, CommandCallback<R>>);

impl<R: TauriRuntime> Default for AppCommands<R> {
    fn default() -> Self {
        Self(FxHashMap::default())
    }
}

impl<R: TauriRuntime> Deref for AppCommands<R> {
    type Target = FxHashMap<ReadOnlyStr, CommandCallback<R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: TauriRuntime> DerefMut for AppCommands<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct WorkspaceDescriptor {
    pub id: Uuid,
    pub name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
}

type WorkspaceMap = HashMap<Uuid, Arc<WorkspaceDescriptor>>;

// TODO: Might be better to create a service for this
#[derive(Deref, DerefMut)]
pub struct ActiveWorkspace<R: TauriRuntime> {
    pub id: Uuid,
    #[deref]
    #[deref_mut]
    pub this: Workspace<R>,
    pub context: Arc<RwLock<WorkspaceContextState>>,
}

#[derive(Deref)]
pub struct WorkspaceReadGuard<'a, R: TauriRuntime> {
    guard: RwLockReadGuard<'a, Workspace<R>>,
}

#[derive(Deref, DerefMut)]
pub struct WorkspaceWriteGuard<'a, R: TauriRuntime> {
    guard: RwLockMappedWriteGuard<'a, Workspace<R>>,
}

pub struct App<R: TauriRuntime> {
    pub(crate) fs: Arc<dyn FileSystem>,
    pub(crate) app_handle: AppHandle<R>,
    pub(crate) commands: AppCommands<R>,
    pub(crate) preferences: AppPreferences,
    pub(crate) defaults: AppDefaults,
    pub(crate) services: AppServices,

    // TODO: This is also might be better to be a service
    pub(crate) activity_indicator: ActivityIndicator<R>,
    pub(crate) active_workspace: RwLock<Option<ActiveWorkspace<R>>>,
    pub(crate) known_workspaces: OnceCell<RwLock<WorkspaceMap>>,
    pub(super) global_storage: Arc<dyn GlobalStorage>,

    // TODO: Not sure this the best place for this, and do we even need it
    pub(crate) abs_path: Arc<Path>,
}

impl<R: TauriRuntime> Deref for App<R> {
    type Target = AppHandle<R>;

    fn deref(&self) -> &Self::Target {
        &self.app_handle
    }
}

pub struct AppBuilder<R: TauriRuntime> {
    fs: Arc<dyn FileSystem>,
    app_handle: AppHandle<R>,
    services: AppServices,
    defaults: AppDefaults,
    preferences: AppPreferences,
    commands: AppCommands<R>,
    activity_indicator: ActivityIndicator<R>,
    global_storage: Arc<dyn GlobalStorage>,
    abs_path: Arc<Path>,
}

impl<R: TauriRuntime> AppBuilder<R> {
    pub fn new(
        app_handle: AppHandle<R>,
        global_storage: Arc<dyn GlobalStorage>,
        activity_indicator: ActivityIndicator<R>,
        defaults: AppDefaults,
        fs: Arc<dyn FileSystem>,
        abs_path: PathBuf,
    ) -> Self {
        Self {
            fs,
            app_handle,
            defaults,
            preferences: AppPreferences {
                theme: RwLock::new(None),
                locale: RwLock::new(None),
            },
            commands: Default::default(),
            services: Default::default(),
            activity_indicator,
            global_storage,
            abs_path: abs_path.into(),
        }
    }

    pub fn with_service<T: Service + Send + Sync>(mut self, service: T) -> Self {
        self.services.insert(TypeId::of::<T>(), Arc::new(service));
        self
    }

    pub fn with_command(mut self, command: CommandDecl<R>) -> Self {
        self.commands.insert(command.name, command.callback);
        self
    }

    pub fn build(self) -> App<R> {
        App {
            fs: self.fs,
            app_handle: self.app_handle,
            commands: self.commands,
            preferences: self.preferences,
            defaults: self.defaults,
            services: self.services,
            activity_indicator: self.activity_indicator,
            active_workspace: RwLock::new(None),
            known_workspaces: OnceCell::new(),
            global_storage: self.global_storage,
            abs_path: self.abs_path,
        }
    }
}
impl<R: TauriRuntime> App<R> {
    pub fn preferences(&self) -> &AppPreferences {
        &self.preferences
    }

    pub fn defaults(&self) -> &AppDefaults {
        &self.defaults
    }

    pub fn service<T: Service>(&self) -> &T {
        let type_id = TypeId::of::<T>();
        let service = self.services.get(&type_id).expect(&format!(
            "Service {} must be registered before it can be used",
            std::any::type_name::<T>()
        ));

        service.downcast_ref::<T>().expect(&format!(
            "Service {} is registered with the wrong type type id",
            std::any::type_name::<T>()
        ))
    }

    pub fn command(&self, id: &ReadOnlyStr) -> Option<CommandCallback<R>> {
        self.commands.get(id).map(|cmd| Arc::clone(cmd))
    }

    pub async fn active_workspace_id(&self) -> Option<Uuid> {
        let guard = self.active_workspace.read().await;
        if guard.is_none() {
            return None;
        }

        let active = guard.as_ref()?;
        Some(active.id)
    }

    pub async fn active_workspace(
        &self,
    ) -> Option<(WorkspaceReadGuard<'_, R>, WorkspaceContext<R>)> {
        let guard = self.active_workspace.read().await;
        if guard.is_none() {
            return None;
        }

        let context_state = guard.as_ref()?.context.clone();
        let workspace_guard = RwLockReadGuard::map(guard, |opt| match opt.as_ref() {
            Some(active) => &active.this,
            None => unreachable!("Already checked for None above"),
        });

        let context = WorkspaceContext::new(self.app_handle.clone(), context_state);
        Some((
            WorkspaceReadGuard {
                guard: workspace_guard,
            },
            context,
        ))
    }

    pub async fn active_workspace_mut(
        &self,
    ) -> Option<(WorkspaceWriteGuard<'_, R>, WorkspaceContext<R>)> {
        let guard = self.active_workspace.write().await;
        if guard.is_none() {
            return None;
        }

        let context_state = guard.as_ref()?.context.clone();
        let workspace_guard = RwLockWriteGuard::map(guard, |opt| match opt.as_mut() {
            Some(active) => &mut active.this,
            None => unreachable!("Already checked for None above"),
        });

        let context = WorkspaceContext::new(self.app_handle.clone(), context_state);
        Some((
            WorkspaceWriteGuard {
                guard: workspace_guard,
            },
            context,
        ))
    }

    pub(super) async fn activate_workspace(&self, id: Uuid, workspace: Workspace<R>) {
        let mut active_workspace = self.active_workspace.write().await;
        *active_workspace = Some(ActiveWorkspace {
            id,
            this: workspace,
            context: Arc::new(RwLock::new(WorkspaceContextState::new())),
        });
    }

    pub(super) async fn deactivate_workspace(&self) {
        let mut active_workspace = self.active_workspace.write().await;
        *active_workspace = None;
    }

    pub(super) async fn workspaces<C: Context<R>>(&self, ctx: &C) -> Result<&RwLock<WorkspaceMap>> {
        Ok(self
            .known_workspaces
            .get_or_try_init(|| async move {
                let mut workspaces: WorkspaceMap = HashMap::new();

                let dir_abs_path = self.absolutize(dirs::WORKSPACES_DIR);
                if !dir_abs_path.exists() {
                    return Ok(RwLock::new(workspaces));
                }

                let restored_items = ListByPrefix::list_by_prefix(
                    self.global_storage.item_store().as_ref(),
                    WORKSPACE_SEGKEY.as_str().expect("invalid utf-8"),
                )?;
                let filtered_restored_items = restored_items.iter().filter_map(|(k, v)| {
                    let path = k.after(&WORKSPACE_SEGKEY);
                    if let Some(path) = path {
                        Some((path, v))
                    } else {
                        None
                    }
                });

                let mut restored_entities = HashMap::with_capacity(restored_items.len());
                for (segkey, value) in filtered_restored_items {
                    let encoded_name = match String::from_utf8(segkey.as_bytes().to_owned()) {
                        Ok(name) => name,
                        Err(_) => {
                            // TODO: logging
                            println!("failed to get the workspace {:?} name", segkey);
                            continue;
                        }
                    };

                    restored_entities.insert(encoded_name, value);
                }

                let fs = <dyn FileSystem>::global::<R, C>(ctx);

                let mut read_dir = fs.read_dir(&dir_abs_path).await?;
                while let Some(entry) = read_dir.next_entry().await? {
                    if !entry.file_type().await?.is_dir() {
                        continue;
                    }

                    let id_str = entry.file_name().to_string_lossy().to_string();
                    let id = match Uuid::parse_str(&id_str) {
                        Ok(id) => id,
                        Err(_) => {
                            // TODO: logging
                            println!("failed to get the collection {:?} name", id_str);
                            continue;
                        }
                    };

                    let summary = Workspace::<R>::summary(ctx, &entry.path()).await?;

                    let restored_entity =
                        match restored_entities.remove(&id_str).map_or(Ok(None), |v| {
                            v.deserialize::<WorkspaceInfoEntity>().map(Some)
                        }) {
                            Ok(value) => value,
                            Err(_err) => {
                                // TODO: logging
                                println!("failed to get the workspace {:?} info", id_str);
                                continue;
                            }
                        };

                    workspaces.insert(
                        id,
                        WorkspaceDescriptor {
                            id,
                            name: summary.manifest.name,
                            abs_path: entry.path().into(),
                            last_opened_at: restored_entity.map(|v| v.last_opened_at),
                        }
                        .into(),
                    );
                }

                Ok::<_, anyhow::Error>(RwLock::new(workspaces))
            })
            .await?)
    }

    pub(super) fn absolutize(&self, path: impl AsRef<Path>) -> PathBuf {
        self.abs_path.join(path)
    }

    /// Test only utility, not feature-flagged for easier CI setup
    pub fn __storage(&self) -> Arc<dyn GlobalStorage> {
        self.global_storage.clone()
    }
}
