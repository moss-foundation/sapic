use std::{env, fs, path::Path, process::Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let input_file = "contrib/index.jsonnet";
    let output_file = Path::new(&out_dir).join(format!("{}.contrib.json", package_name));

    // Rerun this build script if the jsonnet file changes
    println!("cargo:rerun-if-changed={}", input_file);
    println!("cargo:rerun-if-changed=../../contrib/configuration.libsonnet");
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
                fs::write(&output_file, output.stdout)
                    .expect("Failed to write generated JSON file");

                println!("Generated configuration JSON at: {:?}", output_file);
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
