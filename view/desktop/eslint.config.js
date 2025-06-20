import defaultConfig from "@repo/eslint-config";

export default [
  ...defaultConfig,
  {
    ignores: ["**/moss-tabs/"],
  },
];
