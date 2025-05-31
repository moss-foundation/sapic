/**
 * TypeScript Exports Injector
 *
 * Automatically generates and maintains export declarations for TypeScript binding files.
 * This tool scans the bindings directory and updates the index.ts file with proper exports.
 *
 * Usage: ts-node tsExportsInjector.ts <path-to-target-directory>
 *
 * @example
 * ```bash
 * ts-node tsExportsInjector.ts ./src/generated
 * ```
 */

import { existsSync, readdirSync, statSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import { ExportDeclaration, Project, SourceFile } from "ts-morph";

interface Config {
  readonly targetPath: string;
  readonly indexFileName: string;
  readonly bindingsDirectoryName: string;
  readonly fileExtension: string;
}

class TypeScriptExportsInjector {
  private readonly config: Config;
  private readonly project: Project;

  constructor(targetPath: string) {
    this.config = {
      targetPath: resolve(targetPath),
      indexFileName: "index.ts",
      bindingsDirectoryName: "bindings",
      fileExtension: ".ts",
    };

    this.project = new Project({
      useInMemoryFileSystem: false,
      skipFileDependencyResolution: true,
    });

    this.validatePaths();
  }

  /**
   * Validates that required paths exist and are accessible
   */
  private validatePaths(): void {
    const { targetPath, indexFileName, bindingsDirectoryName } = this.config;

    if (!existsSync(targetPath)) {
      throw new Error(`Target directory does not exist: ${targetPath}`);
    }

    const indexPath = join(targetPath, indexFileName);
    if (!existsSync(indexPath)) {
      throw new Error(`Index file does not exist: ${indexPath}`);
    }

    const bindingsPath = join(targetPath, bindingsDirectoryName);
    if (!existsSync(bindingsPath)) {
      throw new Error(`Bindings directory does not exist: ${bindingsPath}`);
    }

    if (!statSync(bindingsPath).isDirectory()) {
      throw new Error(`Bindings path is not a directory: ${bindingsPath}`);
    }
  }

  /**
   * Discovers TypeScript files in the bindings directory
   */
  private discoverBindingFiles(): string[] {
    const bindingsPath = join(this.config.targetPath, this.config.bindingsDirectoryName);

    try {
      const files = readdirSync(bindingsPath, { withFileTypes: true })
        .filter(
          (dirent) => dirent.isFile() && dirent.name.endsWith(this.config.fileExtension) && !dirent.name.startsWith(".")
        )
        .map((dirent) => dirent.name)
        .sort(); // Ensure consistent ordering

      console.log(`🔍 Found ${files.length} TypeScript binding files`);
      return files;
    } catch (error) {
      throw new Error(`Failed to read bindings directory: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Removes existing binding exports from the index file
   */
  private removeExistingBindingExports(indexFile: SourceFile): number {
    const bindingExports = indexFile.getExportDeclarations().filter((decl: ExportDeclaration) => {
      const moduleSpecifier = decl.getModuleSpecifierValue();
      return moduleSpecifier?.startsWith(`./${this.config.bindingsDirectoryName}/`);
    });

    const removedCount = bindingExports.length;
    bindingExports.forEach((decl) => decl.remove());

    if (removedCount > 0) {
      console.log(`🗑️  Removed ${removedCount} existing binding exports`);
    }

    return removedCount;
  }

  /**
   * Adds new export declarations for discovered binding files
   */
  private addNewBindingExports(indexFile: SourceFile, bindingFiles: string[]): void {
    bindingFiles.forEach((fileName) => {
      const moduleName = basename(fileName, this.config.fileExtension);
      const moduleSpecifier = `./${this.config.bindingsDirectoryName}/${moduleName}`;

      // Create export * from syntax directly
      indexFile.addExportDeclaration({
        moduleSpecifier,
      });

      console.log(`📤 Added export for: ${moduleName}`);
    });
  }

  /**
   * Organizes imports and exports in the index file
   */
  private organizeImportsAndExports(indexFile: SourceFile): void {
    try {
      console.log(`📋 Organizing imports and exports...`);
      indexFile.organizeImports();
      console.log(`✅ Successfully organized imports and exports`);
    } catch (error) {
      console.warn(
        `⚠️  Warning: Failed to organize imports: ${error instanceof Error ? error.message : String(error)}`
      );
      // Don't throw here as this is not critical for the main functionality
    }
  }

  /**
   * Main execution method
   */
  public async execute(): Promise<void> {
    try {
      console.log(`🚀 Starting TypeScript exports injection for: ${this.config.targetPath}`);

      const indexPath = join(this.config.targetPath, this.config.indexFileName);
      this.project.addSourceFileAtPath(indexPath);

      const indexFile = this.project.getSourceFile(indexPath);
      if (!indexFile) {
        throw new Error(`Failed to load index file: ${indexPath}`);
      }

      const bindingFiles = this.discoverBindingFiles();
      if (bindingFiles.length === 0) {
        console.warn("⚠️  No TypeScript binding files found");
        return;
      }

      this.removeExistingBindingExports(indexFile);
      this.addNewBindingExports(indexFile, bindingFiles);
      this.organizeImportsAndExports(indexFile);

      console.log(`💾 Saving changes...`);
      await this.project.save();
      console.log(`🎉 Successfully updated ${this.config.indexFileName} with ${bindingFiles.length} exports`);
    } catch (error) {
      console.error(`❌ Error during execution: ${error instanceof Error ? error.message : String(error)}`);
      throw error;
    }
  }
}

async function main(): Promise<void> {
  const targetPath = process.argv[2];

  if (!targetPath) {
    console.error("❌ Usage: ts-node tsExportsInjector.ts <path-to-target-directory>");
    process.exit(1);
  }

  try {
    const injector = new TypeScriptExportsInjector(targetPath);
    await injector.execute();
    console.log("🎉 TypeScript exports injection completed successfully!");
  } catch (error) {
    console.error(`💥 Fatal error: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}

// Execute if this file is run directly
if (require.main === module) {
  main().catch((error) => {
    console.error("💥 Unhandled error:", error);
    process.exit(1);
  });
}

export { TypeScriptExportsInjector };
