import tinycss2
import json
from pathlib import Path

def extract_css_variables(input_file: str, output_file: str):
    """
    Extracts CSS variables from a CSS file and writes them to a json file.

    Args:
        input_file (str): Path to the input CSS file.
        output_file (str): Path to the output json file where variables will be saved.
    """
    try:
        # Read the CSS file content
        css_content = Path(input_file).read_text()

        # Parse the stylesheet rules
        rules = tinycss2.parse_stylesheet(css_content, skip_comments=True, skip_whitespace=True)

        # Parse the declarations in the root rule
        declarations = tinycss2.parse_blocks_contents(rules[0].content)

        # Extract the variables
        variables = [decl.name for decl in declarations if decl.type == "declaration" and decl.lower_name.startswith("--")]

        # Write the variables to the output file as a json array
        Path(output_file).write_text(json.dumps(variables))
        print(f"CSS variables successfully extracted to {output_file}")

    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    input_file = '../assets/themes/light.css'
    output_file = '../packages/config-eslint/moss-lint-plugin/css_variables.json'
    extract_css_variables(input_file, output_file)