import cssutils
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
        txt = Path(input_file).read_text()

        # Parse the CSS content
        css = cssutils.parseString(cssText=txt, validate=False)

        # Extract CSS variables from the first rule's style
        variables = [key for key in css.cssRules.item(0).style.keys() if key.startswith('--')]

        # Write the variables to the output file as a json array
        Path(output_file).write_text(json.dumps(variables))
        print(f"CSS variables successfully extracted to {output_file}")
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    input_file = '../assets/themes/light.css'
    output_file = '../packages/config-eslint/moss-lint-plugin/css_variables.json'
    extract_css_variables(input_file, output_file)