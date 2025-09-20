use std::collections::HashMap;

use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, errors::ValidationResultExt};
use moss_logging::session;
use tauri::Emitter;
use validator::Validate;

use crate::{
    app::App,
    constants::ON_DID_CHANGE_CONFIGURATION_CHANNEL,
    models::{events::OnDidChangeConfigurationForFrontend, operations::UpdateConfigurationInput},
};

impl<R: AppRuntime> App<R> {
    pub async fn update_configuration(
        &self,
        _ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: UpdateConfigurationInput,
    ) -> joinerror::Result<()> {
        input.validate().join_err_bare()?;

        self.configuration_service
            .update_value(
                &input.inner.key,
                input.inner.value.clone(),
                input.inner.target,
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
