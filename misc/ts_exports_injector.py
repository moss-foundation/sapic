#!/usr/bin/env python3

"""
TypeScript Exports Injector

This script automatically generates an index.ts file that exports all modules
from the bindings directory. This ensures that all generated models can be easily
imported from a single entry point.

The script scans the 'bindings' directory recursively and creates export statements
for all TypeScript files found, writing them to index.ts.

Usage:
    python ts_exports_injector.py

This script is called during model generation in the Makefile after imports
are injected to create a centralized export point for all models.
"""

import os
from pathlib import Path

if __name__ == "__main__":
    output = []
    with open("index.ts", mode="w") as f:
        for root, dirs, files in sorted(os.walk("bindings")):
            for file in files:
                import_path = "./" + root.replace("\\", "/") + "/" + Path(file).stem
                output.append(f'export * from "{import_path}";')
        f.write("\n".join(output) + "\n")
