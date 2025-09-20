This is the Sapic repository - one console for modern backends. Design, invoke, subscribe, and observe your APIs, queues, and lambdas, etc. with integration tests, mocks, and environments. AI & Git native with batteries included.

## Rust coding guidelines

- Prioritize code correctness and clarity. Speed and efficiency are secondary priorities unless otherwise specified.
- Do not write organizational or comments that summarize the code. Comments should only be written in order to explain "why" the code is written in some way in the case there is a reason that is tricky / non-obvious.
- Prefer implementing functionality in existing files unless it is a new logical component. Avoid creating many small files.
- Avoid creative additions unless explicitly requested

## Testing

- When adding new tests or modifying old ones, always run them to check the correctness of the changes being made.
- Always attempt to find related tests in an existing test file before creating a new test file
- Run the tests strictly in the crate where they are located and avoid running all the tests in the workspace.
- Be rigorous and test for edge-cases and unexpected inputs.

### Test Organization

- Integration tests for API operations must always be written in the tests folder located in the root folder of the crate.
- One integration test file should represent only one API operation.
- The name of an integration test file representing a single API operation should follow the format `{crate_name}__{api_operation_name}.rs`.
- Unit tests must always be written in the same file where the code being tested is located.

Follow these rules for writing tests unless something else is explicitly requested.

## Code Architecture

### Language Structure

- **Rust code**:
  - (`crates/*`): Core application crates with business logic, APIs, and backend functionality
  - (`libs/*`): Shared utility libraries and and functionality that isn’t specific to this particular project
  - (`view/desktop/bin`): Desktop application entry point and Tauri backend

- **TypeScript code**
  - (`packages/*`): App typescript libraries
  - (`view/desktop/src`): Desktop application frontend with React UI components and logic

- **TypeScript bindings code**:
  - (`crates/*/bindings`): TypeScript type definitions and bindings generated from Rust code
