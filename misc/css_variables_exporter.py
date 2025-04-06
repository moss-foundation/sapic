import cssutils

from pathlib import Path

if __name__ == "__main__":
    txt = Path('../assets/themes/light.css').read_text()
    css = cssutils.parseString(cssText=txt, validate=False)
    variables = css.cssRules.item(0).style.keys()
    Path("../packages/config-eslint/moss-lint-plugin/css_variables.txt").write_text("\n".join(variables))
