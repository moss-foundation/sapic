pub mod operations;
pub mod types;

use joinerror::OptionExt;
use sapic_runtime::app::{GenericAppHandle, settings_storage::SettingsStorage};
use std::sync::{Arc, OnceLock};
use tauri::{
    AppHandle, Runtime, State,
    plugin::{Builder, TauriPlugin},
};
use tracing::instrument;

use crate::operations::*;

pub(crate) type ProviderCallback =
    Arc<dyn Fn(&GenericAppHandle) -> joinerror::Result<Arc<dyn SettingsStorage>> + Send + Sync>;

type AsyncContext<'a> = State<'a, sapic_core::context::ArcContext>;

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
            batch_update_value,
            batch_get_value,
            batch_remove_value,
        ])
        .build()
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn get_value<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: GetValueInput,
) -> joinerror::Result<GetValueOutput> {
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
    ctx: AsyncContext<'a>,
    app_handle: AppHandle<R>,
    input: UpdateValueInput,
) -> joinerror::Result<UpdateValueOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("settings storage provider not found")?;

    let settings_storage: Arc<dyn SettingsStorage> = provider(&GenericAppHandle::new(app_handle))?;

    settings_storage
        .update_value(
            ctx.inner(),
            &input.scope.clone().into(),
            &input.key,
            input.value,
        )
        .await?;

    Ok(UpdateValueOutput {})
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn remove_value<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: RemoveValueInput,
) -> joinerror::Result<RemoveValueOutput> {
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

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn batch_update_value<'a, R: tauri::Runtime>(
    #[allow(unused)] app_handle: AppHandle<R>,
    input: BatchUpdateValueInput,
) -> joinerror::Result<BatchUpdateValueOutput> {
    unimplemented!()
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn batch_get_value<'a, R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    input: BatchGetValueInput,
) -> joinerror::Result<BatchGetValueOutput> {
    let provider = PROVIDER_CALLBACK
        .get()
        .ok_or_join_err::<()>("settings storage provider not found")?;

    let settings_storage: Arc<dyn SettingsStorage> = provider(&GenericAppHandle::new(app_handle))?;

    let values = settings_storage
        .batch_get_value(
            &input.scope.clone().into(),
            &input.keys.iter().map(|k| k.as_str()).collect::<Vec<&str>>(),
        )
        .await?;

    Ok(BatchGetValueOutput {
        scope: input.scope,
        values: values.into_iter().collect(),
    })
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_handle))]
async fn batch_remove_value<'a, R: tauri::Runtime>(
    #[allow(unused)] app_handle: AppHandle<R>,
    input: BatchRemoveValueInput,
) -> joinerror::Result<BatchRemoveValueOutput> {
    unimplemented!()
}
