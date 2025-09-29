use serde_json::Value as JsonValue;
use std::{collections::HashMap, env, fs, path::Path, process::Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let input_file = "contrib/index.jsonnet";

    // Rerun this build script if the jsonnet file changes
    println!("cargo:rerun-if-changed={}", input_file);
    println!("cargo:rerun-if-changed=../../contrib/configuration.libsonnet");
    println!("cargo:rerun-if-changed=../../contrib/resource.libsonnet");
    println!("cargo:rerun-if-changed=../../contrib/index.libsonnet");

    let jsonnet_cmd = if cfg!(target_os = "windows") {
        "jsonnet.exe"
    } else {
        "jsonnet"
    };
    let output = Command::new(jsonnet_cmd).arg(input_file).output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let parsed: HashMap<String, JsonValue> = serde_json::from_slice(&output.stdout)
                    .expect("Failed to parse generated JSON file");

                for (key, value) in parsed {
                    let output_file = Path::new(&out_dir).join(format!("{}.json", key));
                    fs::write(&output_file, serde_json::to_string_pretty(&value).unwrap())
                        .expect("Failed to write generated JSON file");
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                panic!("jsonnet command failed: {}", stderr);
            }
        }
        Err(e) => {
            panic!("jsonnet command failed: {}", e);
        }
    }
}
