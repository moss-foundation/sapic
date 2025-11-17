/**
 * TypeScript Imports Resolver
 *
 * Automatically resolves and adds missing imports to generated TypeScript models.
 * This tool analyzes TypeScript files and their dependencies to fix import statements.
 *
 * Usage: ts-node importsResolver.ts <path-to-target-directory>
 *
 * @example
 * ```bash
 * ts-node importsResolver.ts ./crates/moss-workspace
 * ```
 */

import { existsSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { Project, SourceFile } from "ts-morph";

const KNOWN_PACKAGES = [
  "base",
  "moss-app",
  "moss-common",
  "moss-environment",
  "moss-workspace",
  "moss-project",
  "moss-activity-broadcaster",
  "moss-bindingutils",
  "ipc",
  "moss-git",
  "moss-user",
  "moss-language",
  "moss-extension",
];

interface Config {
  readonly targetPath: string;
  readonly tsConfigFileName: string;
  readonly nodeModulesPath: string;
  readonly repoScope: string;
}

interface ResolverResult {
  readonly success: boolean;
  readonly processedFiles: number;
  readonly resolvedImports: number;
  readonly errors: string[];
}

class TypeScriptImportsResolver {
  private readonly config: Config;
  private readonly project: Project;
  private readonly knownPackages: readonly string[];

  constructor(targetPath: string) {
    this.config = {
      targetPath: resolve(targetPath),
      tsConfigFileName: "tsconfig.json",
      nodeModulesPath: "node_modules",
      repoScope: "@repo",
    };

    // Known packages that might contain type definitions
    this.knownPackages = KNOWN_PACKAGES;

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
        skipFileDependencyResolution: false,
      });
    }

    try {
      return new Project({
        tsConfigFilePath: tsConfigPath,
        useInMemoryFileSystem: false,
        skipFileDependencyResolution: false,
      });
    } catch (error) {
      console.warn(
        `⚠️  Failed to load TypeScript config, using default: ${error instanceof Error ? error.message : String(error)}`
      );
      return new Project({
        useInMemoryFileSystem: false,
        skipFileDependencyResolution: false,
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
   * Discovers and adds available package source files to the project
   */
  private addAvailablePackages(): number {
    let addedPackages = 0;
    const nodeModulesPath = join(this.config.targetPath, this.config.nodeModulesPath);

    if (!existsSync(nodeModulesPath)) {
      console.warn(`⚠️  Node modules directory not found: ${nodeModulesPath}`);
      return addedPackages;
    }

    for (const packageName of this.knownPackages) {
      const packagePath = join(nodeModulesPath, this.config.repoScope, packageName, "index.ts");

      if (existsSync(packagePath)) {
        try {
          this.project.addSourceFileAtPathIfExists(packagePath);
          console.log(`📦 Added package: ${this.config.repoScope}/${packageName}`);
          addedPackages++;
        } catch (error) {
          console.warn(
            `⚠️  Failed to add package ${packageName}: ${error instanceof Error ? error.message : String(error)}`
          );
        }
      } else {
        console.log(`📦 Package not found: ${this.config.repoScope}/${packageName}`);
      }
    }

    return addedPackages;
  }

  /**
   * Resolves source file dependencies
   */
  private async resolveSourceFileDependencies(): Promise<void> {
    try {
      console.log(`🔍 Resolving source file dependencies...`);
      this.project.resolveSourceFileDependencies();
      console.log(`✅ Successfully resolved source file dependencies`);
    } catch (error) {
      console.warn(
        `⚠️  Warning: Failed to resolve some dependencies: ${error instanceof Error ? error.message : String(error)}`
      );
      // Don't throw here as this is not always critical
    }
  }

  /**
   * Fixes missing imports for a single source file
   */
  private fixFileImports(sourceFile: SourceFile): number {
    try {
      const filePath = sourceFile.getFilePath();
      const fileName = sourceFile.getBaseName();

      console.log(`🔧 Fixing imports for: ${fileName}`);

      const diagnostics = sourceFile.getPreEmitDiagnostics();
      const importIssues = diagnostics.filter(
        (d) => d.getMessageText().toString().includes("Cannot find") || d.getMessageText().toString().includes("Module")
      );

      if (importIssues.length > 0) {
        console.log(`   Found ${importIssues.length} potential import issues`);
      }

      sourceFile.fixMissingImports();

      // Check if any imports were actually added/fixed
      const newDiagnostics = sourceFile.getPreEmitDiagnostics();
      const resolvedIssues = Math.max(0, importIssues.length - newDiagnostics.length);

      if (resolvedIssues > 0) {
        console.log(`   ✅ Resolved ${resolvedIssues} import issues`);
      }

      return resolvedIssues;
    } catch (error) {
      console.error(`   ❌ Failed to fix imports: ${error instanceof Error ? error.message : String(error)}`);
      return 0;
    }
  }

  /**
   * Processes all source files to fix missing imports
   */
  private processSourceFiles(): { processedFiles: number; resolvedImports: number; errors: string[] } {
    const sourceFiles = this.project.getSourceFiles();
    const errors: string[] = [];
    let processedFiles = 0;
    let totalResolvedImports = 0;

    console.log(`📁 Processing ${sourceFiles.length} source files...`);

    for (const sourceFile of sourceFiles) {
      try {
        const resolvedImports = this.fixFileImports(sourceFile);
        totalResolvedImports += resolvedImports;
        processedFiles++;
      } catch (error) {
        const errorMessage = `Failed to process ${sourceFile.getBaseName()}: ${error instanceof Error ? error.message : String(error)}`;
        errors.push(errorMessage);
        console.error(`❌ ${errorMessage}`);
      }
    }

    return { processedFiles, resolvedImports: totalResolvedImports, errors };
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
  public async execute(): Promise<ResolverResult> {
    try {
      console.log(`🚀 Starting TypeScript imports resolution for: ${this.config.targetPath}`);

      // Add available packages
      const addedPackages = this.addAvailablePackages();
      console.log(`📦 Added ${addedPackages} available packages`);

      // Resolve dependencies
      await this.resolveSourceFileDependencies();

      // Process source files
      const { processedFiles, resolvedImports, errors } = this.processSourceFiles();

      // Save changes
      await this.saveChanges();

      // Report results
      console.log(`\n📊 Import resolution complete:`);
      console.log(`✅ Processed files: ${processedFiles}`);
      console.log(`🔧 Resolved imports: ${resolvedImports}`);

      if (errors.length > 0) {
        console.error(`❌ Errors encountered: ${errors.length}`);
        errors.forEach((error) => console.error(`   - ${error}`));
      }

      return {
        success: errors.length === 0,
        processedFiles,
        resolvedImports,
        errors,
      };
    } catch (error) {
      const errorMessage = `Error during execution: ${error instanceof Error ? error.message : String(error)}`;
      console.error(`❌ ${errorMessage}`);

      return {
        success: false,
        processedFiles: 0,
        resolvedImports: 0,
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
    console.error("❌ Usage: ts-node importsResolver.ts <path-to-target-directory>");
    process.exit(1);
  }

  try {
    const resolver = new TypeScriptImportsResolver(targetPath);
    const result = await resolver.execute();

    if (result.success) {
      console.log("🎉 TypeScript imports resolution completed successfully!");
    } else {
      console.error("💥 TypeScript imports resolution completed with errors");
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
    console.error("Unhandled error:", error);
    process.exit(1);
  });
}

export { TypeScriptImportsResolver };
