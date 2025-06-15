import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    projects: [
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
      "view/desktop/vitest.config.ts",
    ],
  },
});
