/**
 * Zod Schema Generator
 *
 * Converts TypeScript models to Zod schemas using ts-to-zod and applies
 * post-processing to handle imports and naming conventions.
 *
 * Usage: ts-node zodGenerator.ts <path-to-target-directory>
 *
 * @example
 * ```bash
 * ts-node zodGenerator.ts ./src/generated
 * ```
 */

import { exec } from "node:child_process";
import { existsSync, readdirSync, statSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { promisify } from "node:util";
import { ExportDeclaration, ImportDeclaration, Project, SourceFile } from "ts-morph";

const execAsync = promisify(exec);

interface Config {
  readonly targetPath: string;
  readonly bindingsDirectoryName: string;
  readonly excludedFiles: string[]; // We will skip generating zod files for constants
  readonly tsFileExtension: string;
  readonly zodFileExtension: string;
  readonly zodFileSuffix: string;
}

interface ProcessingResult {
  readonly success: boolean;
  readonly filePath: string;
  readonly zodPath?: string;
  readonly error?: string;
}

class ZodSchemaGenerator {
  private readonly config: Config;
  private readonly project: Project;

  constructor(targetPath: string) {
    this.config = {
      targetPath: resolve(targetPath),
      bindingsDirectoryName: "bindings",
      excludedFiles: ["constants.ts"],
      tsFileExtension: ".ts",
      zodFileExtension: ".zod.ts",
      zodFileSuffix: ".zod",
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
    const { targetPath, bindingsDirectoryName } = this.config;

    if (!existsSync(targetPath)) {
      throw new Error(`Target directory does not exist: ${targetPath}`);
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
  private discoverTypeScriptFiles(): string[] {
    const bindingsPath = join(this.config.targetPath, this.config.bindingsDirectoryName);

    try {
      const files = readdirSync(bindingsPath, { recursive: true, withFileTypes: true })
        .filter((dirent) => {
          if (!dirent.isFile()) return false;
          if (this.config.excludedFiles.includes(dirent.name)) return false;
          if (!dirent.name.endsWith(this.config.tsFileExtension)) return false;
          if (dirent.name.endsWith(this.config.zodFileExtension)) return false;
          if (dirent.name.startsWith(".")) return false;
          return true;
        })
        .map((dirent) => {
          // Handle nested directories
          const parentPath = dirent.parentPath || bindingsPath;
          return join(parentPath, dirent.name);
        })
        .sort();

      console.log(`🔍 Found ${files.length} TypeScript files to process`);
      return files;
    } catch (error) {
      throw new Error(`Failed to read bindings directory: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Converts a TypeScript type name to a Zod schema name
   * Example: UserType -> userTypeSchema
   */
  private convertToSchemaName(typeName: string): string {
    return typeName.charAt(0).toLowerCase() + typeName.slice(1) + "Schema";
  }

  /**
   * Converts a module specifier to point to the Zod version
   * Example: "./types" -> "./types.zod"
   */
  private convertToZodModule(moduleSpecifier: string): string {
    if (moduleSpecifier.startsWith("./")) {
      return moduleSpecifier + this.config.zodFileSuffix;
    }
    return moduleSpecifier;
  }

  /**
   * Post-processes the generated Zod file to fix imports and references
   */
  private async postProcessZodFile(tsPath: string, zodPath: string): Promise<void> {
    try {
      console.log(`🔧 Post-processing Zod file: ${basename(zodPath)}`);

      // Add source files to project
      this.project.addSourceFileAtPath(tsPath);
      this.project.addSourceFileAtPath(zodPath);

      const tsFile = this.project.getSourceFile(tsPath);
      let zodFile = this.project.getSourceFile(zodPath);

      if (!tsFile || !zodFile) {
        throw new Error(`Failed to load source files: ${tsPath} or ${zodPath}`);
      }

      // Process imports from the original TypeScript file
      const importDeclarations = tsFile.getImportDeclarations();

      for (const importDecl of importDeclarations) {
        await this.processImportDeclaration(importDecl, zodFile);
      }

      // Add ts-nocheck comment if file uses recursive schemas
      this.suppressRecursiveSchemaErrors(zodFile);

      // Organize imports and save
      zodFile.organizeImports();
      await this.project.save();

      console.log(`   ✅ Successfully post-processed: ${basename(zodPath)}`);
    } catch (error) {
      throw new Error(`Failed to post-process Zod file: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Adds @ts-nocheck comment if file uses recursive schemas to suppress TypeScript errors
   * This prevents "Type instantiation is excessively deep" errors for recursive types
   */
  private suppressRecursiveSchemaErrors(zodFile: SourceFile): void {
    const fileText = zodFile.getFullText();

    // Check if file uses recursive schemas (jsonValueSchema or jsonRecordValueSchema)
    const usesRecursiveSchema = fileText.includes("jsonValueSchema") || fileText.includes("jsonRecordValueSchema");

    if (usesRecursiveSchema && !fileText.includes("@ts-nocheck")) {
      // Add @ts-nocheck comment at the beginning of the file (after initial comment)
      const firstNode = zodFile.getFirstChildByKind(1); // Get first statement
      if (firstNode) {
        zodFile.insertText(
          0,
          '// @ts-nocheck - File uses recursive schemas that may cause "Type instantiation is excessively deep" errors\n'
        );
      }
    }
  }

  /**
   * Processes a single import declaration and updates the Zod file accordingly
   */
  private async processImportDeclaration(importDecl: ImportDeclaration, zodFile: SourceFile): Promise<void> {
    const moduleSpecifier = importDecl.getModuleSpecifierValue();
    if (!moduleSpecifier) return;

    const zodModuleSpecifier = this.convertToZodModule(moduleSpecifier);
    const namedImports = importDecl.getNamedImports();

    for (const namedImport of namedImports) {
      const typeName = namedImport.getName();
      const schemaName = this.convertToSchemaName(typeName);

      // Add import for the Zod schema
      zodFile.addImportDeclaration({
        namedImports: [schemaName],
        moduleSpecifier: zodModuleSpecifier,
      });

      // Remove any existing variable declaration with the same name
      const existingDeclaration = zodFile.getVariableDeclaration(schemaName);
      if (existingDeclaration) {
        existingDeclaration.remove();
      }
    }
  }

  /**
   * Generates Zod schema for a single TypeScript file
   */
  private async processTypeScriptFile(filePath: string): Promise<ProcessingResult> {
    try {
      console.log(`⚙️  Processing: ${basename(filePath)}`);

      const dirName = dirname(filePath);
      const zodFilename = basename(filePath, this.config.tsFileExtension) + this.config.zodFileExtension;
      const zodPath = join(dirName, zodFilename);

      // Run ts-to-zod command with proper working directory
      const command = `npx ts-to-zod "${basename(filePath)}" "${zodFilename}"`;
      console.log(`   🚀 Executing: ${command} (in directory: ${dirName})`);

      const { stdout, stderr } = await execAsync(command, { cwd: dirName });

      if (stderr && !stderr.includes("Warning")) {
        console.warn(`   ⚠️  Warnings during generation: ${stderr}`);
      }

      if (stdout) {
        console.log(`   📄 ts-to-zod output: ${stdout}`);
      }

      // Post-process the generated file
      await this.postProcessZodFile(filePath, zodPath);

      return {
        success: true,
        filePath,
        zodPath,
      };
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.error(`   ❌ Failed to process ${basename(filePath)}: ${errorMessage}`);

      return {
        success: false,
        filePath,
        error: errorMessage,
      };
    }
  }

  /**
   * Main execution method
   */
  public async execute(): Promise<void> {
    try {
      console.log(`🚀 Starting Zod schema generation for: ${this.config.targetPath}`);

      const typeScriptFiles = this.discoverTypeScriptFiles();

      if (typeScriptFiles.length === 0) {
        console.warn("⚠️  No TypeScript files found to process");
        return;
      }

      console.log(`📁 Processing ${typeScriptFiles.length} files...`);

      const results: ProcessingResult[] = [];

      // Process files sequentially to avoid overwhelming the system
      for (const filePath of typeScriptFiles) {
        const result = await this.processTypeScriptFile(filePath);
        results.push(result);
      }

      // Report results
      const successful = results.filter((r) => r.success);
      const failed = results.filter((r) => !r.success);

      console.log(`\n📊 Processing complete:`);
      console.log(`✅ Successfully processed: ${successful.length} files`);

      if (failed.length > 0) {
        console.error(`❌ Failed to process: ${failed.length} files`);
        failed.forEach((result) => {
          console.log(`   - ${basename(result.filePath)}: ${result.error}`);
        });
      }

      if (failed.length > 0) {
        throw new Error(`Failed to process ${failed.length} out of ${results.length} files`);
      }
    } catch (error) {
      console.error(`❌ Error during execution: ${error instanceof Error ? error.message : String(error)}`);
      throw error;
    }
  }
}

async function main(): Promise<void> {
  const targetPath = process.argv[2];

  if (!targetPath) {
    console.error("❌ Usage: ts-node zodGenerator.ts <path-to-target-directory>");
    process.exit(1);
  }

  try {
    const generator = new ZodSchemaGenerator(targetPath);
    await generator.execute();
    console.log("🎉 Zod schema generation completed successfully!");
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

export { ZodSchemaGenerator };
