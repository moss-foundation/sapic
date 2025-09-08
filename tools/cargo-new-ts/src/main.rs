use std::{
    env, fs,
    path::Path,
    process::{Command, exit},
};

fn index_ts_content() -> String {
    "".to_string()
}

fn package_json_content(crate_name: &str) -> String {
    let package_name = format!("@repo/{crate_name}");

    // TODO: Looks like it's unnecessary to specify `typescript` and `zod` dependencies individually
    // Just putting them at the top-level package.json seems to be enough
    format!(
        r#"{{
  "name": "{}",
  "exports": {{
    ".": "./index.ts"
  }},
  "scripts": {{
    "test": "echo \"Error: no test specified\" && exit 1",
    "format": "prettier --plugin=prettier-plugin-tailwindcss --write \"**/*.{{ts,tsx,md}}\""
  }},
  "devDependencies": {{
    "@repo/moss-bindingutils": "workspace:*",
    "@repo/typescript-config": "workspace:*"
  }},
  "dependencies": {{
    "typescript": "^5.9.2",
    "zod": "^3.24.4"
  }}
}}"#,
        package_name
    )
}

fn tsconfig_json_content() -> String {
    r#"{
  "extends": "",
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
}"#
    .to_string()
}

fn main() {
    // collect arguments passed from cargo command
    let args: Vec<String> = env::args().skip(2).collect();

    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        println!("cargo new-ts <name>");
        return;
    }

    // `cargo new {crate_name} --lib
    let crate_name = &args[0];
    let mut cargo_new = Command::new("cargo");
    cargo_new.arg("new").arg("--lib").arg(crate_name);

    let status = cargo_new.status().expect("failed to invoke cargo new");
    if !status.success() {
        eprintln!("cargo new failed");
        exit(1);
    }

    let crate_dir = Path::new(crate_name);

    // Create `index.ts`, `package.json` and `tsconfig.json`
    fs::write(crate_dir.join("index.ts"), index_ts_content()).unwrap();
    fs::write(
        crate_dir.join("package.json"),
        package_json_content(crate_name),
    )
    .unwrap();
    fs::write(crate_dir.join("tsconfig.json"), tsconfig_json_content()).unwrap();
}
