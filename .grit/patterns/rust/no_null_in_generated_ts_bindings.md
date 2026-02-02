---
title: TypeScript exports must not contain "null"
level: warn
---

```grit
language js(typescript)

r`null` as $null where {
    $filename <: r".*[\/\\]bindings[\/\\](?:.*\.ts|.*[\/\\]\.ts)",
    // List of exception types that are allowed to have | in their type definition.
    $null <: not within or {
        `export type JsonValue = $_`,
    }
}

```

## Test Cases

```typescript
// ✅ Correct nullable field (with #[ts(optional_fields)])
export type NullableType = {
  name?: string;
};

// ❌ Incorrect nullable field
export type NullableType = {
  name: string | null;
};
```
