use serde_json::json;
use std::{
    cell::LazyCell,
    collections::HashMap,
    env, fs,
    path::Path,
    process::{Command, exit},
};

/// These files will have the same content regardless of what the crate name is
const PREDEFINED_FILES: LazyCell<HashMap<String, Vec<u8>>> = LazyCell::new(|| {
    [
        ("index.ts".into(), "".as_bytes().to_vec()),
        (
            "tsconfig.json".into(),
            serde_json::to_vec_pretty(&json!({
              "extends": "@repo/typescript-config/base.json",
              "compilerOptions": {
                "composite": true,
                "tsBuildInfoFile": "./node_modules/.tmp/tsconfig.node.tsbuildinfo",
                "skipLibCheck": true,
                "lib": ["ES2020", "DOM", "DOM.Iterable"],
                "module": "ESNext",
                "moduleResolution": "bundler",
                "allowSyntheticDefaultImports": true,
                "strict": true,
                "noEmit": true
              },
              "include": ["**/*.ts"],
              "exclude": ["node_modules"]
            }))
            .unwrap(),
        ),
    ]
    .into_iter()
    .collect()
});

const PACKAGE_JSON_TEMPLATE: LazyCell<serde_json::Value> = LazyCell::new(|| {
    json!(
          {
      "name": "@repo/{}",
      "exports": {
        ".": "./index.ts"
      },
      "scripts": {
        "test": "echo \"Error: no test specified\" && exit 1",
        "format": "prettier --plugin=prettier-plugin-tailwindcss --write \"**/*.{ts,tsx,md}\""
      },
      "devDependencies": {
        "@repo/typescript-config": "workspace:*",
        "@repo/moss-bindingutils": "workspace:*",
        "@repo/moss-workspace": "workspace:*"
      },
      "dependencies": {
        "typescript": "^5.9.2",
        "zod": "^3.25.32"
      }
    }
      )
});

fn package_json_content(crate_name: &str) -> Vec<u8> {
    let package_name = format!("@repo/{crate_name}");

    let mut value = (*PACKAGE_JSON_TEMPLATE).clone();
    *value.get_mut("name").unwrap() = package_name.into();

    serde_json::to_vec_pretty(&value).unwrap()
}

fn main() {
    // collect arguments passed from cargo command
    let args: Vec<String> = env::args().skip(2).collect();

    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        println!("cargo new-ts <name> [options]");
        return;
    }

    // `cargo new <name> [options]
    let crate_name = &args[0];
    let mut cargo_new = Command::new("cargo");
    cargo_new.arg("new");

    for arg in &args {
        cargo_new.arg(&arg);
    }

    let crate_dir = Path::new(crate_name);
    let status = cargo_new.status().expect("failed to invoke cargo new");
    if !status.success() || !crate_dir.exists() {
        eprintln!("cargo new failed");
        exit(1);
    }

    // Create files that have static content
    for (file, content) in PREDEFINED_FILES.iter() {
        fs::write(crate_dir.join(file), content).unwrap();
    }

    // Create package.json with the correct package name
    fs::write(
        crate_dir.join("package.json"),
        package_json_content(crate_name),
    )
    .unwrap();
}
