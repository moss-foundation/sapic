pub mod operations;
pub mod types;

use joinerror::OptionExt;
use moss_applib::GenericAppHandle;
use sapic_ipc::TauriResult;
use sapic_runtime::app::settings_storage::SettingsStorage;
use std::sync::{Arc, OnceLock};
use tauri::{
    AppHandle, Runtime,
    plugin::{Builder, TauriPlugin},
};
use tracing::instrument;

use crate::operations::*;

pub(crate) type ProviderCallback =
    Arc<dyn Fn(&GenericAppHandle) -> joinerror::Result<Arc<dyn SettingsStorage>> + Send + Sync>;

pub(crate) static PROVIDER_CALLBACK: OnceLock<ProviderCallback> = OnceLock::new();

pub fn init<
    R: Runtime,
    F: Fn(&GenericAppHandle) -> joinerror::Result<Arc<dyn SettingsStorage>> + Send + Sync + 'static,
>(
    f: F,
) -> TauriPlugin<R> {
    let _ = PROVIDER_CALLBACK.set(Arc::new(f));

    Builder::new("settings-storage")
        .invoke_handler(tauri::generate_handler![
            get_value,
            update_value,
            remove_value,
        ])
        .build()
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn get_value<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: GetValueInput,
) -> TauriResult<GetValueOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("settings storage provider not found")?;

    let settings_storage: Arc<dyn SettingsStorage> = provider(&GenericAppHandle::new(app_handle))?;

    let value = settings_storage
        .get_value(&input.scope.clone().into(), &input.key)
        .await?;

    Ok(GetValueOutput {
        scope: input.scope,
        key: input.key,
        value,
    })
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn update_value<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: UpdateValueInput,
) -> TauriResult<UpdateValueOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("settings storage provider not found")?;

    let settings_storage: Arc<dyn SettingsStorage> = provider(&GenericAppHandle::new(app_handle))?;

    settings_storage
        .update_value(&input.scope.clone().into(), &input.key, input.value)
        .await?;

    Ok(UpdateValueOutput {})
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn remove_value<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: RemoveValueInput,
) -> TauriResult<RemoveValueOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("settings storage provider not found")?;

    let settings_storage: Arc<dyn SettingsStorage> = provider(&GenericAppHandle::new(app_handle))?;

    let value = settings_storage
        .remove_value(&input.scope.clone().into(), &input.key)
        .await?;

    Ok(RemoveValueOutput {
        scope: input.scope,
        key: input.key,
        value,
    })
}
