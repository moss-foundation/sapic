import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

import { defaultConfig, defineConfig } from "@repo/eslint-config";

const __dirname = dirname(fileURLToPath(import.meta.url));

export default defineConfig([
  ...defaultConfig,
  {
    ignores: ["**/moss-tabs/"],
  },
]);
