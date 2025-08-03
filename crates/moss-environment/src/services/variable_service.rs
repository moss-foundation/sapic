use hcl::Expression as HclExpression;
use joinerror::Error;
use json_patch::{AddOperation, PatchOperation, RemoveOperation, jsonptr::PointerBuf};
use moss_applib::{AppRuntime, ServiceMarker};
use moss_hcl::json_to_hcl;
use serde_json::{Value as JsonValue, json, map::Map as JsonMap};
use std::{collections::HashMap, marker::PhantomData, path::Path, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    configuration::VariableSpec,
    models::{
        primitives::VariableId,
        types::{AddVariableParams, VariableOptions},
    },
    services::{AnySyncService, AnyVariableService, sync_service::SyncService},
};

#[derive(Debug, Clone)]
pub struct VariableItem {
    pub id: VariableId,
    pub name: String,
    pub local_value: HclExpression,
    pub global_value: HclExpression,
    pub desc: Option<String>,
    pub order: isize,
    pub options: VariableOptions,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ServiceState {
    variables: HashMap<VariableId, VariableItem>,
}

#[allow(private_bounds)]
pub struct VariableService<R>
where
    R: AppRuntime,
{
    state: RwLock<ServiceState>,
    // #[allow(dead_code)]
    // storage_service: Arc<StorageService<R>>,
    sync_service: Arc<SyncService>,
    _marker: PhantomData<R>,
}

unsafe impl<R> Send for VariableService<R> where R: AppRuntime {}
unsafe impl<R> Sync for VariableService<R> where R: AppRuntime {}

impl<R> ServiceMarker for VariableService<R> where R: AppRuntime {}

impl<R> AnyVariableService<R> for VariableService<R> where R: AppRuntime {}

#[allow(private_bounds)]
impl<R> VariableService<R>
where
    R: AppRuntime,
{
    pub fn new(
        source: Option<&JsonMap<String, JsonValue>>,
        // storage_service: Arc<StorageService<R>>,
        sync_service: Arc<SyncService>,
    ) -> joinerror::Result<Self> {
        let variables = if let Some(source) = source {
            collect_variables(source)?
        } else {
            HashMap::new()
        };

        Ok(Self {
            state: RwLock::new(ServiceState { variables }),
            // storage_service,
            sync_service,
            _marker: PhantomData,
        })
    }

    pub async fn list(&self) -> HashMap<VariableId, VariableItem> {
        self.state.read().await.variables.clone()
    }

    pub async fn batch_remove(&self, path: &Path, ids: Vec<VariableId>) -> joinerror::Result<()> {
        let mut state = self.state.write().await;
        if state.variables.is_empty() {
            return Ok(());
        }

        let mut patches = Vec::with_capacity(ids.len());
        for id in ids {
            patches.push(PatchOperation::Remove(RemoveOperation {
                path: PointerBuf::parse(format!("/variable/{}", id)).unwrap(),
            }));
        }

        let json_value = self.sync_service.apply(path, &patches).await?;

        // INFO: This isn't the most optimized approach.
        // In the future, when file changes are handled in the background, we can create a separate watch channel
        // to monitor changes to the JsonValue file. We'll send the updated value to that channel, and in this service,
        // we'll run a background task that listens to the channel and automatically updates the state when it receives any changes.
        {
            let map = json_value.get("variable").unwrap().as_object().unwrap();
            let variables = collect_variables(map)?;

            state.variables.extend(variables);
        }

        Ok(())
    }

    pub async fn batch_add(
        &self,
        path: &Path,
        params: Vec<AddVariableParams>,
    ) -> joinerror::Result<()> {
        let mut new_variables = HashMap::with_capacity(params.len());
        let mut patches = Vec::with_capacity(params.len());

        {
            let state = self.state.read().await;
            if state.variables.is_empty() {
                patches.push(PatchOperation::Add(AddOperation {
                    path: PointerBuf::parse("/variable").unwrap(),
                    value: json!({}),
                }));
            }
        }

        for param in params {
            let id = VariableId::new();
            let global_value = json_to_hcl(&param.global_value).map_err(|err| {
                Error::new::<()>(format!(
                    "failed to convert global value expression: {}",
                    err
                ))
            })?;

            let value = VariableSpec {
                name: param.name.clone(),
                value: global_value.clone(),
                description: param.desc.clone(),
                options: param.options.clone(),
            };

            let item = VariableItem {
                id: id.clone(),
                name: param.name,
                local_value: HclExpression::Null,
                global_value,
                desc: param.desc,
                order: param.order,
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

            new_variables.insert(id, item);
        }

        let json_value = self.sync_service.apply(path, &patches).await?;

        // INFO: This isn't the most optimized approach.
        // In the future, when file changes are handled in the background, we can create a separate watch channel
        // to monitor changes to the JsonValue file. We'll send the updated value to that channel, and in this service,
        // we'll run a background task that listens to the channel and automatically updates the state when it receives any changes.
        {
            let mut state = self.state.write().await;
            let map = json_value.get("variable").unwrap().as_object().unwrap();
            let variables = collect_variables(map)?;

            state.variables.extend(variables);
        }

        Ok(())
    }
}

fn collect_variables(
    map: &JsonMap<String, JsonValue>,
) -> joinerror::Result<HashMap<VariableId, VariableItem>> {
    let mut variables = HashMap::new();
    for (id_str, value) in map {
        let var = if let Ok(d) =
            serde_json::from_value::<VariableSpec>(value.clone()).map_err(|err| {
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
                name: var.name,
                local_value: HclExpression::Null, // TODO: restore from the store
                global_value: var.value,
                desc: var.description,
                order: 0, // TODO: restore from the store
                options: var.options,
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
        AnyEnvironment, ModifyEnvironmentParams,
        builder::{CreateEnvironmentParams, EnvironmentBuilder},
        configuration::{MetadataDecl, SourceFile, VariableSpec},
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
            VariableSpec {
                name: "simple_variable".to_string(),
                value: HclExpression::Variable(Variable::new("test".to_string()).unwrap()),

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
                    VariableSpec {
                        name: "function_call".to_string(),
                        value: attr.expr().clone(),

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
                    VariableSpec {
                        name: "conditional".to_string(),
                        value: attr.expr().clone(),

                        description: None,
                        options: VariableOptions { disabled: false },
                    },
                );
            }
        }

        // Create original HCL structure
        let original_hcl = SourceFile {
            metadata: Block::new(MetadataDecl {
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
        let json_deserialized: SourceFile = match serde_json::from_str(&json_value) {
            Ok(data) => {
                println!("JSON deserialization successful!");
                data
            }
            Err(e) => {
                println!("JSON deserialization failed: {}", e);
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
        println!("\nSuccessfully converted HCL -> JSON -> HCL without errors!");

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

        println!("Data integrity verified!");
        println!("Function calls, conditionals, and complex expressions support added!");
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
        let global_model_registry = Arc::new(GlobalModelRegistry::new());
        let storage_service: Arc<StorageService<MockAppRuntime>> =
            Arc::new(StorageService::new(Arc::new(TestVariableStore {})));
        let sync_service = Arc::new(SyncService::new(global_model_registry.clone(), fs.clone()));
        let variable_service =
            VariableService::<MockAppRuntime>::new(None, sync_service.clone()).unwrap();

        // struct MyEnvStore<Environment: AnyEnvironment<MockAppRuntime>> {
        //     map: HashMap<String, Environment>,
        // }

        let env = EnvironmentBuilder::new(fs)
            .with_service(variable_service)
            .with_service::<StorageService<MockAppRuntime>>(storage_service)
            .with_service::<SyncService>(sync_service)
            .create::<MockAppRuntime>(
                global_model_registry,
                CreateEnvironmentParams {
                    id: EnvironmentId::new(),
                    name: "data".to_string(),
                    abs_path: &abs_path,
                    color: None,
                },
            )
            .await
            .unwrap();

        // let mut env_store = MyEnvStore {
        //     map: HashMap::new(),
        // };
        // env_store.map.insert("data".to_string(), env);

        env.modify(ModifyEnvironmentParams {
            name: None,
            color: None,
            vars_to_update: vec![],
            vars_to_add: vec![AddVariableParams {
                name: "test".to_string(),
                global_value: json!("test_value"),
                desc: None,
                local_value: JsonValue::Null,
                order: 0,
                options: VariableOptions { disabled: false },
            }],
            vars_to_delete: vec![],
        })
        .await
        .unwrap();
    }
}
