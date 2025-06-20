---
title: TypeScript export must use correct .ts file for models files
level: error
tags: ["unix_only"]
---

```grit
language rust

`#[ts(export, export_to = $export_file)]` where {
    or {
        // types files must export to "types.ts"
        and {
            $filename <: r".*\/models\/types(?:\.rs|\/.*\.rs)$",
            $export_file <: not `"types.ts"`
        },
        // operations files must export to "operations.ts"
        and {
            $filename <: r".*\/models\/operations(?:\.rs|\/.*\.rs)$",
            $export_file <: not `"operations.ts"`
        },
        // primitives files must export to "primitives.ts"
        and {
            $filename <: r".*\/models\/primitives(?:\.rs|\/.*\.rs)$",
            $export_file <: not `"primitives.ts"`
        },
        // events files must export to "events.ts"
        and {
            $filename <: r".*\/models\/events(?:\.rs|\/.*\.rs)$",
            $export_file <: not `"events.ts"`
        }
    }
}
```

## Test Cases

```rust
// ✅ Correct usage in models/types.rs
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "types.ts")]
pub struct TypesStruct {
    pub id: String,
}

// ✅ Correct usage in models/operations.rs
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct OperationsStruct {
    pub name: String,
}

// ✅ Correct usage in models/primitives/config.rs
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "primitives.ts")]
pub struct PrimitivesStruct {
    pub value: i32,
}

// ✅ Correct usage in models/events/handler.rs
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "events.ts")]
pub struct EventsStruct {
    pub event_type: String,
}

// ❌ Incorrect usage in models/types.rs - wrong file name
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "other.ts")]
pub struct WrongTypesStruct {
    pub id: String,
}

// ❌ Incorrect usage in models/operations.rs - wrong file name
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "types.ts")]
pub struct WrongOperationsStruct {
    pub name: String,
}

// ❌ Incorrect usage in models/primitives/config.rs - wrong file name
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "config.ts")]
pub struct WrongPrimitivesStruct {
    pub value: i32,
}

// ❌ Incorrect usage in models/events/handler.rs - wrong file name
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "handlers.ts")]
pub struct WrongEventsStruct {
    pub event_type: String,
}
```
