#!/usr/bin/env python3
"""
Rust Warnings Checker Script

This script checks for Rust compiler warnings in a specific crate or entire workspace.
It uses `cargo check` with JSON output format to parse and analyze warnings.

Features:
- Check specific crate with -p/--package option
- Check entire workspace (default behavior)
- Exclude specific crates from workspace check with --exclude option
- Exit with error code 1 if any warnings are found
- Remove duplicate warning messages
- Clean output formatting with location information
- Filter warnings by package when using -p option

Usage Examples:
    # Check entire workspace
    python warnings_check.py
    
    # Check specific package
    python warnings_check.py -p moss_storage2
    
    # Check workspace excluding specific packages
    python warnings_check.py --exclude test-crate --exclude example-crate
    
    # Show help
    python warnings_check.py --help

Exit Codes:
    0 - No warnings found
    1 - One or more warnings found

Output:
    - Warnings are printed to stderr for better integration with CI/CD
    - Each unique warning is listed with bullet points and location info
    - Duplicate warnings are automatically deduplicated
"""

import subprocess
import json
import sys
import re
import argparse

# Paths to check when filtering warnings by package
PACKAGE_PATHS = [
    "crates/{package_dir}/",
    "plugins/{package_dir}/",
    "libs/{package_dir}/",
    "tools/{package_dir}/",
    "view/{package_dir}/bin/",
]

def main():
    parser = argparse.ArgumentParser(description="Check Rust warnings for a specific crate or entire workspace")
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

    # Build cargo check command
    cmd = ["cargo", "--locked", "check", "--message-format=json"]
    
    if args.package:
        cmd.extend(["-p", args.package])
        print(f"Checking package: {args.package}")
    else:
        print("Checking entire workspace")
        if args.exclude:
            cmd.append("--workspace")
            for exclude_pkg in args.exclude:
                cmd.extend(["--exclude", exclude_pkg])
            print(f"Excluding packages: {', '.join(args.exclude)}")

    cmd.append("--all-features")
    cmd.append("--all-targets")

    # Run cargo check in JSON mode
    proc = subprocess.run(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )

    # Parse output line by line
    warnings = []
    for line in proc.stdout.splitlines():
        try:
            obj = json.loads(line)
        except json.JSONDecodeError:
            continue

        if obj.get("message", {}).get("level") == "warning":
            msg = obj["message"]["message"]
            # Remove suffix like " (1 duplicate)"
            msg = re.sub(r" \(\d+ duplicate\)$", "", msg)
            
            # Extract location information
            spans = obj["message"].get("spans", [])
            location_info = ""
            file_name = ""
            if spans:
                primary_span = next((span for span in spans if span.get("is_primary", False)), spans[0])
                file_name = primary_span.get("file_name", "unknown")
                line_start = primary_span.get("line_start", "?")
                line_end = primary_span.get("line_end", "?")
                
                if line_start == line_end:
                    location_info = f" [{file_name}:{line_start}]"
                else:
                    location_info = f" [{file_name}:{line_start}-{line_end}]"
            
            # Filter warnings by package if -p option is used
            if args.package:
                # Convert package name from underscore to dash for directory matching
                package_dir = args.package.replace("_", "-")
                # Check multiple possible paths for the package
                if file_name and not any(path.format(package_dir=package_dir) in file_name for path in PACKAGE_PATHS):
                    continue  # Skip warnings not from the specified package
            
            warning_with_location = f"{msg}{location_info}"
            warnings.append(warning_with_location)

    unique_warnings = set(warnings)
    count = len(unique_warnings)

    if count > 0:
        print(f"Found {count} warning{'s' if count > 1 else ''}:", file=sys.stderr)
        for w in sorted(unique_warnings):
            print(f"  - {w}", file=sys.stderr)
        return 1
    else:
        print("No warnings found.")
        return 0

if __name__ == "__main__":
    sys.exit(main())
