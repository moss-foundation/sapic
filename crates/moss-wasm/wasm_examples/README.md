1. Install wit-deps:
   cargo install wit-deps-cli
2. Update dependencies when upstream WIT changes
   Delete deps/ and deps.lock
   run wit-dept

JavaScript

1. Install jco
   npm install -g @bytecodealliance/jco
   npm install -g @bytecodealliance/componentize-js
2. Generate TypeScript types
   jco types wit -o types
3. Export js component to wasm
   jco componentize index.js -o js_demo.wasm -w wit/

Python

1. Install componentize-py
   pip install componentize-py
2. Generate Python bindings
   componentize-py bindings bindings
3. Export python component to wasm
   componentize-py -d wit -w python-demo componentize app -o python_demo.wasm
