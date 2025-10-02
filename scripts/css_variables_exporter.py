#!/usr/bin/env python3

"""
CSS Variables Exporter

This script extracts CSS custom properties (variables) from a theme JSON file and
exports them to a JSON file. This makes CSS variables available to other tools
such as linters and type checkers.

Usage:
    python css_variables_exporter.py --source [path-to-theme-json] --dest [path-to-output-json]

This script is called during the build process to make CSS theming variables
available throughout the project's toolchain.
"""

import tinycss2
import json
import argparse
from pathlib import Path

def convert_token(token: str):
    """
    Convert a token to css variable name
    For example:
    moss.gray.1 => --moss-gray-1
    """
    return "--" + "-".join(token.split("."))


def extract_css_variables(input_file: str, output_file: str):
    """
    Extracts CSS variables from a theme json file and writes them to a json file.

    Args:
        input_file (str): Path to the input CSS file.
        output_file (str): Path to the output json file where variables will be saved.
    """
    try:
        variables = []
        theme = json.loads(Path(input_file).read_text())

        for key in theme["palette"]:
            variables.append(convert_token(key))

        for key in theme["colors"]:
            variables.append(convert_token(key))

        for key in theme["boxShadows"]:
            variables.append(convert_token(key))

        # Write the variables to the output file as a json array
        Path(output_file).write_text(json.dumps(variables))
        print(f"CSS variables successfully extracted to {output_file}")

    except Exception as e:
        print(f"An error occurred: {e}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Extract CSS custom properties (variables) from a theme json file and export them to a JSON file."
    )
    parser.add_argument("--source", type=Path, required=True, help="Path to the input theme json file")
    parser.add_argument("--dest", type=Path, required=True, help="Path to the output json file")

    args = parser.parse_args()

    extract_css_variables(args.source, args.dest)
