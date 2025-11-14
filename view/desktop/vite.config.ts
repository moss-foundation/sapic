import { resolve } from "path";
import { defineConfig } from "vite";
import svgr from "vite-plugin-svgr";

import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [tailwindcss(), react(), svgr()],

  resolve: {
    alias: {
      "@": resolve(__dirname, "src"),
      "@/hooks": resolve(__dirname, "src/hooks"),
      "@/utils": resolve(__dirname, "src/utils"),
      "@/assets": resolve(__dirname, "src/assets"),
      "@/components": resolve(__dirname, "src/components"),
      "@/store": resolve(__dirname, "src/store"),
      "@/workbench": resolve(__dirname, "src/workbench"),
      "@/welcome": resolve(__dirname, "src/welcome"),
    },
  },
  build: {
    rollupOptions: {
      input: {
        welcome: resolve(__dirname, "welcome.html"),
        workspace: resolve(__dirname, "workspace.html"),
      },
    },
    // don't minify for debug builds
    minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
});
