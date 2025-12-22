import { lezer } from "@lezer/generator/rollup";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import typescript from "@rollup/plugin-typescript";

export default {
  input: "./src/parser.ts",
  external: ["@codemirror/language", "@lezer/lr", "@lezer/highlight"],
  output: [
    {
      file: "./dist/index.js",
      format: "es",
    },
    {
      file: "./dist/index.cjs",
      format: "cjs",
    },
  ],
  plugins: [
    lezer(), // <--- Compiles the .grammar file
    nodeResolve({ extensions: [".js", ".ts"] }),
    typescript({
      declaration: true,
      declarationDir: "./dist",
      rootDir: "./src",
    }),
  ],
};
