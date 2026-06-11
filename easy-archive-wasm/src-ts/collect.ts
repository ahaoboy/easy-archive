import { readdirSync, readFileSync, statSync } from "fs";
import { join, relative, resolve, sep } from "path";
import { File } from "./wasm";

/**
 * Recursively collect all files and directories under `rootDir` as `File` objects.
 * Returns an empty array if the path does not exist or is neither file nor directory.
 */
export function collectFiles(rootDir: string): File[] {
  const resolved = resolve(rootDir);
  const files: File[] = [];

  try {
    const stat = statSync(resolved);
    if (stat.isFile()) {
      const buffer = readFileSync(resolved);
      const name = resolved.split(sep).pop() || "";
      files.push(new File(name, new Uint8Array(buffer), undefined, false, undefined));
      return files;
    }
    if (stat.isDirectory()) {
      walkDir(resolved, resolved, files);
    }
  } catch {
    // path doesn't exist or inaccessible — return empty
  }
  return files;
}

/** Internal recursive directory walker. */
function walkDir(basePath: string, currentPath: string, files: File[]): void {
  const entries = readdirSync(currentPath, { withFileTypes: true });
  for (const entry of entries) {
    if (!entry.isFile() && !entry.isDirectory()) continue;

    const fullPath = join(currentPath, entry.name);
    const relPath = relative(basePath, fullPath).replaceAll("\\", "/") || entry.name;

    if (entry.isDirectory()) {
      files.push(new File(relPath, new Uint8Array(0), undefined, true, undefined));
      walkDir(basePath, fullPath, files);
    } else {
      const buffer = readFileSync(fullPath);
      files.push(new File(relPath, new Uint8Array(buffer), undefined, false, undefined));
    }
  }
}
