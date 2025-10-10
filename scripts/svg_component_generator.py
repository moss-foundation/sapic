import argparse
import json
import logging
import hashlib
from pathlib import Path
from typing import Dict, Optional
from copy import deepcopy
import xml.etree.ElementTree as ET
import tinycss2
import webcolors
import re
import shutil

# Constants
COLOR_ATTRIBUTES = {"fill", "stroke"}
EXCLUDED_VALUES = {"none", "transparent"}
EXCLUDED_ATTRIBUTES = {"width", "height"}
SVG_NAMESPACE = "http://www.w3.org/2000/svg"


class CustomFormatter(logging.Formatter):
    """
    Formatter for colored logging output
    """
    grey    = "\x1b[38;21m"
    yellow  = "\x1b[33;21m"
    red     = "\x1b[31;21m"
    bold_red= "\x1b[31;1m"
    reset   = "\x1b[0m"
    format  = "[%(asctime)s] %(levelname)s: %(message)s"

    FORMATS = {
        logging.DEBUG: grey + format + reset,
        logging.INFO:  grey + format + reset,
        logging.WARNING: yellow + format + reset,
        logging.ERROR: red + format + reset,
        logging.CRITICAL: bold_red + format + reset
    }

    def format(self, record):
        log_fmt = self.FORMATS.get(record.levelno)
        formatter = logging.Formatter(log_fmt, datefmt="%Y-%m-%d %H:%M:%S")
        return formatter.format(record)

def setup_logging(verbose: bool = False) -> None:
    level = logging.DEBUG if verbose else logging.INFO
    handler = logging.StreamHandler()
    handler.setFormatter(CustomFormatter())
    root = logging.getLogger()
    root.setLevel(level)
    root.addHandler(handler)

def normalize_color(color: str) -> str:
    """
    Normalize an SVG color into standard lowercase six-digit hexadecimal.
    """
    rgb = webcolors.html5_parse_legacy_color(color)
    return webcolors.rgb_to_hex(rgb)

def to_camel_case(snake_str):
    return "".join(x.capitalize() for x in snake_str.lower().split("-"))

def to_lower_camel_case(snake_str):
    # We capitalize the first letter of each component except the first one
    # with the 'capitalize' method and join them together.
    camel_string = to_camel_case(snake_str)
    return snake_str[0].lower() + camel_string[1:]

def convert_token(token: str):
    """
    Convert a token to css variable name
    For example:
    moss.gray.1 => --moss-gray-1
    """
    return "--" + "-".join(token.split("."))


def extract_palette_from_json(json_path: Path) -> Dict[str, str]:
    """
    Parse a theme JSON file and extract custom property names for colors.
    Returns mapping from normalized hex value to CSS variable name.
    """
    logging.debug(f"Reading JSON palette from %s", json_path)
    palette_json = json.loads(json_path.read_text())["palette"]
    palette: Dict[str, str] = {}
    for key, item in palette_json.items():
        hex_code = webcolors.normalize_hex(item["value"])
        palette[hex_code] = convert_token(key)
        logging.debug("Mapped %s → %s", hex_code, convert_token(key))

    return palette

def append_svg_props(svg_xml: str) -> str:
    """
    Append props to the svg tag
    """
    return re.sub(r'<svg([^>]*)>', r'<svg\1 {...props}>', svg_xml, count=1)


def compare_svg_structure(a: ET.ElementTree, b: ET.ElementTree) -> bool:
    """
    Determine if two SVG trees are structurally identical, ignoring fill/stroke values.
    """

    # Shortcut if the two svg have different number of elements
    if sum(1 for _ in a.iter()) != sum(1 for _ in b.iter()):
        return False
    for elem_a, elem_b in zip(a.iter(), b.iter()):
        if elem_a.tag != elem_b.tag:
            return False
        # Compare non-color attributes
        attrs_a = {k: v for k, v in elem_a.attrib.items() if k not in COLOR_ATTRIBUTES}
        attrs_b = {k: v for k, v in elem_b.attrib.items() if k not in COLOR_ATTRIBUTES}
        if attrs_a != attrs_b:
            return False
        if (elem_a.text or "").strip() != (elem_b.text or "").strip():
            return False
    return True


def consolidate_svg(
        svg_path: Path,
        light_tree: ET.ElementTree,
        dark_tree: ET.ElementTree,
        light_palette: Dict[str, str],
        dark_palette: Dict[str, str]
) -> ET.ElementTree | None:
    """
    Consolidate identical-structured light and dark svg into a single tree
    """
    result = deepcopy(light_tree)

    # Ensure namespace registration
    ET.register_namespace("", SVG_NAMESPACE)

    light_elements = list(light_tree.iter())
    dark_elements = list(dark_tree.iter())
    res_elements = list(result.iter())

    # Replace raw color attribute with consolidated tailwind class
    for le, de, relem in zip(light_elements, dark_elements, res_elements):
        for attr in COLOR_ATTRIBUTES:
            lc = le.attrib.get(attr)
            dc = de.attrib.get(attr)
            if not lc or not dc or lc in EXCLUDED_VALUES:
                continue

            lc_norm = normalize_color(lc)
            if lc_norm not in light_palette:
                logging.error("Unrecognized color %s in file %s", lc, svg_path / "light.svg")
                return None
            light_var = light_palette[lc_norm]

            dc_norm = normalize_color(dc)
            if dc_norm not in dark_palette:
                logging.error("Unrecognized color %s in file %s", dc, svg_path / "dark.svg")
                return None
            dark_var = dark_palette[dc_norm]

            class_str = f"{attr}-[var({light_var})] dark:{attr}-[var({dark_var})]"
            prev = relem.attrib.pop(attr, None)
            existing = relem.attrib.get("className", "")
            combined = " ".join(filter(None, [existing, class_str]))
            relem.set("className", combined)

    return result


def merge_svg_to_component(
        svg_path: Path,
        name: str,
        light_tree: ET.ElementTree,
        dark_tree: ET.ElementTree,
        light_palette: Dict[str, str],
        dark_palette: Dict[str, str]
) -> str | None:
    """
    Merge light and dark svg with different structures into a component
    """
    # Separate rendering for light/dark
    light_el = light_tree.getroot()
    dark_el = dark_tree.getroot()
    light_el.set('className', 'block dark:hidden')
    dark_el.set('className', 'hidden dark:block')

    light_elements = list(light_tree.iter())
    dark_elements = list(dark_tree.iter())

    # Replace raw color attribute with tailwind class
    for le, de in zip(light_elements, dark_elements):
        for attr in COLOR_ATTRIBUTES:
            lc = le.attrib.get(attr)
            dc = de.attrib.get(attr)
            if not lc or not dc or lc in EXCLUDED_VALUES:
                continue

            lc_norm = normalize_color(lc)
            if lc_norm not in light_palette:
                logging.error("Unrecognized color %s in file %s", lc, svg_path / "light.svg")
                return None
            light_var = light_palette[lc_norm]
            class_str = f"{attr}-[var({light_var})]"
            del le.attrib[attr]
            existing = le.attrib.get("className", "")
            combined = " ".join(filter(None, [existing, class_str]))
            le.set("className", combined)

            dc_norm = normalize_color(dc)
            if dc_norm not in dark_palette:
                logging.error("Unrecognized color %s in file %s", dc, svg_path / "dark.svg")
                return None
            dark_var = dark_palette[dc_norm]
            class_str = f"{attr}-[var({dark_var})]"
            del de.attrib[attr]
            existing = de.attrib.get("className", "")
            combined = " ".join(filter(None, [existing, class_str]))
            de.set("className", combined)

    light_xml = ET.tostring(light_el, encoding='unicode')
    light_svg = append_svg_props(light_xml)
    dark_xml = ET.tostring(dark_el, encoding='unicode')
    dark_svg = append_svg_props(dark_xml)
    return (
        f"import React from 'react';"
        f"\nimport type {{ SVGProps }} from 'react';"
        f"\nconst Svg{name}: React.FC<SVGProps<SVGSVGElement>> = props => ("
        f"\n  <div>"
        f"\n    {light_svg}"  # Light theme
        f"\n    {dark_svg}"   # Dark theme
        f"\n  </div>"
        f"\n);"
        f"\nexport default Svg{name};"
    )


def svg_to_component(name: str, svg_xml: str) -> str:
    """
    Wrap raw SVG XML into a React functional component.
    """
    svg_with_props = append_svg_props(svg_xml)
    return (
        f"import React from 'react';"
        f"\nimport type {{ SVGProps }} from 'react';"
        f"\nconst Svg{name}: React.FC<SVGProps<SVGSVGElement>> = props => {svg_with_props};"
        f"\nexport default Svg{name};"
    )


def generate_components(
        icons_dir: Path,
        light_json: Path,
        dark_json: Path,
        output_dir: Path
) -> None:
    """
    Generate React components for each icon in icons_dir based on plan.json.
    """
    plan_file = icons_dir / 'plan.json'
    if not plan_file.exists():
        logging.error("Plan file not found: %s", plan_file)
        return

    plan_data = json.loads(plan_file.read_text())
    light_palette = extract_palette_from_json(light_json)
    dark_palette = extract_palette_from_json(dark_json)

    # Remove old build artifacts
    if output_dir.exists():
        shutil.rmtree(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    for name, props in plan_data.items():
        svg_path = icons_dir / name
        # logging.info("Processing icon '%s'", name)
        light_tree = ET.parse(svg_path / 'light.svg')
        dark_tree = ET.parse(svg_path / 'dark.svg')

        # ignore `width` and `height` attributes at the top level
        for attr in EXCLUDED_ATTRIBUTES:
            if attr in light_tree.getroot().attrib:
                del light_tree.getroot().attrib[attr]
            if attr in dark_tree.getroot().attrib:
                del dark_tree.getroot().attrib[attr]

        # Reformat the attribute names
        elems = list(light_tree.iter()) + list(dark_tree.iter())
        for elem in elems:
            for attr in elem.attrib.copy():
                # Convert kebab-case to camelCase
                if "-" in attr:
                    camel = to_lower_camel_case(attr)
                    val = elem.attrib[attr]
                    del elem.attrib[attr]
                    elem.set(camel, val)

        if props.get('identical', False) and compare_svg_structure(light_tree, dark_tree):
            merged = consolidate_svg(svg_path, light_tree, dark_tree, light_palette, dark_palette)
            if not merged:
                continue
            svg_xml = ET.tostring(merged.getroot(), encoding='unicode')
            component = svg_to_component(name, svg_xml)
        else:
            component = merge_svg_to_component(svg_path, name, light_tree, dark_tree, light_palette, dark_palette)
            if not component:
                continue

        (output_dir / f"{name}.tsx").write_text(component)

    # Generate index.ts
    exports = [f"export {{ default as {name} }} from './{name}';" for name in plan_data]
    (output_dir / 'index.ts').write_text("\n".join(exports))
    logging.info("Components generated in %s", output_dir)

    # Update the lock_file after successful generation
    names = sorted([p.name for p in icons_dir.iterdir() if p.is_dir()])
    lock_file = icons_dir / 'plan.lock'
    hash_val = hashlib.md5(''.join(names).encode()).hexdigest()
    lock_file.write_text(hash_val)


def create_plan(icons_dir: Path, force: bool = False) -> None:
    """
    Create or update plan.json for SVG icons based on structural comparison.
    """
    names = sorted([p.name for p in icons_dir.iterdir() if p.is_dir()])
    lock_file = icons_dir / 'plan.lock'
    hash_val = hashlib.md5(''.join(names).encode()).hexdigest()

    if lock_file.exists() and lock_file.read_text() == hash_val and not force:
        logging.info("No changes in icon list.")
        return

    plan_path = icons_dir / 'plan.json'

    existing = json.loads(plan_path.read_text()) if plan_path.exists() else {}
    updated = {}

    for name in names:
        if not force and name in existing:
            updated[name] = existing[name]
            continue

        light_svg = Path(icons_dir, name, "light.svg")
        if not light_svg.exists():
            logging.error(f"`light.svg` not found for icon `{name}`")
            continue

        dark_svg = Path(icons_dir, name, "dark.svg")
        if not dark_svg.exists():
            logging.error(f"`dark.svg` not found for icon `{name}`")
            continue

        try:
            light = ET.parse(icons_dir / name / 'light.svg')
            dark = ET.parse(icons_dir / name / 'dark.svg')
            identical = compare_svg_structure(light, dark)
            updated[name] = {'identical': identical}
            logging.info("Planned icon '%s': identical=%s", name, identical)
        except Exception as e:
            logging.error(f"Failed to create plan for icon `{name}`: {e}")

    plan_path.write_text(json.dumps(updated, indent=2))


def main():
    parser = argparse.ArgumentParser(
        description="Generate React SVG icon components with light/dark theming"
    )
    parser.add_argument(
        '-v', '--verbose', action='store_true', help='Enable debug logging'
    )
    subparsers = parser.add_subparsers(dest='command')

    plan_parser = subparsers.add_parser('plan', help='Generate or update the icon plan')
    plan_parser.add_argument('--source', type=Path, required=True, help='Source icons folder')
    plan_parser.add_argument('--force', action='store_true', help='Force plan regeneration')

    gen_parser = subparsers.add_parser('gen', help='Generate React components')
    gen_parser.add_argument('--source', type=Path, required=True, help='Source icons folder')
    gen_parser.add_argument('--light-json', type=Path, required=True, help='Light theme JSON file')
    gen_parser.add_argument('--dark-json', type=Path, required=True, help='Dark theme JSON file')
    gen_parser.add_argument('--output-dir', type=Path, required=True, help='Output directory')

    args = parser.parse_args()
    setup_logging(args.verbose)

    if args.command == 'plan':
        create_plan(args.source, args.force)
    elif args.command == 'gen':
        generate_components(args.source, args.light_json, args.dark_json, args.output_dir)
    else:
        parser.print_help()


if __name__ == '__main__':
    main()