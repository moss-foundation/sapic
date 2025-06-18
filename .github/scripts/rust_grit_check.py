import argparse
import json
import subprocess
import sys
from os import PathLike
from pathlib import Path
from typing import Dict


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
