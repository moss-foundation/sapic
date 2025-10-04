// /// Since we don't want to commit wasm artifacts into the repo
// /// We will mark these tests #[ignore] for manual testing

// #[cfg(test)]
// mod tests {
//     use std::{
//         path::Path,
//         sync::{Arc, Mutex, OnceLock},
//     };

//     use hcl::{
//         Body, Expression,
//         eval::{Context, Evaluate, FuncDef, ParamType},
//     };

//     use crate::{plugin_world::Value as WasmValue, wasm_host::WasmHost};

//     const PLUGIN_PATH: &'static str = "plugins";

//     static WASM_HOST: OnceLock<Arc<Mutex<WasmHost>>> = OnceLock::new();

//     fn hcl_greeter_function(arg: hcl::eval::FuncArgs) -> Result<hcl::Value, String> {
//         let wasm_val: WasmValue = arg[0]
//             .clone()
//             .try_into()
//             .map_err(|e: anyhow::Error| e.to_string())?;

//         let wasm_host = WASM_HOST.get().ok_or("WasmHost not initialized")?;

//         let host_guard = wasm_host
//             .lock()
//             .map_err(|_| "Failed to acquire WasmHost lock")?;

//         let output = host_guard
//             .execute_plugin("hcl_greeter", wasm_val)
//             .map_err(|e| e.to_string())?;

//         let output_hcl: hcl::Value = output
//             .try_into()
//             .map_err(|e: anyhow::Error| e.to_string())?;
//         Ok(output_hcl)
//     }

//     #[test]
//     #[ignore]
//     fn test_wasm_hcl_integration() {
//         let mut wasm_host = WasmHost::new(Path::new(PLUGIN_PATH)).unwrap();
//         wasm_host.load_plugin("hcl_greeter").unwrap();

//         WASM_HOST
//             .set(Arc::new(Mutex::new(wasm_host)))
//             .map_err(|_| "Failed to initialize WasmHost")
//             .unwrap();

//         let mut hcl_context = Context::new();
//         let func = FuncDef::builder()
//             .param(ParamType::String)
//             .build(hcl_greeter_function);

//         hcl_context.declare_func("hcl_greeter", func);

//         let input = r#"message = hcl_greeter("WASM")"#;
//         let body: Body = hcl::from_str(input).unwrap();
//         let result = body.evaluate(&hcl_context).unwrap();

//         assert_eq!(
//             result.attributes().next().unwrap().expr,
//             Expression::from("Hello, WASM!")
//         );
//         dbg!(&result);
//     }

//     fn echo_function(arg: hcl::eval::FuncArgs) -> Result<hcl::Value, String> {
//         let wasm_val: WasmValue = arg[0]
//             .clone()
//             .try_into()
//             .map_err(|e: anyhow::Error| e.to_string())?;

//         let wasm_host = WASM_HOST.get().ok_or("WasmHost not initialized")?;

//         let host_guard = wasm_host
//             .lock()
//             .map_err(|_| "Failed to acquire WasmHost lock")?;

//         let output = host_guard
//             .execute_plugin("echo", wasm_val)
//             .map_err(|e| e.to_string())?;

//         let output_hcl: hcl::Value = output
//             .try_into()
//             .map_err(|e: anyhow::Error| e.to_string())?;
//         Ok(output_hcl)
//     }

//     #[test]
//     #[ignore]
//     fn test_data_roundtrip() {
//         // Testing passing a string across WASM boundary
//         // If this works correctly, we can theoretically use json string to pass data
//         let mut wasm_host = WasmHost::new(Path::new(PLUGIN_PATH)).unwrap();
//         wasm_host.load_plugin("echo").unwrap();

//         // Rust->WASM->Rust
//         let data = "TestData".to_string();
//         let data_from_wasm = wasm_host
//             .execute_plugin("echo", WasmValue::Str(data.clone()))
//             .unwrap()
//             .as_str()
//             .unwrap()
//             .to_string();

//         assert_eq!(data, data_from_wasm);

//         // HCL->WASM->HCL
//         WASM_HOST
//             .set(Arc::new(Mutex::new(wasm_host)))
//             .map_err(|_| "Failed to initialize WasmHost")
//             .unwrap();

//         let mut hcl_context = Context::new();
//         let func = FuncDef::builder()
//             .param(ParamType::String)
//             .build(echo_function);

//         let input = r#"message = echo(data)"#;
//         hcl_context.declare_func("echo", func);
//         hcl_context.declare_var("data", data_from_wasm);

//         let body: Body = hcl::from_str(input).unwrap();
//         let result = body.evaluate(&hcl_context).unwrap();

//         assert_eq!(
//             result.attributes().next().unwrap().expr,
//             Expression::from(data)
//         );
//     }
// }
