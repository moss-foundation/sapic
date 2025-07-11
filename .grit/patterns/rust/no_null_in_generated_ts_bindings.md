---
title: TypeScript exports must not contain "null"
level: error
---

```grit
language js(typescript)

r`null` where {
    $filename <: r".*[\/\\]bindings[\/\\](?:.*\.ts|.*[\/\\]\.ts)"
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
