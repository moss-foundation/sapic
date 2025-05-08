/* 
  Add missing imports to the generated typescript models
  Argument: path to the crate
*/
import { join } from "path";
import { Project } from "ts-morph";

const path = join(process.argv[2], "tsconfig.json");

let project = new Project({ tsConfigFilePath: path, skipFileDependencyResolution: true, skipLoadingLibFiles: true });

project.getSourceFiles().forEach((file) => file.fixMissingImports());
project.saveSync();
