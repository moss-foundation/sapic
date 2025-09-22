use std::{env, fs, path::Path, process::Command};

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let jsonnet_file = "contrib/index.jsonnet";
    let output_file = Path::new(&out_dir).join("contrib.json");

    // Rerun this build script if the jsonnet file changes
    println!("cargo:rerun-if-changed={}", jsonnet_file);
    println!("cargo:rerun-if-changed=../../contrib/configuration.libsonnet");
    println!("cargo:rerun-if-changed=../../contrib/index.libsonnet");

    let jsonnet_cmd = if cfg!(target_os = "windows") {
        "jsonnet.exe"
    } else {
        "jsonnet"
    };
    let output = Command::new(jsonnet_cmd).arg(jsonnet_file).output();

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
