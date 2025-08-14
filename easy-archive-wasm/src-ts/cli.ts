import {
  chmodSync,
  existsSync,
  mkdirSync,
  readdirSync,
  readFileSync,
  statSync,
  writeFileSync,
} from "fs";
import { dirname, join, relative, resolve, sep } from "path";
import { encode, File, guess } from "./index";
import { humanSize, modeToString } from "./index";
import { extractTo } from "./tool";

// Function to collect files and directories recursively, skipping symlinks
function collectFiles(inputPath: string): File[] {
  const resolvedPath = resolve(inputPath);
  const files: File[] = [];

  // If input is a file, process it directly
  if (statSync(resolvedPath).isFile()) {
    const buffer = readFileSync(resolvedPath);
    const fileName = resolvedPath.split(sep).pop() || "";
    files.push(
      new File(
        fileName,
        new Uint8Array(buffer),
        undefined,
        false,
        undefined,
      ),
    );

    return files;
  }

  // If input is a directory, process recursively
  if (statSync(resolvedPath).isDirectory()) {
    collectFilesRecursive(resolvedPath, resolvedPath, files);
  }

  return files;
}

// Recursive helper function to collect files and directories
function collectFilesRecursive(
  basePath: string,
  currentPath: string,
  files: File[],
): void {
  for (const entry of readdirSync(currentPath, { withFileTypes: true })) {
    const path = join(currentPath, entry.name);
    // Skip symlinks and other non-file/directory entries
    if (!entry.isFile() && !entry.isDirectory()) {
      continue;
    }

    const relPath = relative(basePath, path).replaceAll("\\", "/") ||
      entry.name;

    if (entry.isDirectory()) {
      files.push(
        new File(relPath, new Uint8Array(0), undefined, true, undefined),
      );
      collectFilesRecursive(basePath, path, files);
    } else if (entry.isFile()) {
      const buffer = readFileSync(path);
      files.push(
        new File(relPath, new Uint8Array(buffer), undefined, false, undefined),
      );
    }
  }
}

function main() {
  const [input, output] = process.argv.slice(2);

  if (!input || !output) {
    console.log("usage:\neasy-archive <input> <output>");
    console.log("input and output parameters are required");
    process.exit(1);
  }

  // Guess archive format for input and output
  const inputFmt = guess(input);
  const outputFmt = guess(output);

  // Handle compression or decompression
  if (inputFmt && !outputFmt) {
    // Decompression
    const ret = extractTo(input, output);
    if (!ret) {
      console.log(`failed to decode ${input}`);
      process.exit(1);
    }

    const { files, type } = ret;
    const infoList: [string, string, string][] = [];
    let totalSize = 0;

    for (const file of files) {
      totalSize += file.buffer.length;
      infoList.push([
        modeToString(file.mode ?? 0, file.isDir),
        humanSize(file.buffer.length),
        file.path,
      ]);
    }

    console.log(
      `decompress ${files.length} files(${humanSize(totalSize)
      }) to ${output} By ${type.toUpperCase()}`,
    );
    for (const file of files) {
      const outputPath = join(output, file.path).replaceAll("\\", "/");
      const outputDir = dirname(outputPath);

      if (!existsSync(outputDir)) {
        mkdirSync(outputDir, { recursive: true });
      }

      if (file.isDir && !existsSync(outputPath)) {
        mkdirSync(outputPath, { recursive: true });
      }

      if (!file.isDir) {
        writeFileSync(outputPath, file.buffer);
      }

      if (file.mode && process.platform !== "win32") {
        chmodSync(outputPath, file.mode);
      }
    }
  } else if (!inputFmt && outputFmt) {
    // Compression
    const inputPath = resolve(input);
    if (!existsSync(inputPath)) {
      console.log("input file or directory does not exist");
      process.exit(1);
    }

    const files = collectFiles(inputPath);
    const totalSize = files.reduce((sum, file) => sum + file.bufferSize, 0);
    const buffer = encode(outputFmt, files); // Assume encode function exists
    if (!buffer) {
      console.log(`failed to encode files to ${output}`);
      process.exit(1);
    }

    writeFileSync(output, buffer);
    console.log(
      `compressed ${files.length} files (${humanSize(totalSize)
      }) to ${output}(${humanSize(buffer.length)})`,
    );
  } else if (inputFmt && outputFmt) {
    console.log(
      "both input and output are archive formats, please choose one as a directory",
    );
    process.exit(1);
  } else {
    console.log(
      "cannot identify input and output formats, at least one must be an archive format",
    );
    process.exit(1);
  }
}

main();
