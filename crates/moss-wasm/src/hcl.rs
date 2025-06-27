use hcl::Body;

mod tests {
    use std::{
        path::Path,
        sync::{Arc, Mutex, OnceLock},
    };

    use hcl::{
        Body,
        eval::{Context, Evaluate, FuncDef, ParamType},
    };

    use crate::{plugin_world::Value as WasmValue, wasm_host::WasmHost};

    const PLUGIN_PATH: &'static str = "plugins";

    static WASM_HOST: OnceLock<Arc<Mutex<WasmHost>>> = OnceLock::new();

    fn hcl_greeter_function(arg: hcl::eval::FuncArgs) -> Result<hcl::Value, String> {
        let wasm_val: WasmValue = arg[0]
            .clone()
            .try_into()
            .map_err(|e: anyhow::Error| e.to_string())?;

        let wasm_host = WASM_HOST.get().ok_or("WasmHost not initialized")?;

        let host_guard = wasm_host
            .lock()
            .map_err(|_| "Failed to acquire WasmHost lock")?;

        let output = host_guard
            .execute_plugin("hcl_greeter", wasm_val)
            .map_err(|e| e.to_string())?;

        let output_hcl: hcl::Value = output
            .try_into()
            .map_err(|e: anyhow::Error| e.to_string())?;
        Ok(output_hcl)
    }

    #[test]
    fn test_hcl() {
        let mut wasm_host = WasmHost::new(Path::new(PLUGIN_PATH)).unwrap();
        wasm_host.load_plugin("hcl_greeter").unwrap();

        WASM_HOST
            .set(Arc::new(Mutex::new(wasm_host)))
            .map_err(|_| "Failed to initialize WasmHost")
            .unwrap();

        let mut hcl_context = Context::new();
        let func = FuncDef::builder()
            .param(ParamType::String)
            .build(hcl_greeter_function);

        hcl_context.declare_func("hcl_greeter", func);

        let input = r#"message = hcl_greeter("WASM")"#;
        let body: Body = hcl::from_str(input).unwrap();
        let result = body.evaluate(&hcl_context);
        dbg!(&result);
    }
}
