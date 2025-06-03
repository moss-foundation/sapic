# Gen Bindings Tools

A comprehensive collection of TypeScript utilities for managing generated bindings, exports, imports, and Zod schemas. These tools work together to create a seamless workflow for TypeScript code generation and maintenance.

## Table of Contents

- [Overview](#overview)
- [Tools](#tools)
  - [TypeScript Exports Injector](#typescript-exports-injector)
  - [Imports Resolver](#imports-resolver)
  - [Imports Consolidator](#imports-consolidator)
  - [Zod Schema Generator](#zod-schema-generator)
- [Workflow](#workflow)
- [Development](#development)
- [Architecture](#architecture)
- [Contributing](#contributing)

## Overview

This toolkit provides four essential utilities for TypeScript binding generation and maintenance:

1. **🔧 Imports Resolver** - Fixes missing imports in generated TypeScript models
2. **📋 Zod Generator** - Converts TypeScript types to Zod schemas
3. **📤 Exports Injector** - Maintains export declarations in index files
4. **✨ Imports Consolidator** - Organizes and optimizes import statements

## Tools

### TypeScript Imports Resolver (`importsResolver.ts`)

Automatically resolves and adds missing imports to generated TypeScript models by analyzing dependencies and fixing import statements.

#### Usage

```bash
# Run the imports resolver
npm run importsResolver <path-to-target-directory>

# Example
npm run importsResolver ./crates/moss-workspace
```

#### How it works

1. 🏗️ Initializes TypeScript project with proper configuration
2. 📦 Discovers and adds available package source files
3. 🔍 Resolves source file dependencies
4. 🔧 Fixes missing imports for each source file
5. 💾 Saves all changes to the file system
6. 📊 Reports detailed results with metrics

---

### Zod Schema Generator (`zodGenerator.ts`)

Converts TypeScript models to Zod schemas using ts-to-zod and applies intelligent post-processing to handle imports and naming conventions.

#### Usage

```bash
# Run the Zod generator
npm run zodGenerator <path-to-target-directory>

# Example
npm run zodGenerator ./crates/moss-workspace
```

#### How it works

1. 🏗️ Validates target directory and bindings folder structure
2. 🔍 Discovers all TypeScript files (excluding existing .zod.ts files)
3. ⚙️ Runs ts-to-zod for each file to generate initial schemas
4. 🔧 Post-processes generated files to fix imports and naming
5. 📋 Organizes imports and saves the final Zod schema files
6. 📊 Reports comprehensive results with detailed statistics

---

### TypeScript Exports Injector (`tsExportsInjector.ts`)

Automatically generates and maintains export declarations for TypeScript binding files by scanning directories and updating index.ts files.

#### Usage

```bash
# Run the exports injector
npm run tsExportsInjector <path-to-target-directory>

# Example
npm run tsExportsInjector ./crates/moss-workspace
```

#### How it works

1. ✅ Validates target directory, index.ts, and bindings directory existence
2. 🔍 Discovers all TypeScript files in the bindings directory
3. 🗑️ Removes existing binding exports from index.ts
4. 📤 Adds new export declarations using `export * from` syntax
5. 📋 Organizes imports and exports for clean, consistent formatting
6. 💾 Saves the updated index.ts file

---

### TypeScript Imports Consolidator (`importsConsolidator.ts`)

Organizes and consolidates import statements in TypeScript files, automatically sorting, grouping, and optimizing imports according to best practices.

#### Usage

```bash
# Run the imports consolidator
npm run importsConsolidator <path-to-target-directory>

# Example
npm run importsConsolidator ./crates/moss-workspace
```

#### How it works

1. 🏗️ Initializes TypeScript project with proper configuration
2. ✅ Validates target paths and accessibility
3. 📁 Processes all source files in the project
4. 📋 Organizes imports for each file individually
5. ✨ Tracks optimization statistics (before/after import counts)
6. 💾 Saves all changes to the file system
7. 📊 Reports comprehensive consolidation results

## Architecture

The tools are built using modern TypeScript and these key technologies:

- **[ts-morph](https://ts-morph.com/)** - TypeScript compiler API wrapper for AST manipulation
- **[ts-to-zod](https://github.com/fabien0102/ts-to-zod)** - TypeScript to Zod schema generation
- **[zod](https://zod.dev/)** - TypeScript-first schema validation
- **Node.js built-in modules** - File system operations and path handling
