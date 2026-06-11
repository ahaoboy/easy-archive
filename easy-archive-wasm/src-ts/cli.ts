import {
  chmodSync,
  existsSync,
  mkdirSync,
  writeFileSync,
} from "fs";
import { dirname, join, resolve } from "path";
import { encode, guess } from "./index";
import { humanSize, modeToString } from "./index";
import { extractTo } from "./tool";
import { collectFiles } from "./collect";

// ── Helpers ──────────────────────────────────────────────────────────

/** Safely read a WASM File's mode, falling back to 0 if the getter throws. */
function safeMode(file: { mode?: number | null; isDir: boolean }): number {
  try {
    return file.mode ?? 0;
  } catch {
    return 0;
  }
}

/** Print a summary table of the archive contents. */
function printFileList(files: Array<{ path: string; buffer: Uint8Array | { length: number }; isDir: boolean; mode?: number | null }>): number {
  let totalSize = 0;
  for (const file of files) {
    totalSize += file.buffer.length;
    console.log(
      `${modeToString(safeMode(file), file.isDir).padEnd(11)} ${humanSize(file.buffer.length).padStart(8)} ${file.path}`,
    );
  }
  return totalSize;
}

/** Write files to an output directory, creating parent dirs and setting permissions. */
function writeExtractedFiles(outputDir: string, files: Array<{ path: string; buffer: Uint8Array; isDir: boolean; mode?: number | null }>): void {
  for (const file of files) {
    const outputPath = join(outputDir, file.path).replaceAll("\\", "/");
    const outputParent = dirname(resolve(outputPath));

    if (!existsSync(outputParent)) {
      mkdirSync(outputParent, { recursive: true });
    }

    if (file.isDir) {
      if (!existsSync(outputPath)) {
        mkdirSync(outputPath, { recursive: true });
      }
    } else if (file.buffer.length) {
      writeFileSync(outputPath, file.buffer);
    }

    const mode = safeMode(file);
    if (mode && process.platform !== "win32") {
      try { chmodSync(outputPath, mode); } catch { /* ignore */ }
    }
  }
}

// ── Main ─────────────────────────────────────────────────────────────

function main() {
  const [input, output] = process.argv.slice(2);

  if (!input || !output) {
    console.log("usage:\neasy-archive <input> <output>");
    console.log("input and output parameters are required");
    process.exit(1);
  }

  const inputFmt = guess(input);
  const outputFmt = guess(output);

  // ── Decompression: input is an archive, output is a directory ──────
  if (inputFmt && !outputFmt) {
    const ret = extractTo(input, output);
    if (!ret) {
      console.log(`failed to decode ${input}`);
      process.exit(1);
    }

    const { files, type, outputDir } = ret;
    const totalSize = printFileList(files);

    console.log(
      `\ndecompress ${files.length} files (${humanSize(totalSize)}) to ${outputDir} by ${type.toUpperCase()}`,
    );

    // extractTo already writes files for WASM; for shell we may need chmod.
    // Always ensure files are on disk at the requested output path.
    writeExtractedFiles(output || outputDir, files);
  } else if (!inputFmt && outputFmt) {
    // ── Compression: input is a directory/file, output is an archive ──
    const inputPath = resolve(input);
    if (!existsSync(inputPath)) {
      console.log("input file or directory does not exist");
      process.exit(1);
    }

    const files = collectFiles(inputPath);
    const totalSize = files.reduce((sum, f) => sum + f.bufferSize, 0);
    const buffer = encode(outputFmt, files);
    if (!buffer) {
      console.log(`failed to encode files to ${output}`);
      process.exit(1);
    }

    writeFileSync(output, buffer);
    console.log(
      `compressed ${files.length} files (${humanSize(totalSize)}) to ${output} (${humanSize(buffer.length)})`,
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
