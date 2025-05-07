import { Project, SourceFile } from "ts-morph";
import fs, { readdirSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { exec } from "node:child_process";

// Types that might be useful for AI agent
const PATHS = [
  "../../crates/moss-collection/bindings",
  "../../crates/moss-state/bindings",
  "../../crates/moss-logging/bindings",
  "../../crates/moss-environment/bindings",
  "../../crates/moss-workspace/bindings",
  "../../crates/moss-workbench/bindings",
];

function postprocessZod(tsPath: string, zodPath: string) {
  // Compare the original file and zod file, and process the latter
  let project = new Project();
  project.addSourceFileAtPath(tsPath);
  project.addSourceFileAtPath(zodPath);
  let tsFile = project.getSourceFile(tsPath) as SourceFile;
  let zodFile = project.getSourceFile(zodPath) as SourceFile;

  for (let decl of tsFile?.getImportDeclarations()) {
    let module = decl.getModuleSpecifierValue();
    // Convert local imports to the zod module
    // e.g. from "./types" => from "./types.zod"
    if (module.startsWith("./")) {
      module = module + ".zod";
    }

    for (let i of decl.getNamedImports()) {
      // Convert type import to zod schema import
      // OriginalType => originalTypeSchema
      let typeName = i.getName();
      let schemaName = typeName.charAt(0).toLowerCase() + typeName.slice(1) + "Schema";
      zodFile.addImportDeclaration({
        namedImports: [schemaName],
        moduleSpecifier: module,
      });
      zodFile.getVariableDeclaration(schemaName)?.remove();
    }
    project.saveSync();
  }
}

function processFile(path: fs.PathLike) {
  // 1. Run ts-to-zod
  const tsPath = path.toString();
  const dirName = dirname(tsPath);
  const zodFilename = basename(tsPath, ".ts") + ".zod.ts";
  const zodPath = join(dirName, zodFilename);
  exec(`npx ts-to-zod ${tsPath} ${zodPath}`, (err, stdout, stderr) => {
    if (err) {
      console.error();
      console.error("Error:");
      console.error(err);
      console.error();
    } else {
      postprocessZod(tsPath, zodPath);
    }
    console.log(stdout);
    console.error(stderr);
  });
}

for (let path of PATHS) {
  console.log(`Processing path ${path}`);
  try {
    const files = readdirSync(path, { recursive: true }) as string[];
    for (let file of files) {
      if (file.endsWith(".zod.ts")) {
        continue;
      }
      let filePath = join(path, file);
      console.log(`Processing file ${filePath}`);
      processFile(filePath);
    }
  } catch (error) {
    console.log(error);
  }
}
