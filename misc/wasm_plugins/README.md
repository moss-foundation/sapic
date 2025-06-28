For JS and Python

1. Install wit-deps:
   cargo install wit-deps-cli
2. Declare upstream WIT dependencies in deps.toml
   plugin-base = "../../../../crates/moss-wasm/wit"
3. Update dependencies when upstream WIT changes
   Delete deps/ and deps.lock
   run wit-dept

JavaScript

1. Install jco
   npm install -g @bytecodealliance/jco
   npm install -g @bytecodealliance/componentize-js
2. Write plugin WIT in the wit/ folder
3. Generate TypeScript types
   jco types wit -o types
4. Export js component to wasm
   jco componentize index.js -o js_demo.wasm -w wit/

Python

1. Install componentize-py
   pip install componentize-py
2. Write plugin WIT in the wit/ folder
3. Generate Python bindings
   componentize-py bindings bindings
4. Export python component to wasm
   componentize-py -d wit -w python-demo componentize app -o python_demo.wasm

Rust
Apparently Cargo workspace will not work correctly when you have nested members, thus all rust plugins are placed under
misc/rust_wasm_plugins at the root level

1. Install cargo-component and wasm-tools
   cargo install --locked cargo-component
2. Scaffolding a component
   cargo component new rust_demo --lib --namespace plugin
3. Add dependency on plugin:base in Cargo.toml
   [package.metadata.component.target.dependencies]
   # Relative path to the "wit" folder
   "plugin:base" = { path = "../../../crates/moss-wasm/wit"}
4. Update rust_demo/wit/world.wit
5. Generate bindings
   cargo component bindings
6. Implement `Guest` trait on Component in `lib.rs`
7. Build component
   cargo component build --release
   The output will be in the target folder at the repo root
