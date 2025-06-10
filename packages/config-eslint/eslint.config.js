import { defineConfig } from "eslint/config";

import defaultConfig from "./configs/default.js";

/** @type {import('eslint').Linter.Config[]} */
export default defineConfig([defaultConfig]);
