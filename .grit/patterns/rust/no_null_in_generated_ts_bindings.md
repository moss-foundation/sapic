---
title: TypeScript exports must not contain "null"
level: error
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

// ✅ Correct - JsonValue в списке исключений, может содержать null
export type JsonValue = number | string | boolean | Array<JsonValue> | { [key in string]?: JsonValue } | null;

// ❌ Incorrect nullable field (не в списке исключений)
export type SomeType = {
  name: string | null;
};

// ❌ Incorrect union type with null (не в списке исключений)
export type MyUnion = string | number | null;
```
