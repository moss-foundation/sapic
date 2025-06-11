import { defaultConfig, defineConfig } from "@repo/eslint-config";

export default defineConfig([
  ...defaultConfig,
  {
    ignores: ["**/moss-tabs/"],
  },
]);
