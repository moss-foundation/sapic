use moss_applib::AppRuntime;

use crate::{models::operations::ListConfigurationSchemasOutput, window::Window};

impl<R: AppRuntime> Window<R> {
    pub async fn list_configuration_schemas(
        &self,
        _ctx: &R::AsyncContext,
    ) -> joinerror::Result<ListConfigurationSchemasOutput> {
        let schemas = self.configuration_service.schemas();
        Ok(ListConfigurationSchemasOutput {
            schemas: schemas
                .into_iter()
                .map(|(_, schema)| schema.as_ref().into())
                .collect(),
        })
    }
}
