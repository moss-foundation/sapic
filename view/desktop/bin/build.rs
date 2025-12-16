use std::{env, fs, path::Path, process::Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let input_file = "../i18n/index.jsonnet";

    // Rerun this build script if the jsonnet file changes
    println!("cargo:rerun-if-changed={}", input_file);

    let jsonnet_cmd = if cfg!(target_os = "windows") {
        "jsonnet.exe"
    } else {
        "jsonnet"
    };

    let output = Command::new(jsonnet_cmd).arg(input_file).output();
    match output {
        Ok(output) => {
            if output.status.success() {
                let output_file = Path::new(&out_dir).join("main.i18n.json");
                fs::write(&output_file, output.stdout)
                    .expect("Failed to write generated JSON file");
            }
        }
        Err(e) => {
            panic!("jsonnet command failed: {}", e);
        }
    }

    tauri_build::build()
}

// TEST Update
