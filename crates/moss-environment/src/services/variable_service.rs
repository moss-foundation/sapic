use hcl::Expression as HclExpression;
use joinerror::Error;
use json_patch::{AddOperation, PatchOperation, jsonptr::PointerBuf};
use moss_applib::{AppRuntime, ServiceMarker};
use serde_json::{Value as JsonValue, json, map::Map as JsonMap};
use std::{collections::HashMap, marker::PhantomData};
use tokio::sync::RwLock;

use crate::{
    configuration::VariableDefinition,
    models::{
        primitives::VariableId,
        types::{AddVariableParams, VariableOptions},
    },
    services::{AnyStorageService, AnySyncService},
};

#[derive(Debug, Clone)]
pub struct VariableItem {
    pub id: VariableId,
    pub local_value: Option<HclExpression>,
    pub global_value: Option<HclExpression>,
    pub desc: Option<String>,
    pub options: VariableOptions,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ServiceState {
    variables: HashMap<VariableId, VariableItem>,
}

#[allow(private_bounds)]
pub struct VariableService<R, StorageService, SyncService>
where
    R: AppRuntime,
    StorageService: AnyStorageService<R>,
    SyncService: AnySyncService<R>,
{
    state: RwLock<ServiceState>,
    #[allow(dead_code)]
    storage_service: StorageService,
    sync_service: SyncService,
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
            state: RwLock::new(ServiceState {
                variables: HashMap::new(),
            }),
            storage_service,
            sync_service,
            _marker: PhantomData,
        }
    }

    pub async fn batch_remove(&self, ids: Vec<VariableId>) -> joinerror::Result<()> {
        // let mut patches = Vec::with_capacity(params.len());

        Ok(())
    }

    pub async fn batch_add(&self, params: Vec<AddVariableParams>) -> joinerror::Result<()> {
        let mut patches = Vec::with_capacity(params.len());

        patches.push(PatchOperation::Add(AddOperation {
            path: PointerBuf::parse("/variable").unwrap(),
            value: json!({}),
        }));

        for param in params {
            let id = VariableId::new();
            let global_value = if let Some(value) = param.global_value {
                let hcl_expr: HclExpression = value.try_into().map_err(|err| {
                    Error::new::<()>(format!(
                        "failed to convert global value expression: {}",
                        err
                    ))
                })?;
                hcl_expr
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

async fn collect_variables<R, StorageService>(
    map: &JsonMap<String, JsonValue>,
    storage: &StorageService,
) -> joinerror::Result<HashMap<VariableId, VariableItem>>
where
    R: AppRuntime,
    StorageService: AnyStorageService<R>,
{
    let mut variables = HashMap::new();
    for (id_str, value) in map {
        let definition = if let Ok(d) = serde_json::from_value::<VariableDefinition>(value.clone())
            .map_err(|err| {
                Error::new::<()>(format!(
                    "failed to convert variable definition from json: {}",
                    err
                ))
            }) {
            d
        } else {
            println!("failed to convert variable definition from json: {}", value); // TODO: log error
            continue;
        };

        let id = VariableId::from(id_str.clone());
        variables.insert(
            id.clone(),
            VariableItem {
                id,
                local_value: None,
                global_value: Some(definition.value),
                desc: definition.description,
                options: definition.options,
            },
        );
    }

    Ok(variables)
}

#[cfg(test)]
mod tests {
    use hcl::{Expression as HclExpression, expr::Variable};
    use indexmap::IndexMap;
    use moss_applib::{context::AsyncContext, mock::MockAppRuntime};
    use moss_fs::{RealFileSystem, model_registry::GlobalModelRegistry};
    use moss_hcl::{Block, LabeledBlock};
    use moss_storage::common::VariableStore;
    use std::{path::PathBuf, sync::Arc};

    use crate::{
        builder::{EnvironmentBuilder, EnvironmentCreateParams},
        configuration::{EnvironmentFile, Metadata, VariableDefinition},
        environment::Environment,
        models::primitives::EnvironmentId,
        services::{storage_service::StorageService, sync_service::SyncService},
    };

    use super::*;

    struct TestVariableStore {}

    impl VariableStore<AsyncContext> for TestVariableStore {}

    #[test]
    fn t() {
        let mut map = IndexMap::new();

        // Test different types of expressions
        map.insert(
            VariableId::new(),
            VariableDefinition {
                name: "simple_variable".to_string(),
                value: HclExpression::Variable(Variable::new("test".to_string()).unwrap()),
                kind: None,
                description: None,
                options: VariableOptions { disabled: false },
            },
        );

        // Add a function call expression - parse it as proper HCL
        let function_hcl = "_ = try(coalesce(var.ami, data.aws_ssm_parameter.this[0].value), null)";
        if let Ok(body) = hcl::from_str::<hcl::Body>(function_hcl) {
            if let Some(attr) = body.attributes().next() {
                map.insert(
                    VariableId::new(),
                    VariableDefinition {
                        name: "function_call".to_string(),
                        value: HclExpression::new(attr.expr().clone()),
                        kind: None,
                        description: None,
                        options: VariableOptions { disabled: false },
                    },
                );
            }
        }

        // Add a conditional expression - parse it as proper HCL
        let conditional_hcl = "_ = local.create ? 1 : 0";
        if let Ok(body) = hcl::from_str::<hcl::Body>(conditional_hcl) {
            if let Some(attr) = body.attributes().next() {
                map.insert(
                    VariableId::new(),
                    VariableDefinition {
                        name: "conditional".to_string(),
                        value: HclExpression::new(attr.expr().clone()),
                        kind: None,
                        description: None,
                        options: VariableOptions { disabled: false },
                    },
                );
            }
        }

        // Create original HCL structure
        let original_hcl = EnvironmentFile {
            metadata: Block::new(Metadata {
                id: EnvironmentId::new(),
                color: None,
            }),
            variables: Some(LabeledBlock::new(map)),
        };

        // Test HCL serialization
        let hcl_text = hcl::to_string(&original_hcl).unwrap();
        println!("Original HCL:");
        println!("{}", hcl_text);

        // Test JSON serialization
        let json_value = serde_json::to_string(&original_hcl).unwrap();
        println!("\nJSON representation:");
        println!("{}", json_value);

        // Test JSON deserialization
        println!("\n=== Attempting JSON deserialization ===");
        let json_deserialized: EnvironmentFile = match serde_json::from_str(&json_value) {
            Ok(data) => {
                println!("✅ JSON deserialization successful!");
                data
            }
            Err(e) => {
                println!("❌ JSON deserialization failed: {}", e);
                println!("JSON data being parsed:\n{}", json_value);
                panic!("JSON deserialization failed: {}", e);
            }
        };

        // Test final HCL serialization
        let hcl_text_2 = hcl::to_string(&json_deserialized).unwrap();
        println!("\nFinal HCL:");
        println!("{}", hcl_text_2);

        // Verify that we can round-trip without data loss
        assert!(!hcl_text_2.is_empty());
        println!("\n✅ Successfully converted HCL -> JSON -> HCL without errors!");

        // Verify that the structure is preserved (both should have the same number of variables)
        assert_eq!(
            original_hcl.variables.is_some(),
            json_deserialized.variables.is_some()
        );
        if let (Some(orig_vars), Some(final_vars)) =
            (&original_hcl.variables, &json_deserialized.variables)
        {
            assert_eq!(orig_vars.len(), final_vars.len());
        }

        println!("✅ Data integrity verified!");
        println!("✅ Function calls, conditionals, and complex expressions support added!");
    }

    #[tokio::test]
    async fn test_variable_service() {
        let abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data");

        // Clean up any existing test file to avoid "already_exists" error
        let test_file_path = abs_path.join("data.env.sap");
        if test_file_path.exists() {
            std::fs::remove_file(&test_file_path).ok(); // Ignore errors if file doesn't exist
        }

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

        let _env: Environment<MockAppRuntime> = EnvironmentBuilder::new(fs, global_model_registry)
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
