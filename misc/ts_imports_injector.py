#!/usr/bin/env python3
import os
import re
import sys
import json
from collections import defaultdict
from typing import Dict, List, Set, Tuple

auto_generated_comment = """\
// This file was generated by misc/importer.py. Do not edit this file manually.
//
// The necessary import statements have been automatically added by a Python script.
// This ensures that all required dependencies are correctly referenced and available
// within this module.
//
// If you need to add or modify imports, please update the package.json and
// re-run `make gen-models` it to regenerate the file accordingly.
"""

CUSTOM_SECTION_KEY = "tsImportRules"  # Unique key in package.json for the import map


def load_import_map(import_map_path: str) -> Dict[str, Dict[str, Dict[str, any]]]:
    """
    Loads the import_map from the custom section in package.json.

    Args:
        import_map_path (str): Path to the package.json file.

    Returns:
        Dict[str, Dict[str, Dict[str, any]]]: The parsed import_map from the custom section.

    Raises:
        FileNotFoundError: If the file does not exist.
        KeyError: If the custom section key is not found in package.json.
        json.JSONDecodeError: If the file contains invalid JSON.
    """
    if not os.path.isfile(import_map_path):
        raise FileNotFoundError(f"[ERROR] The file '{import_map_path}' does not exist.")

    with open(import_map_path, "r", encoding="utf-8") as f:
        package_json = json.load(f)

    if CUSTOM_SECTION_KEY not in package_json:
        raise KeyError(
            f"[ERROR] '{CUSTOM_SECTION_KEY}' section not found in {import_map_path}."
        )

    return package_json[CUSTOM_SECTION_KEY]


def parse_existing_imports(content: str) -> Dict[str, Set[str]]:
    """
    Parses existing import statements in a TypeScript file.

    Args:
        content (str): The content of the TypeScript file.

    Returns:
        Dict[str, Set[str]]: A dictionary mapping import paths to sets of imported types.
    """
    import_pattern = re.compile(
        r'import\s+(type\s+)?\{\s*([^}]+)\s*\}\s+from\s+[\'"]([^\'"]+)[\'"];?'
    )
    imports = defaultdict(set)
    for match in import_pattern.finditer(content):
        is_type_import = bool(match.group(1))
        types_str = match.group(2)
        import_path = match.group(3)
        types = {t.strip() for t in types_str.split(",") if t.strip()}
        imports[(import_path, is_type_import)].update(types)
    return imports


def generate_import_line(
    import_path: str, types: Set[str], is_type_import: bool
) -> str:
    """
    Generates an import statement line.

    Args:
        import_path (str): The module path to import from.
        types (Set[str]): A set of types to import.
        is_type_import (bool): Whether to use 'import type' or 'import'.

    Returns:
        str: The formatted import statement.
    """
    import_keyword = "import type" if is_type_import else "import"
    types_formatted = ", ".join(sorted(types))
    return f"{import_keyword} {{ {types_formatted} }} from '{import_path}';"


def add_imports_to_content(
    content: str, imports_to_add: Dict[Tuple[str, bool], Set[str]]
) -> str:
    """
    Adds necessary import statements to the TypeScript file content.

    Args:
        content (str): The original content of the TypeScript file.
        imports_to_add (Dict[Tuple[str, bool], Set[str]]):
            A dictionary mapping (import_path, is_type_import) to sets of types to import.

    Returns:
        str: The updated content with added import statements.
    """
    if not imports_to_add:
        return content  # No imports to add

    # Generate import lines
    new_import_lines = [
        generate_import_line(path, types, is_type_import)
        for (path, is_type_import), types in imports_to_add.items()
    ]

    # Find the position to insert new imports (after existing imports)
    lines = content.splitlines()
    insert_position = 0
    for i, line in enumerate(lines):
        if line.startswith("import "):
            insert_position = i + 1
        elif line.strip() == "":
            continue
        else:
            break

    # Insert new imports
    updated_lines = (
        lines[:insert_position] + new_import_lines + [""] + lines[insert_position:]
    )
    return "\n".join(updated_lines)


def process_file(file_path: str, import_details: Dict[str, Dict[str, any]]):
    """
    Processes a single TypeScript file to add necessary imports.

    Args:
        file_path (str): The path to the TypeScript file.
        import_details (Dict[str, Dict[str, any]]):
            A dictionary mapping import paths to their types and import kind.
    """
    if not os.path.isfile(file_path):
        print(f"[ERROR] File not found: {file_path}")
        return

    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()

    lines = content.split("\n")
    if lines:
        first_line = lines[0].strip()
        if first_line.startswith("//") or first_line.startswith("/*"):
            print(f"[INFO] Removing the first comment line from a file: {file_path}")
            lines = lines[1:]
            content = "\n".join(lines)

    existing_imports = parse_existing_imports(content)

    # Determine which imports need to be added
    imports_to_add = defaultdict(set)
    for import_path, details in import_details.items():
        types = set(details.get("types", []))
        is_type_import = details.get("is_type_import", True)
        existing_types = existing_imports.get((import_path, is_type_import), set())
        missing_types = types - existing_types
        if missing_types:
            imports_to_add[(import_path, is_type_import)].update(missing_types)

    if not imports_to_add:
        print(f"[INFO] No imports to add for file: {file_path}")
        return

    updated_content = add_imports_to_content(content, imports_to_add)

    # Adding an expanded auto-generated comment
    updated_content = f"{auto_generated_comment}\n{updated_content}"

    with open(file_path, "w", encoding="utf-8") as f:
        f.write(updated_content)

    # Log added imports
    for (path, is_type_import), types in imports_to_add.items():
        import_kind = "type import" if is_type_import else "import"
        print(
            f"[ADDED] {import_kind} for types {sorted(types)} from '{path}' in {file_path}"
        )


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(
            "[ERROR] Please provide the path to the package.json file as a CLI argument."
        )
        sys.exit(1)

    package_json_path = sys.argv[1]

    try:
        import_map = load_import_map(package_json_path)
    except (FileNotFoundError, KeyError, json.JSONDecodeError) as e:
        print(f"[ERROR] Failed to load import_map: {e}")
        sys.exit(1)

    for file_path, import_details in import_map.items():
        print(f"Processing file: {file_path}")
        process_file(file_path, import_details)
        print("-" * 50)
