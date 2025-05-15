/* 
  Add missing imports to the generated typescript models
  Argument: path to the crate
*/
import { join } from "path";
import { Project } from "ts-morph";

const path = process.argv[2];
const configPath = join(path, "tsconfig.json");

let project = new Project({ tsConfigFilePath: configPath });

project.addSourceFileAtPathIfExists(join(path, "node_modules", "@repo", "bindings-utils", "index.ts"));
project.addSourceFileAtPathIfExists(join(path, "node_modules", "@repo", "moss-common", "index.ts"));
project.addSourceFileAtPathIfExists(join(path, "node_modules", "@repo", "moss-environment", "index.ts"));
project.addSourceFileAtPathIfExists(join(path, "node_modules", "@repo", "moss-workspace", "index.ts"));
project.addSourceFileAtPathIfExists(join(path, "node_modules", "@repo", "moss-theme", "index.ts"));
project.addSourceFileAtPathIfExists(join(path, "node_modules", "@repo", "moss-nls", "index.ts"));
project.resolveSourceFileDependencies();
project.getSourceFiles().forEach((file) => {
  file.fixMissingImports();
});
project.saveSync();
