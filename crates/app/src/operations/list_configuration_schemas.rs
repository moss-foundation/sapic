use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use sapic_ipc::contracts::configuration::ListConfigurationSchemasOutput;
use sapic_runtime::globals::GlobalConfigurationRegistry;

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn list_configuration_schemas(
        &self,
        _ctx: &R::AsyncContext,
        delegate: &AppDelegate<R>,
    ) -> joinerror::Result<ListConfigurationSchemasOutput> {
        let registry = GlobalConfigurationRegistry::get(delegate);

        Ok(ListConfigurationSchemasOutput {
            schemas: registry
                .nodes()
                .into_iter()
                .map(|(_, node)| node.as_ref().into())
                .collect(),
        })
    }
}
