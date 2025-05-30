import { join } from "path";
import { Project, SourceFile } from "ts-morph";
import { readdirSync } from "node:fs";
import { basename } from "node:path";

const path = process.argv[2];
const indexPath = join(path, "index.ts");
const bindingsPath = join(path, "bindings");

let project = new Project();

project.addSourceFileAtPath(indexPath);

let indexFile = project.getSourceFile(indexPath) as SourceFile;

// Remove old bindings exports
indexFile
  .getExportDeclarations()
  .filter((decl) => decl.getModuleSpecifierValue()?.startsWith("./bindings/"))
  .forEach((decl) => decl.remove());

// Generate new bindings exports
let tsFiles = readdirSync(bindingsPath).filter((name) => name.endsWith(".ts"));
tsFiles.forEach((fileName) => {
  const importPath = basename(fileName, ".ts");
  console.log(importPath);
  indexFile.addExportDeclaration({
    namedExports: [],
    moduleSpecifier: `./bindings/${importPath}`,
  });
});

project.saveSync();
