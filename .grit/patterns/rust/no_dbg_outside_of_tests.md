---
title: No dbg!() outside test functions
level: error
---

```grit
language rust

`dbg!($_)` as $dbg where {
    $dbg <: not within $fn where {
        $fn <: after or {
            `#[test]`,
            `#[tokio::test]`,
        },
        $fn <: or {
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
fn test_case() {
    dbg!("Correct");
}


fn non_test_case() {
    dbg!("Incorrect");
}
```
