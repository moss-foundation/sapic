use hcl::Expression as HclExpression;
use joinerror::Error;
use json_patch::{AddOperation, PatchOperation, jsonptr::PointerBuf};
use moss_applib::{AppRuntime, ServiceMarker};
use serde_json::json;
use std::{collections::HashMap, marker::PhantomData, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    configuration::VariableDefinition,
    models::{
        primitives::VariableId,
        types::{AddVariableParams, Expression, VariableKind, VariableName, VariableOptions},
    },
    services::{
        AnyStorageService, AnySyncService, storage_service::StorageService,
        sync_service::SyncService,
    },
};

#[derive(Debug, Clone)]
pub struct VariableItem {
    pub id: VariableId,
    pub kind: Option<VariableKind>,
    pub global_value: Option<Expression>,
    pub desc: Option<String>,
    pub options: VariableOptions,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ServiceState {
    variables: HashMap<VariableName, VariableItem>,
}

#[allow(private_bounds)]
pub struct VariableService<R, StorageService, SyncService>
where
    R: AppRuntime,
    StorageService: AnyStorageService<R>,
    SyncService: AnySyncService<R>,
{
    #[allow(dead_code)]
    storage_service: StorageService,

    sync_service: SyncService,
    // state: Arc<RwLock<ServiceState>>,
    _marker: PhantomData<R>,
}

impl<R, StorageService, SyncService> ServiceMarker
    for VariableService<R, StorageService, SyncService>
where
    R: AppRuntime,
    StorageService: AnyStorageService<R>,
    SyncService: AnySyncService<R>,
{
}

#[allow(private_bounds)]
impl<R, StorageService, SyncService> VariableService<R, StorageService, SyncService>
where
    R: AppRuntime,
    StorageService: AnyStorageService<R>,
    SyncService: AnySyncService<R>,
{
    pub fn new(storage_service: StorageService, sync_service: SyncService) -> Self {
        Self {
            storage_service,
            sync_service,
            _marker: PhantomData,
        }
    }

    pub async fn batch_add(&self, params: Vec<AddVariableParams>) -> joinerror::Result<()> {
        let mut patches = Vec::with_capacity(params.len());

        patches.push(PatchOperation::Add(AddOperation {
            path: PointerBuf::parse("/variable").unwrap(),
            value: json!({}),
        }));

        for param in params {
            let id = VariableId::new();
            let global_value: HclExpression = if let Some(value) = param.global_value {
                value.try_into().map_err(|err| {
                    Error::new::<()>(format!(
                        "failed to convert global value expression: {}",
                        err
                    ))
                })?
            } else {
                HclExpression::Null
            };

            let value = VariableDefinition {
                name: param.name,
                value: global_value,
                kind: param.kind,
                description: param.desc,
                options: param.options,
            };

            patches.push(PatchOperation::Add(AddOperation {
                path: PointerBuf::parse(format!("/variable/{}", id)).unwrap(),
                value: serde_json::to_value(value).map_err(|err| {
                    Error::new::<()>(format!(
                        "failed to convert variable definition to json: {}",
                        err
                    ))
                })?,
            }));
        }

        self.sync_service.apply(&patches).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use moss_applib::{context::AsyncContext, mock::MockAppRuntime};
    use moss_fs::{RealFileSystem, model_registry::GlobalModelRegistry};
    use moss_hcl::{Block, LabeledBlock};
    use moss_storage::common::VariableStore;
    use std::path::{Path, PathBuf};

    use crate::{
        builder::{EnvironmentBuilder, EnvironmentCreateParams},
        configuration::{EnvironmentFile, Metadata},
        environment::Environment,
        models::primitives::EnvironmentId,
    };

    use super::*;

    struct TestVariableStore {}

    impl VariableStore<AsyncContext> for TestVariableStore {}

    #[test]
    fn t() {
        let mut map = IndexMap::new();
        map.insert(
            VariableId::new(),
            VariableDefinition {
                name: "test".to_string(),
                value: HclExpression::Null,
                kind: None,
                description: None,
                options: VariableOptions { disabled: false },
            },
        );

        let h = EnvironmentFile {
            metadata: Block::new(Metadata {
                id: EnvironmentId::new(),
                color: None,
            }),
            variables: Some(LabeledBlock::new(map)),
        };

        let hcl_value = hcl::to_value(h).unwrap();
        let json_value = serde_json::to_value(hcl_value).unwrap();

        println!("{}", json_value);

        let r = r#"
        {
            "metadata": { "id": "AieBKAgIMq" },
            "variable": { "zcs0RWuerj": { "name": "test", "value": null, "kind": null, "description": null, "options": { "disabled": false } } }
        }
        "#;

        let hcl_value = hcl::from_str::<EnvironmentFile>(r).unwrap();

        dbg!(&hcl_value);
    }

    #[tokio::test]
    async fn test_variable_service() {
        let abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data");

        let fs = Arc::new(RealFileSystem::new());
        let global_model_registry = GlobalModelRegistry::new();
        let storage_service: StorageService<MockAppRuntime> =
            StorageService::new(Arc::new(TestVariableStore {}));
        let sync_service = SyncService::new(
            abs_path.join("data.env.sap").to_string_lossy().to_string(),
            global_model_registry.clone(),
            fs.clone(),
        );
        let variable_service = VariableService::new(storage_service, sync_service);

        let env: Environment<MockAppRuntime> = EnvironmentBuilder::new(fs, global_model_registry)
            .create(EnvironmentCreateParams {
                name: "data".to_string(),
                abs_path,
                color: None,
            })
            .await
            .unwrap();

        variable_service
            .batch_add(vec![AddVariableParams {
                name: "test".to_string(),
                global_value: None,
                kind: None,
                desc: None,
                local_value: None,
                order: 0,
                options: VariableOptions { disabled: false },
            }])
            .await
            .unwrap();
    }
}
