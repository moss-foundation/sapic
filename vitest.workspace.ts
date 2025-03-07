import { defineWorkspace } from "vitest/config";

export default defineWorkspace([
  {
    test: {
      name: "packages",
      include: [
        "packages/config-eslint/**/*.{test,spec}.?(c|m)[jt]s?(x)",
        "packages/config-tailwind/**/*.{test,spec}.?(c|m)[jt]s?(x)",
        "packages/config-typescript/**/*.{test,spec}.?(c|m)[jt]s?(x)",
      ],
    },
  },
  {
    test: {
      name: "moss-tabs",
      include: ["view/desktop/src/lib/moss-tabs/**/*.{test,spec}.?(c|m)[jt]s?(x)"],
      globals: true,
      environment: "jsdom",
      setupFiles: ["./view/desktop/src/lib/moss-tabs/vitest.setup.ts"],
    },
  },
  //{
  //  test: {
  //    name: "desktop",
  //    include: ["view/desktop/**/*.{test,spec}.?(c|m)[jt]s?(x)"],
  //  },
  //},
]);
