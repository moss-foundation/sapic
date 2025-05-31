#!/usr/bin/env python3

"""
TypeScript Exports Injector

This script automatically updates an index.ts file to export all modules
from the bindings directory while preserving all other existing content.
This ensures that all generated models can be easily imported from a single 
entry point without losing any manual imports or other code.

The script scans the 'bindings' directory recursively and creates export statements
for all TypeScript files found, replacing only the bindings-related exports
in index.ts while keeping everything else intact.

Usage:
    python ts_exports_injector.py

This script is called during model generation in the Makefile after imports
are injected to create a centralized export point for all models.
"""

import os
import re
from pathlib import Path

def generate_bindings_exports():
    """Generate export statements for all files in the bindings directory."""
    exports = []
    
    # Get all TypeScript files and sort them for consistent output
    ts_files = []
    for root, dirs, files in os.walk("bindings"):
        for file in files:
            if file.endswith('.ts'):
                import_path = "./" + root.replace("\\", "/") + "/" + Path(file).stem
                ts_files.append(import_path)
    
    # Sort files to ensure consistent order
    ts_files.sort()
    
    for import_path in ts_files:
        exports.append(f'export * from "{import_path}";')
    
    return exports

def update_index_file():
    """Update index.ts file, replacing only bindings exports while preserving other content."""
    index_file = "index.ts"
    
    # Read existing content if file exists
    existing_lines = []
    if os.path.exists(index_file):
        with open(index_file, 'r') as f:
            existing_lines = f.readlines()
    
    # Filter out existing bindings exports
    # Pattern matches: export * from "./bindings/...";
    bindings_export_pattern = re.compile(r'^export \* from ["\']\.\/bindings\/.*["\'];?\s*$')
    non_bindings_lines = []
    
    for line in existing_lines:
        if not bindings_export_pattern.match(line):
            non_bindings_lines.append(line.rstrip('\n'))
    
    # Remove trailing empty lines from non-bindings content
    while non_bindings_lines and non_bindings_lines[-1] == '':
        non_bindings_lines.pop()
    
    # Generate new bindings exports
    new_bindings_exports = generate_bindings_exports()
    
    # Combine content
    final_content = []
    
    # Add existing non-bindings content
    if non_bindings_lines:
        final_content.extend(non_bindings_lines)
        final_content.append('')  # Add empty line before bindings exports
    
    # Add new bindings exports
    final_content.extend(new_bindings_exports)
    
    # Write updated content
    with open(index_file, 'w') as f:
        f.write('\n'.join(final_content) + '\n')

if __name__ == "__main__":
    update_index_file()
