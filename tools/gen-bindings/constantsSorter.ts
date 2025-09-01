import { Project, VariableStatementStructure } from "ts-morph";

interface ConstantItem {
  name: string;
  structure: VariableStatementStructure;
}

const targetPath = process.argv[2];
if (!targetPath) {
  console.error("Usage: ts-node constantsSorter.ts <path-to-target-directory>");
  process.exit(1);
}

const project = new Project();
const srcs = project.addSourceFilesAtPaths(`${targetPath}/bindings/*.ts`);

for (const src of srcs) {
  console.log(`Processing ${src.getFilePath()}`);
  const items: ConstantItem[] = [];
  src.getVariableStatements().forEach((stmt) => {
    const decl = stmt.getDeclarations()[0];
    const name = decl.getName();
    const structure = stmt.getStructure();
    items.push({
      name,
      structure,
    });
    stmt.remove();
  });

  items.sort((a, b) => a.name.toLowerCase().localeCompare(b.name.toLowerCase()));

  if (items.length > 0) {
    // Append all const variable statements at the end
    src.addVariableStatements(items.map((item) => item.structure));
    src.saveSync();
  }
}
