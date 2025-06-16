---
title: No dbg!() outside test functions
level: error
---

```grit
language rust

`dbg!($_)` as $dbg where {
    $dbg <: not within `fn $name($_) {$_}`
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
