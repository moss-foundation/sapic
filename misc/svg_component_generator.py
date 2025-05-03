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

# Constants
COLOR_ATTRIBUTES = {"fill", "stroke"}
EXCLUDED_VALUES = {"none", "transparent"}
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


def extract_css_palette(css_path: Path) -> Dict[str, str]:
    """
    Parse a CSS file and extract custom property names for colors.
    Returns mapping from normalized hex value to CSS variable name.
    """
    logging.debug(f"Reading CSS palette from %s", css_path)
    content = css_path.read_text()
    rules = tinycss2.parse_stylesheet(content, skip_comments=True, skip_whitespace=True)

    palette: Dict[str, str] = {}
    for rule in rules:
        if rule.type != "qualified-rule":
            continue
        for decl in tinycss2.parse_declaration_list(rule.content):
            if decl.type == "declaration" and decl.lower_name.startswith("--"):
                vals = [tok for tok in decl.value if tok.type == "hash"]
                if len(vals) == 1:
                    hex_code = webcolors.normalize_hex(f"#{vals[0].value}")
                    palette[hex_code] = decl.lower_name
                    logging.debug("Mapped %s → %s", hex_code, decl.lower_name)
    return palette


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
        light_tree: ET.ElementTree,
        dark_tree: ET.ElementTree,
        light_palette: Dict[str, str],
        dark_palette: Dict[str, str]
) -> ET.ElementTree:
    """
    Merge light and dark SVGs into a single tree by mapping colors to Tailwind CSS variables.
    """
    result = deepcopy(light_tree)

    # Ensure namespace registration
    ET.register_namespace("", SVG_NAMESPACE)

    light_elements = list(light_tree.iter())
    dark_elements = list(dark_tree.iter())
    res_elements = list(result.iter())

    for le, de, relem in zip(light_elements, dark_elements, res_elements):
        for attr in COLOR_ATTRIBUTES:
            lc = le.attrib.get(attr)
            dc = de.attrib.get(attr)
            if not lc or not dc or lc in EXCLUDED_VALUES:
                continue
            try:
                lc_norm = normalize_color(lc)
                dc_norm = normalize_color(dc)
                light_var = light_palette[lc_norm]
                dark_var = dark_palette[dc_norm]
            except KeyError as e:
                logging.warning("Unrecognized color %s in %s", e, relem.tag)
                continue

            class_str = f"{attr}-[var({light_var})] dark:{attr}-[var({dark_var})]"
            prev = relem.attrib.pop(attr, None)
            existing = relem.attrib.get("className", "")
            combined = " ".join(filter(None, [existing, class_str]))
            relem.set("className", combined)

    return result


def svg_to_component(name: str, svg_xml: str) -> str:
    """
    Wrap raw SVG XML into a React functional component.
    """
    svg_with_props = re.sub(r'<svg([^>]*)>', r'<svg\1 {...props}>', svg_xml, count=1)
    return (
        f"import React from 'react';"
        f"\nimport type {{ SVGProps }} from 'react';"
        f"\nconst Svg{name}: React.FC<SVGProps<SVGSVGElement>> = props => {svg_with_props};"
        f"\nexport default Svg{name};"
    )


def generate_components(
        icons_dir: Path,
        light_css: Path,
        dark_css: Path,
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
    light_palette = extract_css_palette(light_css)
    dark_palette = extract_css_palette(dark_css)

    output_dir.mkdir(parents=True, exist_ok=True)

    for name, props in plan_data.items():
        logging.info("Processing icon '%s'", name)
        light_tree = ET.parse(icons_dir / name / 'light.svg')
        dark_tree = ET.parse(icons_dir / name / 'dark.svg')

        if props.get('identical', False) and compare_svg_structure(light_tree, dark_tree):
            merged = consolidate_svg(light_tree, dark_tree, light_palette, dark_palette)
            svg_xml = ET.tostring(merged.getroot(), encoding='unicode')
            component = svg_to_component(name, svg_xml)
        else:
            # Separate rendering for light/dark
            light_el = light_tree.getroot()
            dark_el = dark_tree.getroot()
            light_el.set('className', 'block dark:hidden')
            dark_el.set('className', 'hidden dark:block')
            light_xml = ET.tostring(light_el, encoding='unicode')
            dark_xml = ET.tostring(dark_el, encoding='unicode')
            component = (
                f"import React from 'react';"
                f"\nimport type {{ SVGProps }} from 'react';"
                f"\nconst Svg{name}: React.FC<SVGProps<SVGSVGElement>> = props => ("
                f"\n  <div {{...props}}>"
                f"\n    {light_xml}"  # Light theme
                f"\n    {dark_xml}"   # Dark theme
                f"\n  </div>"
                f"\n);"
                f"\nexport default Svg{name};"
            )

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

        light = ET.parse(icons_dir / name / 'light.svg')
        dark = ET.parse(icons_dir / name / 'dark.svg')
        identical = compare_svg_structure(light, dark)
        updated[name] = {'identical': identical}
        logging.info("Planned icon '%s': identical=%s", name, identical)

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
    gen_parser.add_argument('--light-css', type=Path, required=True, help='Light theme CSS file')
    gen_parser.add_argument('--dark-css', type=Path, required=True, help='Dark theme CSS file')
    gen_parser.add_argument('--output-dir', type=Path, required=True, help='Output directory')

    args = parser.parse_args()
    setup_logging(args.verbose)

    if args.command == 'plan':
        create_plan(args.source, args.force)
    elif args.command == 'gen':
        generate_components(args.source, args.light_css, args.dark_css, args.output_dir)
    else:
        parser.print_help()


if __name__ == '__main__':
    main()