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

#[derive(Debug, Clone)]
pub struct WorkspaceDescriptor {
    pub id: Uuid,
    pub name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
}

type WorkspaceMap = HashMap<Uuid, Arc<WorkspaceDescriptor>>;

#[derive(Deref)]
pub struct WorkspaceReadGuard<'a, R: TauriRuntime> {
    guard: RwLockReadGuard<'a, Workspace<R>>,
}

#[derive(Deref, DerefMut)]
pub struct WorkspaceWriteGuard<'a, R: TauriRuntime> {
    guard: RwLockMappedWriteGuard<'a, Workspace<R>>,
}

#[derive(Deref, DerefMut)]
pub struct ActiveWorkspace<R: TauriRuntime> {
    pub id: Uuid,
    #[deref]
    #[deref_mut]
    pub this: Workspace<R>,
    pub context: Arc<RwLock<WorkspaceContextState>>,
}

pub struct WorkspaceService<R: TauriRuntime> {
    pub(crate) active_workspace: RwLock<Option<ActiveWorkspace<R>>>,
    pub(crate) known_workspaces: OnceCell<RwLock<WorkspaceMap>>,
}

// impl<R: TauriRuntime> WorkspaceService<R> {
//     pub fn new(app_handle: AppHandle<R>) -> Self {
//         Self {
//             active_workspace: RwLock::new(None),
//             known_workspaces: OnceCell::new(),
//         }
//     }

//     pub async fn active_workspace_id(&self) -> Option<Uuid> {
//         let guard = self.active_workspace.read().await;
//         if guard.is_none() {
//             return None;
//         }

//         let active = guard.as_ref()?;
//         Some(active.id)
//     }

//     pub async fn active_workspace(
//         &self,
//     ) -> Option<(WorkspaceReadGuard<'_, R>, WorkspaceContext<R>)> {
//         let guard = self.active_workspace.read().await;
//         if guard.is_none() {
//             return None;
//         }

//         let context_state = guard.as_ref()?.context.clone();
//         let workspace_guard = RwLockReadGuard::map(guard, |opt| match opt.as_ref() {
//             Some(active) => &active.this,
//             None => unreachable!("Already checked for None above"),
//         });

//         let context = WorkspaceContext::new(self.app_handle.clone(), context_state);
//         Some((
//             WorkspaceReadGuard {
//                 guard: workspace_guard,
//             },
//             context,
//         ))
//     }

//     pub async fn active_workspace_mut(
//         &self,
//     ) -> Option<(WorkspaceWriteGuard<'_, R>, WorkspaceContext<R>)> {
//         let guard = self.active_workspace.write().await;
//         if guard.is_none() {
//             return None;
//         }

//         let context_state = guard.as_ref()?.context.clone();
//         let workspace_guard = RwLockWriteGuard::map(guard, |opt| match opt.as_mut() {
//             Some(active) => &mut active.this,
//             None => unreachable!("Already checked for None above"),
//         });

//         let context = WorkspaceContext::new(self.app_handle.clone(), context_state);
//         Some((
//             WorkspaceWriteGuard {
//                 guard: workspace_guard,
//             },
//             context,
//         ))
//     }

//     pub(super) async fn activate_workspace(&self, id: Uuid, workspace: Workspace<R>) {
//         let mut active_workspace = self.active_workspace.write().await;
//         *active_workspace = Some(ActiveWorkspace {
//             id,
//             this: workspace,
//             context: Arc::new(RwLock::new(WorkspaceContextState::new())),
//         });
//     }

//     pub(super) async fn deactivate_workspace(&self) {
//         let mut active_workspace = self.active_workspace.write().await;
//         *active_workspace = None;
//     }
// }
