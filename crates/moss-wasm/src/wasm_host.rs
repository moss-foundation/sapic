use anyhow::{Result, anyhow};
use plugin::base::types::{
    Number as WasmNumber, SimpleValue as WasmSimpleValue, Value as WasmValue,
};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use wasmtime::{
    AsContextMut, Config, Engine, Store,
    component::{Component, HasSelf, Instance, Linker, ResourceTable},
};
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use crate::plugin_world::{PluginWorld, plugin};

enum LoadError {
    NoArtifact,
    NoLockfile,
    MismatchingHash,
}

// TODO: Customize the capabilities of each plugin using contexts
pub struct WasiHostCtx {
    wasi: WasiCtx,
    http: WasiHttpCtx,
    resources: ResourceTable,
    // TODO: Other context for plugins
}

impl WasiHostCtx {
    pub fn new() -> Self {
        // By default, the plugins will not get any special permissions
        Self {
            wasi: WasiCtx::builder().inherit_stdio().build(),
            http: WasiHttpCtx::new(),
            resources: ResourceTable::new(),
        }
    }
}

impl IoView for WasiHostCtx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resources
    }
}

impl WasiView for WasiHostCtx {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

impl WasiHttpView for WasiHostCtx {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
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

pub struct PluginInstance {
    plugin: PluginWorld,
    store: Store<WasiHostCtx>,
}

impl PluginInstance {
    pub fn new(plugin: PluginWorld, store: Store<WasiHostCtx>) -> Self {
        Self { plugin, store }
    }
}

// TODO: Caching of compiled wasm component artifacts
pub struct WasmHost {
    plugin_path: PathBuf,
    engine: Engine,
    linker: Linker<WasiHostCtx>,
    plugin_registry: HashMap<String, PluginInstance>,
    hasher: Sha256,
}

impl WasmHost {
    pub fn new(plugin_path: &Path) -> Result<Self> {
        let mut config = Config::new();
        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);
        // Adding WASI apis to the linker
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

        // Adding WASI http apis to the linker
        wasmtime_wasi_http::add_only_http_to_linker_sync(&mut linker)?;

        // Implement the host functions used by plugins
        PluginWorld::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;

        Ok(Self {
            plugin_path: plugin_path.to_path_buf(),
            hasher: Sha256::new(),
            engine,
            linker,
            plugin_registry: HashMap::new(),
        })
    }

    pub fn load_plugin(&mut self, plugin_name: &str) -> Result<()> {
        // If a compiled artifact exists and matches with the saved hash, load it,
        // Otherwise, compile the plugin and save the artifact
        // Then, instantiate the plugin into the registry
        let component;
        match self.load_artifact(plugin_name) {
            Ok(artifact_bytes) => unsafe {
                component = Component::deserialize(&self.engine, &artifact_bytes)?;
            },
            Err(_) => {
                component = self.compile_plugin(plugin_name, vec![])?;
                self.save_artifact(plugin_name, &component)?;
            }
        }
        let plugin_instance = self.instantiate_component(component)?;
        self.plugin_registry
            .insert(plugin_name.to_string(), plugin_instance);
        Ok(())
    }

    fn execute_plugin(&mut self, plugin_name: &str, val: WasmValue) -> Result<WasmValue> {
        let plugin_instance = self
            .plugin_registry
            .get_mut(plugin_name)
            .ok_or(anyhow!("Plugin {plugin_name} is not registered"))?;

        let mut store = &mut plugin_instance.store;
        let output = plugin_instance.plugin.call_execute(store, &val)?;
        Ok(output)
    }
}

impl WasmHost {
    // FIXME: Right now we are only verifying the integrity of the artifact
    // If the artifact does not match with the hash, we will recompile it
    // But we might also want to force recompilation when the wasm binary changes

    fn load_artifact(&mut self, plugin_name: &str) -> Result<Vec<u8>, LoadError> {
        let artifact_path = self
            .plugin_path
            .join(plugin_name)
            .with_extension("component");
        let lockfile_path = self.plugin_path.join(plugin_name).with_extension("lock");

        let artifact_bytes = fs::read(&artifact_path).map_err(|_| LoadError::NoArtifact)?;

        self.hasher.update(&artifact_bytes);

        let combined_hash = format!("{:X}", self.hasher.finalize_reset());
        if let Ok(stored_hash) = fs::read_to_string(&lockfile_path) {
            if combined_hash == stored_hash {
                Ok(artifact_bytes)
            } else {
                Err(LoadError::MismatchingHash)
            }
        } else {
            Err(LoadError::NoLockfile)
        }
    }

    fn instantiate_component(&mut self, component: Component) -> Result<PluginInstance> {
        let mut store = Store::new(&self.engine, WasiHostCtx::new());
        let plugin = PluginWorld::instantiate(store.as_context_mut(), &component, &self.linker)?;

        Ok(PluginInstance::new(plugin, store))
    }

    fn compile_plugin(
        &mut self,
        plugin_name: &str,
        _permissions: Vec<String>,
    ) -> Result<Component> {
        let wasm_path = self.plugin_path.join(plugin_name).with_extension("wasm");
        let wasm_bytes = fs::read(&wasm_path)?;
        Component::new(&self.engine, wasm_bytes)
    }

    fn save_artifact(&mut self, plugin_name: &str, component: &Component) -> Result<()> {
        let artifact_bytes = component.serialize()?;
        let artifact_path = self
            .plugin_path
            .join(plugin_name)
            .with_extension("component");
        fs::write(&artifact_path, &artifact_bytes)?;
        self.hasher.update(&artifact_bytes);
        let plugin_hash = format!("{:X}", self.hasher.finalize_reset());
        let lockfile_path = self.plugin_path.join(plugin_name).with_extension("lock");

        fs::write(&lockfile_path, plugin_hash)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLUGIN_PATH: &'static str = "plugins";

    #[test]
    fn test_js_demo() {
        let mut host = WasmHost::new(Path::new(PLUGIN_PATH)).unwrap();
        host.load_plugin("js_demo").unwrap();
        let val_null = WasmSimpleValue::Null;
        host.execute_plugin("js_demo", val_null.clone().into())
            .unwrap();

        let val_bool = WasmSimpleValue::Boolean(true);
        host.execute_plugin("js_demo", val_bool.clone().into())
            .unwrap();

        let val_num_unsigned = WasmSimpleValue::Num(WasmNumber::Unsigned(10000));
        host.execute_plugin("js_demo", val_num_unsigned.clone().into())
            .unwrap();

        // FIXME: negative integers does not work for JavaScript plugins
        let val_num_signed = WasmSimpleValue::Num(WasmNumber::Signed(-10000));
        // host.execute_plugin("js_demo", val_num_signed.clone().into() ).unwrap();

        let val_num_float = WasmSimpleValue::Num(WasmNumber::Float(3.14));
        host.execute_plugin("js_demo", val_num_float.clone().into())
            .unwrap();

        let val_string = WasmSimpleValue::Str("The Answer".to_string());
        host.execute_plugin("js_demo", val_string.clone().into())
            .unwrap();

        let val_arr = WasmValue::Arr(vec![
            val_null.clone(),
            val_bool.clone(),
            val_num_unsigned.clone(),
            val_num_signed.clone(),
            val_num_float.clone(),
            val_string.clone(),
        ]);
        host.execute_plugin("js_demo", val_arr.clone()).unwrap();

        let val_obj = WasmValue::Obj(vec![
            ("Null".to_string(), val_null.clone()),
            ("Bool".to_string(), val_bool.clone()),
            ("Unsigned".to_string(), val_num_unsigned.clone()),
            ("Signed".to_string(), val_num_signed.clone()),
            ("Float".to_string(), val_num_float.clone()),
            ("String".to_string(), val_string.clone()),
        ]);
        host.execute_plugin("js_demo", val_obj.clone()).unwrap();
    }

    #[test]
    fn test_python_demo() {
        let mut host = WasmHost::new(Path::new(PLUGIN_PATH)).unwrap();
        host.load_plugin("python_demo").unwrap();
        let val_null = WasmSimpleValue::Null;
        host.execute_plugin("python_demo", val_null.clone().into())
            .unwrap();

        let val_bool = WasmSimpleValue::Boolean(true);
        host.execute_plugin("python_demo", val_bool.clone().into())
            .unwrap();

        let val_num_unsigned = WasmSimpleValue::Num(WasmNumber::Unsigned(10000));
        host.execute_plugin("python_demo", val_num_unsigned.clone().into())
            .unwrap();

        let val_num_signed = WasmSimpleValue::Num(WasmNumber::Signed(-10000));
        host.execute_plugin("python_demo", val_num_signed.clone().into())
            .unwrap();

        let val_num_float = WasmSimpleValue::Num(WasmNumber::Float(3.14));
        host.execute_plugin("python_demo", val_num_float.clone().into())
            .unwrap();

        let val_string = WasmSimpleValue::Str("The Answer".to_string());
        host.execute_plugin("python_demo", val_string.clone().into())
            .unwrap();

        let val_arr = WasmValue::Arr(vec![
            val_null.clone(),
            val_bool.clone(),
            val_num_unsigned.clone(),
            val_num_signed.clone(),
            val_num_float.clone(),
            val_string.clone(),
        ]);
        host.execute_plugin("python_demo", val_arr.clone()).unwrap();

        let val_obj = WasmValue::Obj(vec![
            ("Null".to_string(), val_null.clone()),
            ("Bool".to_string(), val_bool.clone()),
            ("Unsigned".to_string(), val_num_unsigned.clone()),
            ("Signed".to_string(), val_num_signed.clone()),
            ("Float".to_string(), val_num_float.clone()),
            ("String".to_string(), val_string.clone()),
        ]);
        host.execute_plugin("python_demo", val_obj.clone()).unwrap();
    }

    #[test]
    fn test_rust_demo() {
        let mut host = WasmHost::new(Path::new(PLUGIN_PATH)).unwrap();
        host.load_plugin("rust_demo").unwrap();
        let val_null = WasmSimpleValue::Null;
        host.execute_plugin("rust_demo", val_null.clone().into())
            .unwrap();

        let val_bool = WasmSimpleValue::Boolean(true);
        host.execute_plugin("rust_demo", val_bool.clone().into())
            .unwrap();

        let val_num_unsigned = WasmSimpleValue::Num(WasmNumber::Unsigned(10000));
        host.execute_plugin("rust_demo", val_num_unsigned.clone().into())
            .unwrap();

        let val_num_signed = WasmSimpleValue::Num(WasmNumber::Signed(-10000));
        host.execute_plugin("rust_demo", val_num_signed.clone().into())
            .unwrap();

        let val_num_float = WasmSimpleValue::Num(WasmNumber::Float(3.14));
        host.execute_plugin("rust_demo", val_num_float.clone().into())
            .unwrap();

        let val_string = WasmSimpleValue::Str("The Answer".to_string());
        host.execute_plugin("rust_demo", val_string.clone().into())
            .unwrap();

        let val_arr = WasmValue::Arr(vec![
            val_null.clone(),
            val_bool.clone(),
            val_num_unsigned.clone(),
            val_num_signed.clone(),
            val_num_float.clone(),
            val_string.clone(),
        ]);
        host.execute_plugin("rust_demo", val_arr.clone()).unwrap();

        let val_obj = WasmValue::Obj(vec![
            ("Null".to_string(), val_null.clone()),
            ("Bool".to_string(), val_bool.clone()),
            ("Unsigned".to_string(), val_num_unsigned.clone()),
            ("Signed".to_string(), val_num_signed.clone()),
            ("Float".to_string(), val_num_float.clone()),
            ("String".to_string(), val_string.clone()),
        ]);
        host.execute_plugin("rust_demo", val_obj.clone()).unwrap();
    }
}
