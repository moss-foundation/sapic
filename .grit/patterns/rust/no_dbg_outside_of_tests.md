---
title: No dbg!() outside test functions
level: error
---

```grit
language rust

`dbg!($_)` as $dbg where {
    $dbg <: not within `fn $_($_) {$_}` as $fn where {
        $fn <: after or {
            `#[test]`,
            `#[tokio::test]`,
        }
    }
}
```

## Test Cases

```rust
#[test]
fn test_case() {
    dbg!("Correct");
}


fn non_test_case() {
    dbg!("Incorrect");
}
```
