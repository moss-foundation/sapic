import argparse
import fnmatch
import json
import os
import subprocess
import sys
from os import PathLike
from pathlib import Path
from typing import Dict, List

# Set GRIT_DEBUG=1 to see which files are being ignored
DEBUG = os.environ.get("GRIT_DEBUG", "0") == "1"


# Map of rule exceptions: rule name -> list of file paths to ignore
# Paths are relative to the project root and support glob patterns (*, ?, **)
#
# Examples:
#   - Exact file: "crates/some-crate/src/debug.rs"
#   - Directory (all files inside): "plugins/shared_storage/bindings/"
#   - Directory (without trailing slash): "plugins/shared_storage/bindings"
#   - Wildcard dir: "crates/*/src/temporary.rs" (matches any crate)
#   - Wildcard file: "crates/ipc/bindings/*.ts" (matches all .ts files)
#   - Recursive: "plugins/**/bindings/" (matches all files in any bindings folder)
#   - Multiple wildcards: "crates/*/tests/**/test_*.rs"
#
# NOTE: 
#   - Rule names must match exactly the grit rule name (local_name in grit output)
#   - Paths ending with / are treated as directories (all files inside are ignored)
#   - Paths without / but pointing to a directory will also work
#
RULE_EXCEPTIONS: Dict[str, List[str]] = {
    # "no_dbg_outside_of_tests": [
    #     "crates/some-crate/src/debug.rs",
    #     "crates/*/src/temporary.rs",
    # ],
    
    "no_null_in_generated_ts_bindings": [
        "plugins/shared_storage/bindings",
        "plugins/settings-storage/bindings",
    ],
}


def is_path_ignored(rule_name: str, file_path: str) -> bool:
    """
    Check if a specific file path should be ignored for a given rule.
    
    Args:
        rule_name: Name of the grit rule
        file_path: Path to the file (can be absolute or relative)
    
    Returns:
        True if the file should be ignored for this rule, False otherwise
    """
    if rule_name not in RULE_EXCEPTIONS:
        return False
    
    ignored_patterns = RULE_EXCEPTIONS[rule_name]
    file_path_obj = Path(file_path)
    
    for pattern in ignored_patterns:
        # Normalize the pattern to handle directory paths
        pattern_normalized = pattern
        
        # If pattern ends with /, it's a directory - match all files under it
        if pattern.endswith('/'):
            # Convert "dir/" to "dir/**/*" for matching
            pattern_normalized = pattern.rstrip('/') + '/**'
        
        # Try to match using Path.match() which supports ** patterns
        try:
            if file_path_obj.match(pattern_normalized):
                return True
        except ValueError:
            # If Path.match() fails, try fnmatch as fallback
            pass
        
        # Check if file_path starts with the pattern (for directory patterns)
        if str(file_path_obj).startswith(pattern.rstrip('/')):
            return True
        
        # Also try fnmatch for simple patterns
        if fnmatch.fnmatch(str(file_path_obj), pattern_normalized):
            return True
        
        # Try matching against the relative path if file_path is absolute
        if file_path_obj.is_absolute():
            try:
                # Try to get the relative path from the current working directory
                rel_path = file_path_obj.relative_to(Path.cwd())
                rel_path_str = str(rel_path)
                
                if rel_path.match(pattern_normalized):
                    return True
                if fnmatch.fnmatch(rel_path_str, pattern_normalized):
                    return True
                if rel_path_str.startswith(pattern.rstrip('/')):
                    return True
            except ValueError:
                # file_path is not relative to cwd
                pass
    
    return False


def parse_crate_paths() -> Dict[str, PathLike]:
    """
    Returns a mapping from crate name to its filesystem directory path.
    """
    cmd = ["cargo", "metadata", "--no-deps", "--format-version=1"]
    proc = subprocess.run(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )

    obj = json.loads(proc.stdout)
    crate_paths: Dict[str, Path] = {}
    for package in obj["packages"]:
        name = package["name"]
        crate_path = Path(package["manifest_path"]).parent
        crate_paths[name] = crate_path

    return crate_paths


def grit_check(args: argparse.Namespace) -> int:
    crate_list = parse_crate_paths()

    # Determine which crates to target for grit
    if args.package:
        target_crates = {args.package: crate_list[args.package]} if args.package in crate_list else {}
        if len(target_crates) == 0:
            print(f"Invalid crate: {args.package}", file=sys.stderr)
            return 1
    else:
        excludes = set(args.exclude or [])
        target_crates = {name: path for name, path in crate_list.items() if name not in excludes}

    failed = False
    for name, path in target_crates.items():
        print(f"Running grit check on crate '{name}' at {path}")
        cmd = [
            "grit", "check", str(path), "--json"
        ]

        proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        warnings = []
        errors = []

        res = proc.stderr
        try:
            obj = json.loads(res)
        except json.JSONDecodeError:
            continue

        for result in obj["results"]:
            pattern = result["local_name"]
            file_path = result["path"]
            
            # Check if this file should be ignored for this rule
            if is_path_ignored(pattern, file_path):
                if DEBUG:
                    print(f"[DEBUG] Ignoring {pattern} in {file_path}", file=sys.stderr)
                continue
            
            line_start = result["start"]["line"]
            line_end = result["end"]["line"]
            if line_start == line_end:
                location_info = f" [{file_path}:{line_start}]"
            else:
                location_info = f" [{file_path}:{line_start}-{line_end}]"

            message = f"{pattern}{location_info}"
            if result["extra"]["severity"] == "error":
                errors.append(message)
            elif result["extra"]["severity"] == "warn":
                warnings.append(message)

        # Both error and warn violations will fail the check
        errors_count = len(errors)
        if errors_count > 0:
            print(f"Found {errors_count} GritQL errors{'s' if errors_count > 1 else ''}:", file=sys.stderr)
            for error in sorted(errors):
                print(f"  - {error}", file=sys.stderr)
            failed = True

        warnings_count = len(warnings)
        if warnings_count > 0:
            print(f"Found {warnings_count} GritQL warnings{'s' if warnings_count > 1 else ''}:", file=sys.stderr)
            for warning in sorted(warnings):
                print(f"  - {warning}", file=sys.stderr)
            failed = True

    if failed:
        return 1
    else:
        print("No GritQL violations found")
        return 0


def main():
    parser = argparse.ArgumentParser(description="GritQL Check for a specific crate or entire workspace")
    parser.add_argument(
        "-p", "--package",
        help="Package to check (if not specified, checks entire workspace)"
    )
    parser.add_argument(
        "--exclude",
        action="append",
        help="Packages to exclude from workspace check (can be used multiple times)"
    )
    args = parser.parse_args()

    # Cargo Warnings Check
    return grit_check(args)


if __name__ == "__main__":
    sys.exit(main())
