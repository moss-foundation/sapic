use sapic_base::configuration::types::ConfigurationSchema;
use serde::Serialize;
use ts_rs::TS;
use validator::Validate;

#[derive(Debug, Clone, Serialize, TS, Validate)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListConfigurationSchemasOutput {
    #[ts(type = "ConfigurationSchema[]")]
    pub schemas: Vec<ConfigurationSchema>,
}
