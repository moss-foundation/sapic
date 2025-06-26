1. Install wit-deps:
   cargo install wit-deps-cli
2. Update dependencies
   run wit-deps at wasm_examples/js_demo
3. Export js component to wasm
   jco componentize index.js -o js_demo.wasm -w wit/
4. Generate TypeScript types
   jco types wit -o types
