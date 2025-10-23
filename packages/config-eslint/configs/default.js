import { createTypeScriptImportResolver } from "eslint-import-resolver-typescript";
import importX, { createNodeResolver } from "eslint-plugin-import-x";
import reactHooksPlugin from "eslint-plugin-react-hooks";
import reactYouMightNotNeedAnEffect from "eslint-plugin-react-you-might-not-need-an-effect";
import { defineConfig } from "eslint/config";
import tseslint from "typescript-eslint";

import tseslintParser from "@typescript-eslint/parser";

import mossLintPlugin from "../moss-lint-plugin/index.js";

export default defineConfig(
  ...tseslint.configs.recommended,
  importX.flatConfigs.recommended,
  importX.flatConfigs.typescript,
  reactYouMightNotNeedAnEffect.configs.recommended,
  {
    ignores: [
      "node_modules/",
      "dist/",
      ".gitignore",
      ".prettierignore",
      "target/",
      ".turbo/",
      ".vscode/",
      "**/*.stories.*",
      "**/*.test.*",
      "**/*.spec.*",
    ],
  },
  {
    languageOptions: {
      parser: tseslintParser,
      parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module",
      },
    },
    settings: {
      "import-x/resolver-next": [
        createTypeScriptImportResolver({
          alwaysTryTypes: true,
          project: ["tsconfig.json"],
        }),
        createNodeResolver({
          "extensions": [".js", ".jsx", ".ts", ".tsx", ".json"],
        }),
      ],
      react: {
        version: "detect",
      },
    },
    rules: {
      "import-x/named": "error",
      "import-x/default": "off",
      "import-x/no-named-as-default": "off",
      "import-x/no-named-as-default-member": "off",
      "import-x/namespace": [1, { allowComputed: true }],
    },
  },
  {
    files: ["**/*.{ts,tsx,js,jsx}"],
    plugins: {
      "react-hooks": reactHooksPlugin,
      "@typescript-eslint": tseslint.plugin,
      mossLint: mossLintPlugin,
    },
    rules: {
      "react-hooks/rules-of-hooks": "error",
      "react-hooks/exhaustive-deps": "warn",
      "@typescript-eslint/no-unused-vars": [
        "warn",
        {
          "argsIgnorePattern": "^_",
          "varsIgnorePattern": "^_",
          "caughtErrorsIgnorePattern": "^_",
        },
      ],
      "@typescript-eslint/no-explicit-any": "error",
      "prefer-const": "warn",
      "mossLint/tw-no-bg-with-arbitrary-value": "error",
      "mossLint/only-valid-token-names": "error",
      "mossLint/tw-no-old-syntax-for-arbitrary-values": "error",
    },
  }
);
