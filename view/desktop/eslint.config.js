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

// export default [
//   ...defaultConfig,
//   {
//     ignores: ["**/moss-tabs/"],
//   },
//   {
//     files: ["*.ts", "*.tsx"],
//     settings: {
//       "import/resolver": {
//         node: {
//           paths: ["src"],
//           extensions: [".js", ".jsx", ".ts", ".d.ts", ".tsx"],
//         },
//         typescript: {
//           project: "./tsconfig.json",
//         },
//         alias: {
//           map: [
//             ["@/assets", path.resolve(__dirname, "./src/assets")],
//             ["@/components", path.resolve(__dirname, "./src/components")],
//             ["@/hooks", path.resolve(__dirname, "./src/hooks")],
//             ["@/store", path.resolve(__dirname, "./src/store")],
//           ],
//           extensions: [".js", ".jsx", ".ts", ".d.ts", ".tsx"],
//         },
//       },
//     },
//   },
// ];
