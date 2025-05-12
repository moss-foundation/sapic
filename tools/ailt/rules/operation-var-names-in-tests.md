# Test Variable Naming and Resource Cleanup Rule

## Metadata

```yaml
tags: [naming, best-practices, cleanup]
languages: [rust]
include:
  - "**/tests/**/*.rs"
```

## Rule Logic

```
// Match each test function
test_item() as $test where {
  // It must be annotated with #[test]
  $test <: within `#[test] fn $name() { $body }`

  // Within the body, there must be variable declarations with expected names:
  // 1. <operation_name>_result
  // 2. <operation_name>_output
  within $body {
    variable_declarator($undeployed) where { $undeployed.name.matches(r"^[a-z0-9_]+_result$") },
    variable_declarator($expanded)  where { $expanded.name.matches(r"^[a-z0-9_]+_output$") }
  },

  // Ensure that resources created during the test are cleaned up:
  // there must be at least one call expression to a cleanup function
  // (e.g. cleanup_resources(), teardown(), drop_*)
  within $body {
    call_expr($cleanup) where {
      $cleanup.callee.matches(r"^(cleanup|teardown|drop_).*$")
    }
  }
}
```

## Description

This rule enforces best practices in Rust test files located in any crate under a `tests/` directory. It ensures:

1. **Consistent variable naming for API results**

   - The _undeployed_ or raw result of an API operation must be assigned to a variable named  
     `<operation_name>_result` (e.g., `fetch_user_result`).
   - The _expanded_ or processed output must be stored in a variable named  
     `<operation_name>_output` (e.g., `fetch_user_output`).

2. **Proper cleanup of resources**  
   Each test must include at least one explicit cleanup call — such as `cleanup_*`, `teardown_*`, or `drop_*` — to ensure that resources created during the test are released appropriately.

This promotes test readability, debugging clarity, and avoids side effects between tests.
