---
title: No stuff between test directive and function
level: error
---

```grit
language rust

r"#\[.*test\]" as $directive where {
    $directive <: before $decl where {
        $decl <: not or {
            `$_ fn $_($_) {$_}`,
            // Make it handle tests with return type
            `$_ fn $_($_) -> $_ {$_}`,
        }
    }
}
```

## Test Cases

```rust
#[test]
fn correct_case() {}

#[test]
// Nothing allowed here
fn incorrect_case() {}

```
