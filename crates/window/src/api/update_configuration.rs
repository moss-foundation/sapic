use std::collections::HashMap;

use crate::{
    constants::ON_DID_CHANGE_CONFIGURATION_CHANNEL,
    models::{events::OnDidChangeConfigurationForFrontend, operations::UpdateConfigurationInput},
    window::Window,
};
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, errors::ValidationResultExt};
use moss_logging::session;
use sapic_runtime::{app::settings_storage::SettingScope, globals::GlobalSettingsStorage};
use tauri::Emitter;
use validator::Validate;

impl<R: AppRuntime> Window<R> {
    pub async fn update_configuration(
        &self,
        _ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: UpdateConfigurationInput,
    ) -> joinerror::Result<()> {
        input.validate().join_err_bare()?;

        let settings_storage = GlobalSettingsStorage::get(app_delegate);
        settings_storage
            .update_value(
                &SettingScope::User,
                &input.inner.key,
                input.inner.value.clone(),
            )
            .await?;

        if let Err(e) = app_delegate.emit(
            ON_DID_CHANGE_CONFIGURATION_CHANNEL,
            OnDidChangeConfigurationForFrontend {
                affected_keys: vec![input.inner.key.clone()],
                changes: HashMap::from([(input.inner.key, input.inner.value)]),
            },
        ) {
            session::error!(
                "failed to emit event after updating configuration: {}",
                e.to_string()
            );
        }

        Ok(())
    }
}
