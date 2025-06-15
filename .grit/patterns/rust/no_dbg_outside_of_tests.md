---
title: No dbg!() outside test functions
level: error
---

```grit
language rust

`dbg!($args)` where {
    $args <: not within `fn test_$_() {$_}`
}
```

## Test Cases

```rust
fn non_test() {
    dbg!("Incorrect");
}

fn test() {
    dbg!("Correct");
}
```
