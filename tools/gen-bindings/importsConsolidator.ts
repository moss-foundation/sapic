/**
 * TypeScript Imports Consolidator
 *
 * Organizes and consolidates import statements in TypeScript files within a project.
 * This tool uses ts-morph to automatically sort, group, and optimize imports according
 * to TypeScript best practices.
 *
 * Usage: ts-node importsConsolidator.ts <path-to-target-directory>
 *
 * @example
 * ```bash
 * ts-node importsConsolidator.ts ./crates/moss-workspace
 * ```
 */

import { existsSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { Project, SourceFile } from "ts-morph";

interface Config {
  readonly targetPath: string;
  readonly tsConfigFileName: string;
  readonly skipDependencyResolution: boolean;
  readonly skipLoadingLibFiles: boolean;
}

interface ConsolidationResult {
  readonly success: boolean;
  readonly processedFiles: number;
  readonly organizedFiles: number;
  readonly errors: string[];
}

class TypeScriptImportsConsolidator {
  private readonly config: Config;
  private readonly project: Project;

  constructor(targetPath: string) {
    this.config = {
      targetPath: resolve(targetPath),
      tsConfigFileName: "tsconfig.json",
      skipDependencyResolution: true,
      skipLoadingLibFiles: true,
    };

    this.project = this.initializeProject();
    this.validatePaths();
  }

  /**
   * Initializes the TypeScript project with proper configuration
   */
  private initializeProject(): Project {
    const tsConfigPath = join(this.config.targetPath, this.config.tsConfigFileName);

    if (!existsSync(tsConfigPath)) {
      console.warn(`⚠️  TypeScript config not found at ${tsConfigPath}, using default configuration`);
      return new Project({
        useInMemoryFileSystem: false,
        skipFileDependencyResolution: this.config.skipDependencyResolution,
        skipLoadingLibFiles: this.config.skipLoadingLibFiles,
      });
    }

    try {
      return new Project({
        tsConfigFilePath: tsConfigPath,
        skipFileDependencyResolution: this.config.skipDependencyResolution,
        skipLoadingLibFiles: this.config.skipLoadingLibFiles,
      });
    } catch (error) {
      console.warn(
        `⚠️  Failed to load TypeScript config, using default: ${error instanceof Error ? error.message : String(error)}`
      );
      return new Project({
        useInMemoryFileSystem: false,
        skipFileDependencyResolution: this.config.skipDependencyResolution,
        skipLoadingLibFiles: this.config.skipLoadingLibFiles,
      });
    }
  }

  /**
   * Validates that required paths exist and are accessible
   */
  private validatePaths(): void {
    const { targetPath } = this.config;

    if (!existsSync(targetPath)) {
      throw new Error(`Target directory does not exist: ${targetPath}`);
    }

    if (!statSync(targetPath).isDirectory()) {
      throw new Error(`Target path is not a directory: ${targetPath}`);
    }
  }

  /**
   * Organizes imports for a single source file
   */
  private organizeFileImports(sourceFile: SourceFile): boolean {
    try {
      const filePath = sourceFile.getFilePath();
      const fileName = sourceFile.getBaseName();

      console.log(`📋 Organizing imports for: ${fileName}`);

      // Get initial import count for comparison
      const initialImports = sourceFile.getImportDeclarations().length;

      // Organize imports
      sourceFile.organizeImports();

      // Get final import count
      const finalImports = sourceFile.getImportDeclarations().length;

      if (initialImports !== finalImports) {
        console.log(`   ✨ Optimized imports: ${initialImports} → ${finalImports}`);
      } else {
        console.log(`   ✅ Imports already organized`);
      }

      return true;
    } catch (error) {
      console.error(`   ❌ Failed to organize imports: ${error instanceof Error ? error.message : String(error)}`);
      return false;
    }
  }

  /**
   * Processes all source files to organize their imports
   */
  private processSourceFiles(): { processedFiles: number; organizedFiles: number; errors: string[] } {
    const sourceFiles = this.project.getSourceFiles();
    const errors: string[] = [];
    let processedFiles = 0;
    let organizedFiles = 0;

    console.log(`📁 Processing ${sourceFiles.length} source files...`);

    for (const sourceFile of sourceFiles) {
      try {
        const wasOrganized = this.organizeFileImports(sourceFile);
        processedFiles++;

        if (wasOrganized) {
          organizedFiles++;
        }
      } catch (error) {
        const errorMessage = `Failed to process ${sourceFile.getBaseName()}: ${error instanceof Error ? error.message : String(error)}`;
        errors.push(errorMessage);
        console.error(`❌ ${errorMessage}`);
      }
    }

    return { processedFiles, organizedFiles, errors };
  }

  /**
   * Saves all changes to the file system
   */
  private async saveChanges(): Promise<void> {
    try {
      console.log(`💾 Saving changes...`);
      await this.project.save();
      console.log(`✅ Successfully saved all changes`);
    } catch (error) {
      throw new Error(`Failed to save changes: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Main execution method
   */
  public async execute(): Promise<ConsolidationResult> {
    try {
      console.log(`🚀 Starting TypeScript imports consolidation for: ${this.config.targetPath}`);

      // Process source files
      const { processedFiles, organizedFiles, errors } = this.processSourceFiles();

      // Save changes
      await this.saveChanges();

      // Report results
      console.log(`\n📊 Import consolidation complete:`);
      console.log(`✅ Processed files: ${processedFiles}`);
      console.log(`📋 Organized files: ${organizedFiles}`);

      if (errors.length > 0) {
        console.error(`❌ Errors encountered: ${errors.length}`);
        errors.forEach((error) => console.error(`   - ${error}`));
      }

      return {
        success: errors.length === 0,
        processedFiles,
        organizedFiles,
        errors,
      };
    } catch (error) {
      const errorMessage = `Error during execution: ${error instanceof Error ? error.message : String(error)}`;
      console.error(`❌ ${errorMessage}`);

      return {
        success: false,
        processedFiles: 0,
        organizedFiles: 0,
        errors: [errorMessage],
      };
    }
  }
}

/**
 * Main entry point
 */
async function main(): Promise<void> {
  const targetPath = process.argv[2];

  if (!targetPath) {
    console.error("❌ Usage: ts-node importsConsolidator.ts <path-to-target-directory>");
    process.exit(1);
  }

  try {
    const consolidator = new TypeScriptImportsConsolidator(targetPath);
    const result = await consolidator.execute();

    if (result.success) {
      console.log("🎉 TypeScript imports consolidation completed successfully!");
    } else {
      console.error("💥 TypeScript imports consolidation completed with errors");
      process.exit(1);
    }
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

export { TypeScriptImportsConsolidator };
