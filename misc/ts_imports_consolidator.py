#!/usr/bin/env python3

"""
TypeScript Imports Consolidator

This script consolidates multiple import statements from the same source into a single import.
For example:
    import type { A } from "./types";
    import type { B } from "./types";
    import type { C } from "./types";

Will be consolidated into:
    import type { A, B, C } from "./types";

Usage:
    python ts_imports_consolidator.py <file_or_directory>

This script is called after model generation in the Makefile to clean up
and optimize TypeScript import statements.
"""

import os
import re
import sys
import glob
from collections import defaultdict


def consolidate_imports(file_path):
    with open(file_path, "r") as file:
        content = file.read()

    # Find all import statements
    import_pattern = re.compile(
        r'import\s+type\s+\{\s*([^}]+)\s*\}\s+from\s+[\'"]([^\'"]+)[\'"];', re.MULTILINE
    )
    imports = import_pattern.findall(content)

    # Group imports by source
    grouped_imports = defaultdict(list)
    for imported_items, source in imports:
        # Split multiple items in one import and clean whitespace
        items = [item.strip() for item in imported_items.split(",")]
        grouped_imports[source].extend(items)

    # Rebuild content with consolidated imports
    if grouped_imports:
        # Extract non-import content (comments, etc.)
        non_import_content = []
        lines = content.split("\n")
        in_import_block = False

        for line in lines:
            if import_pattern.search(line):
                in_import_block = True
            elif in_import_block and not line.strip():
                continue
            elif in_import_block:
                in_import_block = False
                non_import_content.append(line)
            else:
                non_import_content.append(line)

        # Rebuild the file content
        new_content = "\n".join(non_import_content[:5])  # Keep the header comments
        new_content += "\n\n"

        # Add consolidated imports
        for source, items in grouped_imports.items():
            # Remove duplicates while preserving order
            unique_items = []
            for item in items:
                if item not in unique_items:
                    unique_items.append(item)

            # Format the import statement
            if len(unique_items) > 3:
                # Multi-line format for many imports
                new_content += f"import type {{\n  {',\n  '.join(unique_items)}\n}} from \"{source}\";\n"
            else:
                # Single line for few imports
                new_content += (
                    f"import type {{ {', '.join(unique_items)} }} from \"{source}\";\n"
                )

        new_content += "\n"
        # Add the rest of the content
        new_content += "\n".join(non_import_content[5:]).lstrip()

        # Write back to the file if changes were made
        if new_content != content:
            with open(file_path, "w") as file:
                file.write(new_content)
            return True

    return False


def main():
    if len(sys.argv) > 1:
        # Process specific file or directory
        path = sys.argv[1]
        if os.path.isfile(path) and path.endswith(".ts"):
            changed = consolidate_imports(path)
            if changed:
                print(f"Consolidated imports in {path}")
        elif os.path.isdir(path):
            for ts_file in glob.glob(os.path.join(path, "**", "*.ts"), recursive=True):
                changed = consolidate_imports(ts_file)
                if changed:
                    print(f"Consolidated imports in {ts_file}")
    else:
        print("Usage: python ts_imports_consolidator.py <file_or_directory>")
        print("Consolidates multiple imports from the same source in TypeScript files.")
        sys.exit(1)


if __name__ == "__main__":
    main()
