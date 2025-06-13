import { resolve } from "path";
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
      {
        test: {
          name: "desktop",
          include: ["view/desktop/src/**/*.{test,spec}.?(c|m)[jt]s?(x)"],
          exclude: ["view/desktop/src/lib/**/*.{test,spec}.?(c|m)[jt]s?(x)"],
          globals: true,
          environment: "jsdom",
          alias: {
            "@": resolve(__dirname, "./view/desktop/src"),
            "@/components": resolve(__dirname, "./view/desktop/src/components"),
            "@/store": resolve(__dirname, "./view/desktop/src/store"),
            "@/utils": resolve(__dirname, "./view/desktop/src/utils"),
            "@/hooks": resolve(__dirname, "./view/desktop/src/hooks"),
            "@/assets": resolve(__dirname, "./view/desktop/src/assets"),
            "@/constants": resolve(__dirname, "./view/desktop/src/constants"),
          },
        },
      },
    ],
  },
});
