use hcl::Body;

mod tests {
    use std::{path::Path, sync::Arc};

    use hcl::{
        Body,
        eval::{Evaluate, FuncDef, ParamType},
    };

    use crate::{plugin_world::Value as WasmValue, wasm_host::WasmHost};

    const PLUGIN_PATH: &'static str = "plugins";

    #[test]
    fn test_hcl() {
        let mut wasm_host = Arc::new(WasmHost::new(Path::new(PLUGIN_PATH)).unwrap());
        wasm_host.load_plugin("hcl_greeter").unwrap();

        let hcl_context = hcl::eval::Context::new();
        let func = FuncDef::builder()
            .param(ParamType::String)
            .build(move |arg| {
                let wasm_val: WasmValue = arg[0].try_into().map_err(|e| e.to_string())?;
                let output = wasm_host
                    .execute_plugin("hcl_greeter", wasm_val)
                    .map_err(|e| e.to_string())?;
                let output_hcl: hcl::Value = output.try_into().map_err(|e| e.to_string())?;
                Ok(output_hcl)
            });

        hcl_context.declare_func("hcl_greeter", func);

        let input = r#"message = hcl_greet("WASM")"#;
        let body: Body = hcl::from_str(input).unwrap();
        body.evaluate(ctx);
        dbg!(body);
    }
}
