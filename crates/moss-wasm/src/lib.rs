mod plugin_world;

use anyhow::Result;
use plugin::base::types::{
    Number as WasmNumber, SimpleValue as WasmSimpleValue, Value as WasmValue,
};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs};
use wasmtime::{
    AsContextMut, Config, Engine, Store,
    component::{Component, HasSelf, Instance, Linker, ResourceTable},
};
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView};

use crate::plugin_world::{PluginWorld, plugin};

const PLUGIN_PATH: &'static str = "plugins";

enum ArtifactError {
    NoArtifact,
    NoLockfile,
    MismatchingHash,
}

pub struct WasiHostCtx {
    wasi_ctx: WasiCtx,
    wasi_table: ResourceTable,
    // TODO: Other context for plugins
}

impl WasiHostCtx {
    pub fn new() -> Self {
        Self {
            wasi_ctx: WasiCtx::builder().inherit_stdio().build(),
            wasi_table: ResourceTable::new(),
        }
    }
}

impl IoView for WasiHostCtx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.wasi_table
    }
}

impl WasiView for WasiHostCtx {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

impl plugin::base::host_functions::Host for WasiHostCtx {
    fn greet(&mut self, content: WasmValue) -> () {
        match content {
            WasmValue::Null => println!("Hello Null!"),
            WasmValue::Boolean(b) => println!("Hello Bool {b}!"),
            WasmValue::Num(number) => match number {
                WasmNumber::Float(f) => println!("Hello Float {f}!"),
                WasmNumber::Signed(i) => println!("Hello Signed {i}!"),
                WasmNumber::Unsigned(u) => println!("Hello Unsigned {u}!"),
            },
            WasmValue::Str(s) => println!("Hello String {s}!"),
            WasmValue::Arr(simple_values) => {
                println!("Hello Array with length {}!", simple_values.len())
            }
            WasmValue::Obj(items) => println!(
                "Hello Object with the following keys: {}",
                items
                    .into_iter()
                    .map(|item| item.0)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}

impl plugin::base::types::Host for WasiHostCtx {}

pub struct AddonInstance {
    instance: Instance,
    store: Store<WasiHostCtx>,
}

impl AddonInstance {
    pub fn new(instance: Instance, store: Store<WasiHostCtx>) -> Self {
        Self { instance, store }
    }
}

// TODO: Caching of compiled wasm component artifacts
pub struct WasmHost {
    hasher: Sha256,
    engine: Engine,
    linker: Linker<WasiHostCtx>,
    addon_registry: HashMap<String, AddonInstance>,
}

impl WasmHost {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);
        // Adding WASI apis to the linker
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

        // Implement the host functions used by plugins
        PluginWorld::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;

        Ok(Self {
            hasher: Sha256::new(),
            engine,
            linker,
            addon_registry: HashMap::new(),
        })
    }

    pub fn load_addon(&mut self, addon_name: &str) -> Result<()> {
        // If a compiled artifact exists, load it,
        // Otherwise, compile the addon and save the artifact
        // Then, instantiate the addon into the registry
        let component;
        match self.load_artifact(addon_name) {
            Ok(artifact_bytes) => unsafe {
                component = Component::deserialize(&self.engine, &artifact_bytes)?;
            },

            Err(_) => {
                component = self.compile_addon(addon_name, vec![])?;
                self.save_artifact(addon_name, &component)?;
            }
        }
        let addon_instance = self.instantiate_component(component)?;
        self.addon_registry
            .insert(addon_name.to_string(), addon_instance);
        Ok(())
    }

    fn execute_addon(&mut self, addon_name: &str, val: WasmValue) -> Result<()> {
        let addon_instance = self.addon_registry.get_mut(addon_name).unwrap();
        let mut store = &mut addon_instance.store;
        let func = addon_instance
            .instance
            .get_typed_func::<(WasmValue,), (WasmValue,)>(&mut store, "execute")?;
        func.call(&mut store, (val,))?;
        func.post_return(&mut store)?;
        Ok(())
    }
}

impl WasmHost {
    fn load_artifact(&mut self, addon_name: &str) -> Result<Vec<u8>, ArtifactError> {
        let artifact_bytes = fs::read(format!("{}/{}.component", PLUGIN_PATH, addon_name))
            .map_err(|_| ArtifactError::NoArtifact)?;
        self.hasher.update(&artifact_bytes);
        let artifact_hash = format!("{:X}", self.hasher.finalize_reset());
        if let Ok(stored_hash) = fs::read_to_string(format!("{}/{}.lock", PLUGIN_PATH, addon_name))
        {
            if artifact_hash == stored_hash {
                Ok(artifact_bytes)
            } else {
                Err(ArtifactError::MismatchingHash)
            }
        } else {
            Err(ArtifactError::NoLockfile)
        }
    }

    fn instantiate_component(&mut self, component: Component) -> Result<AddonInstance> {
        let mut store = Store::new(&self.engine, WasiHostCtx::new());
        let instance = self
            .linker
            .instantiate(store.as_context_mut(), &component)?;
        Ok(AddonInstance::new(instance, store))
    }

    fn compile_addon(&mut self, addon_name: &str, _permissions: Vec<String>) -> Result<Component> {
        let wasm_bytes = fs::read(format!("{}/{}.wasm", PLUGIN_PATH, addon_name))?;
        Component::new(&self.engine, wasm_bytes)
    }

    fn save_artifact(&mut self, addon_name: &str, component: &Component) -> Result<()> {
        let component_bytes = component.serialize()?;
        fs::write(
            format!("{}/{}.component", PLUGIN_PATH, addon_name),
            &component_bytes,
        )?;
        self.hasher.update(&component_bytes);
        let addon_hash = format!("{:X}", self.hasher.finalize_reset());
        fs::write(format!("{}/{}.lock", PLUGIN_PATH, addon_name), addon_hash)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_demo() {
        let mut host = WasmHost::new().unwrap();
        host.load_addon("js_demo").unwrap();
        let val_null = WasmSimpleValue::Null;
        host.execute_addon("js_demo", val_null.clone().into())
            .unwrap();

        let val_bool = WasmSimpleValue::Boolean(true);
        host.execute_addon("js_demo", val_bool.clone().into())
            .unwrap();

        let val_num_unsigned = WasmSimpleValue::Num(WasmNumber::Unsigned(10000));
        host.execute_addon("js_demo", val_num_unsigned.clone().into())
            .unwrap();

        // FIXME: negative integers does not work for JavaScript addons
        let val_num_signed = WasmSimpleValue::Num(WasmNumber::Signed(-10000));
        // host.execute_addon("js_demo", val_num_signed.clone().into() ).unwrap();

        let val_num_float = WasmSimpleValue::Num(WasmNumber::Float(3.14));
        host.execute_addon("js_demo", val_num_float.clone().into())
            .unwrap();

        let val_string = WasmSimpleValue::Str("The Answer".to_string());
        host.execute_addon("js_demo", val_string.clone().into())
            .unwrap();

        let val_arr = WasmValue::Arr(vec![
            val_null.clone(),
            val_bool.clone(),
            val_num_unsigned.clone(),
            val_num_signed.clone(),
            val_num_float.clone(),
            val_string.clone(),
        ]);
        host.execute_addon("js_demo", val_arr.clone()).unwrap();

        let val_obj = WasmValue::Obj(vec![
            ("Null".to_string(), val_null.clone()),
            ("Bool".to_string(), val_bool.clone()),
            ("Unsigned".to_string(), val_num_unsigned.clone()),
            ("Signed".to_string(), val_num_signed.clone()),
            ("Float".to_string(), val_num_float.clone()),
            ("String".to_string(), val_string.clone()),
        ]);
        host.execute_addon("js_demo", val_obj.clone()).unwrap();
    }

    #[test]
    fn test_python_demo() {
        let mut host = WasmHost::new().unwrap();
        host.load_addon("python_demo").unwrap();
        let val_null = WasmSimpleValue::Null;
        host.execute_addon("python_demo", val_null.clone().into())
            .unwrap();

        let val_bool = WasmSimpleValue::Boolean(true);
        host.execute_addon("python_demo", val_bool.clone().into())
            .unwrap();

        let val_num_unsigned = WasmSimpleValue::Num(WasmNumber::Unsigned(10000));
        host.execute_addon("python_demo", val_num_unsigned.clone().into())
            .unwrap();

        let val_num_signed = WasmSimpleValue::Num(WasmNumber::Signed(-10000));
        host.execute_addon("python_demo", val_num_signed.clone().into())
            .unwrap();

        let val_num_float = WasmSimpleValue::Num(WasmNumber::Float(3.14));
        host.execute_addon("python_demo", val_num_float.clone().into())
            .unwrap();

        let val_string = WasmSimpleValue::Str("The Answer".to_string());
        host.execute_addon("python_demo", val_string.clone().into())
            .unwrap();

        let val_arr = WasmValue::Arr(vec![
            val_null.clone(),
            val_bool.clone(),
            val_num_unsigned.clone(),
            val_num_signed.clone(),
            val_num_float.clone(),
            val_string.clone(),
        ]);
        host.execute_addon("python_demo", val_arr.clone()).unwrap();

        let val_obj = WasmValue::Obj(vec![
            ("Null".to_string(), val_null.clone()),
            ("Bool".to_string(), val_bool.clone()),
            ("Unsigned".to_string(), val_num_unsigned.clone()),
            ("Signed".to_string(), val_num_signed.clone()),
            ("Float".to_string(), val_num_float.clone()),
            ("String".to_string(), val_string.clone()),
        ]);
        host.execute_addon("python_demo", val_obj.clone()).unwrap();
    }
}
