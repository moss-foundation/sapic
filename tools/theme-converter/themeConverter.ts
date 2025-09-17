import fs from "fs/promises";
import path from "path";
import postcss from "postcss";
import * as process from "node:process";

type StringValue = {
  "type": "String";
  "value": string;
};

type VariableValue = {
  "type": "Variable";
  "value": string;
};

type CssValue = StringValue | VariableValue;

type ThemeMode = "light" | "dark";
type Theme = {
  "identifier": string;
  "displayName": string;
  "mode": ThemeMode;
  "tokens": Record<string, CssValue>;
};

const VARIABLE_RE = /var\((.*)\)/;

function cssToJsonKey(cssKey: string): string {
  const bare = cssKey.replace(/^--/, "");
  return bare.split("-").join(".");
}

async function convertCssTokens(cssText: string): Promise<Record<string, CssValue>> {
  const root = postcss.parse(cssText);
  const tokens: Record<string, CssValue> = {};

  root.walkRules((rule) => {
    // rule.selector can be a single string like ':root' or a comma-separated list.
    if (!rule.selector) return;
    const selectors = rule.selector.split(",").map((s) => s.trim());
    if (!selectors.includes(":root")) return;

    rule.walkDecls((decl) => {
      if (!decl.prop || !decl.prop.startsWith("--")) {
        return;
      }
      const key = cssToJsonKey(decl.prop);

      const value = decl.value;

      const variableMatch = VARIABLE_RE.exec(value);

      if (variableMatch) {
        const variableCssName = variableMatch[1];
        const variableKey = cssToJsonKey(variableCssName);
        tokens[key] = {
          "type": "Variable",
          value: variableKey,
        };
      } else {
        tokens[key] = {
          "type": "String",
          value: value,
        };
      }
    });
  });

  return tokens;
}

async function main(): Promise<void> {
  const targetPath = process.argv[2];
  const outputPath = process.argv[3];

  if (!targetPath) {
    console.error("❌ Usage: ts-node themeConverter.ts <path-to-css> <path-to-json-output>");
    process.exit(1);
  }

  const css = await fs.readFile(targetPath, "utf8");
  const tokens = await convertCssTokens(css);

  const theme: Theme = {
    "identifier": "",
    "displayName": "",
    "mode": "light",
    tokens: tokens,
  };

  await fs.writeFile(outputPath, JSON.stringify(theme, null, 2));
}

// Execute if this file is run directly
if (require.main === module) {
  main().catch((error) => {
    console.error("💥 Unhandled error:", error);
    process.exit(1);
  });
}
