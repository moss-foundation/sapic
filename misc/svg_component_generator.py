import argparse
import json
import re
from typing import Dict
from copy import deepcopy
import tinycss2
import webcolors
import xml.etree.ElementTree as ET
import os
from pathlib import Path
import hashlib

COLOR_ATTRIBUTES = {"fill", "stroke"}

# Color values that should not be translated
EXCLUDED_VALUES = {"none", "transparent"}

def compare_svg_tags(tag1: ET.Element, tag2: ET.Element) -> bool:
    if tag1.tag != tag2.tag:
        return False
    if tag1.text != tag2.text:
        return False
    if tag1.tail != tag2.tail:
        return False
    for attr in tag1.attrib:
        # Check the equality of non-color attributes
        if attr in COLOR_ATTRIBUTES:
            continue
        if tag1.attrib[attr] != tag2.attrib[attr]:
            return False
    return True


def compare_svgs(tree1: ET.ElementTree, tree2: ET.ElementTree) -> bool:
    """
    Check if two SVG files are structurally equivalent
    We compare the two trees after removing all their color attributes
    """
    for elem1, elem2 in zip(tree1.iter(), tree2.iter()):
        if not compare_svg_tags(elem1, elem2):
            return False

    return True

def convert_to_component(name: str, svg_content: str) -> str:
    """
    Convert a consolidated svg into a React component
    """
    # Inject the props into the opening <svg ...> tag
    svg_with_props = re.sub(
        r'<svg([^>]*)>',
        r'<svg\1 {...props}>',
        svg_content,
        count=1,  # only replace the first occurrence
    )
    return (
        f"""import * as React from "react";
import type {{ SVGProps }} from "react";
const Svg{name} = (props: SVGProps<SVGSVGElement>) => {svg_with_props};
export default Svg{name};"""
    )

def combine_svgs_to_component(name: str, light_svg: str, dark_svg: str) -> str:
    """
    Combine the light and dark version of svg into the same component
    This is done when the two svgs are structurally different
    """
    return (
        f"""import * as React from "react";
import type {{ SVGProps }} from "react";
const Svg{name} = (props: SVGProps<SVGSVGElement>) => {{
    return (
        <div {{...props}}>
            {{/* Light theme SVG */}}
            {light_svg}
            
            {{/* Dark theme SVG */}}
            {dark_svg}
        </div>
    );
}};
"""
    )


def plan(folder: str, force=False):
    """
    Generate a generation plan for a given folder of icons
    """
    # Get all icon folders
    icon_list = sorted(icon_name for icon_name in os.listdir(folder) if not Path(folder, icon_name).is_file())

    # Check if there's an update to the icon list
    hash_string = "".join(f"{len(name)}{name}" for name in icon_list)
    icon_hash = hashlib.md5(hash_string.encode()).hexdigest()
    lock_file = Path(folder, "plan.lock")
    if lock_file.exists() and lock_file.read_text() == icon_hash and not force:
        print("No changes to the icon list")
        return
    lock_file.write_text(icon_hash)

    # Update the generation plan
    plan_file = Path(folder, "plan.json")
    icon_plans = {}
    if plan_file.exists():
        icon_plans = json.loads(plan_file.read_text())

    plan_updated = False
    for icon_name in icon_list:
        if icon_name in icon_plans and not force:
            continue
        print(f"Updating the plan for {icon_name}")
        icon_plans[icon_name] = plan_icon(folder, icon_name)
        plan_updated = True

    if plan_updated:
        plan_file.write_text(json.dumps(icon_plans))


def plan_icon(folder: str, name: str) -> Dict[str, bool]:
    """
    Check if the given icon have two svgs that can be consolidated
    """
    light = ET.parse(os.path.join(folder, name, "light.svg"))
    dark = ET.parse(os.path.join(folder, name, "dark.svg"))
    # Check if two svgs are structurally the same
    if compare_svgs(light, dark):
        return {"identical": True}
    else:
        return {"identical": False}


def gen(folder: str,
        light_css_path: str,
        dark_css_path: str,
        output_path: str
        ):
    """
    Generate React components for all icons in a given folder
    Each icon has its own folder with two files: dark.svg and light.svg.
    """
    plan_file = Path(folder, "plan.json")
    icon_plans = json.loads(plan_file.read_text())
    light_dict = extract_css_palette(light_css_path)
    dark_dict = extract_css_palette(dark_css_path)

    for name, plan in icon_plans.items():
        # FIXME: should we use the names exported from figma like `Theme = Light.svg`?
        print(f"Generating {name}")
        light = ET.parse(os.path.join(folder, name, "light.svg"))
        dark = ET.parse(os.path.join(folder, name, "dark.svg"))

        component = None
        # If the two svg variants are structurally the same
        # Consolidate the light and dark colors into tailwind class definition
        if plan['identical']:
            result = consolidate_svg_colors(light, dark, light_dict, dark_dict)
            svg = ET.tostring(result.getroot(), encoding="unicode").replace("\n", "")
            component = convert_to_component(name, svg)
        # If the two svg files are structurally different
        # Put the two svg variants in a div and toggle the visibility of the two svg subtags
        else:
            light.getroot().set("className", "block dark:hidden")
            dark.getroot().set("className", "hidden dark:block")
            light_svg = ET.tostring(light.getroot(), encoding="unicode").replace("\n", "")
            dark_svg = ET.tostring(dark.getroot(), encoding="unicode").replace("\n", "")
            component = combine_svgs_to_component(name, light_svg, dark_svg)

        Path(output_path, f"{name}.tsx").open("w").write(component)

    # Export all the icons from the index.ts file
    index_content = "\n".join(f"export {{ default as {name} }} from './{name}'" for name in icon_plans)
    Path(output_path, "index.ts").write_text(index_content)


def normalize_color(color: str) -> str:
    """
    Normalize an SVG color into standard lowercase hexadecimal color
    """
    parsed_color = webcolors.html5_parse_legacy_color(color)
    return webcolors.rgb_to_hex(parsed_color)


def extract_css_palette(input_file: str) -> Dict[str, str]:
    """
    Extract icon colors from a css file into a dictionary, with hex values as key and variable name as value
    """
    # Extract CSS palette from a css file
    css_content = Path(input_file).read_text()
    rules = tinycss2.parse_stylesheet(
        css_content, skip_comments=True, skip_whitespace=True
    )

    # Find all palette color definitions
    # All palette colors ends with a numeric suffix
    icon_colors = [
        rule for rule in tinycss2.parse_blocks_contents(rules[0].content)
        if rule.type == "declaration" and rule.lower_name.split("-")[-1].isdigit()
    ]

    # Extract the hexadecimal value of the color declarations, and normalize it
    result = {}

    for rule in icon_colors:
        hash_tokens = [val for val in rule.value if val.type == "hash"]
        if len(hash_tokens) != 1:
            continue
        hex_value = hash_tokens[0].value
        normalized_hex = webcolors.normalize_hex(f"#{hex_value}")
        if normalized_hex not in result:
            result[normalized_hex] = rule.lower_name
        else:
            print(f"{normalized_hex} is already mapped to {result[normalized_hex]}")

    return result


def consolidate_svg_colors(
        light: ET.ElementTree,
        dark: ET.ElementTree,
        light_dict: Dict[str, str],
        dark_dict: Dict[str, str]) -> ET.ElementTree:
    """
    Translate SVG colors into variable names defined in a given color dictionary
    Then consolidate the light and dark versions into a same svg
    """
    result = deepcopy(light)

    # Match the corresponding color values in the light/dark tree
    # Translate the values into variables and write it as a tailwind class attribute in result tree
    for attr in COLOR_ATTRIBUTES:
        light_elems = light.findall(f"[@{attr}]") + light.findall(f".//*[@{attr}]")
        dark_elems = dark.findall(f"[@{attr}]") + dark.findall(f".//*[@{attr}]")
        result_elems = result.findall(f"[@{attr}]") + result.findall(f".//*[@{attr}]")

        for (light_elem, dark_elem, result_elem) in zip(light_elems, dark_elems, result_elems):
            light_color = light_elem.get(attr)
            dark_color = dark_elem.get(attr)

            # We should not change special color values
            if light_color in EXCLUDED_VALUES:
                continue

            light_color = normalize_color(light_color)
            if light_color not in light_dict:
                print(f"Unrecognized light color value '{light_color}'")
                continue

            dark_color = normalize_color(dark_color)
            if dark_color not in dark_dict:
                print(f"Unrecognized dark color value '{dark_color}'")
                continue

            light_var = light_dict[light_color]
            dark_var = dark_dict[dark_color]

            class_name = f"{attr}-[var({light_var})] dark:{attr}-[var({dark_var})]"

            # Remove the original attribute from the result tree
            # And append the consolidated tailwind classes
            del result_elem.attrib[attr]
            prev_class_name = result_elem.get("className")
            if prev_class_name is None:
                prev_class_name = ""

            new_class_name = f"{prev_class_name} {class_name}".strip()
            result_elem.set("className", new_class_name)

    return result



if __name__ == "__main__":
    ET.register_namespace("", "http://www.w3.org/2000/svg")
    parser = argparse.ArgumentParser(description="SVG React Component Generator")
    subparsers = parser.add_subparsers(dest="command")

    plan_parser = subparsers.add_parser("plan")
    plan_parser.add_argument("--folder", help="Path to the icons folder", required=True)
    plan_parser.add_argument("--force", help="Force regeneration of plans", action="store_true")

    gen_parser = subparsers.add_parser("gen")
    gen_parser.add_argument("--folder", help="Path to the icons Folder", required=True)
    gen_parser.add_argument("--light_css", help="Path to the light theme css file", required=True)
    gen_parser.add_argument("--dark_css", help="Path to the dark theme css file", required=True)
    gen_parser.add_argument("--output_path", help="Path to the output folder", required=True)

    kwargs = vars(parser.parse_args())


    if kwargs["command"] == "plan":
        plan(kwargs["folder"], kwargs["force"])
    elif kwargs["command"] == "gen":
        gen(kwargs["folder"], kwargs["light_css"], kwargs["dark_css"], kwargs["output_path"])
