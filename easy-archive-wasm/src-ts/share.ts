import { dirname, isAbsolute, resolve, sep } from "path";

export function modeToString(mode: number, isDir: boolean): string {
  const rwxMapping = [
    "---",
    "--x",
    "-w-",
    "-wx",
    "r--",
    "r-x",
    "rw-",
    "rwx",
  ];
  const owner = rwxMapping[(mode >> 6) & 0b111];
  const group = rwxMapping[(mode >> 3) & 0b111];
  const others = rwxMapping[mode & 0b111];
  const d = isDir ? "d" : "-";
  return `${d}${owner}${group}${others}`;
}

export function humanSize(bytes: number): string {
  if (bytes < 0) {
    throw new Error("Size must be non-negative");
  }

  const units = ["", "K", "M", "G", "T", "P", "E", "Z", "Y"];
  let index = 0;
  let size = bytes;

  while (size >= 1024 && index < units.length - 1) {
    size /= 1024;
    index++;
  }

  return `${parseFloat(size.toPrecision(2))}${units[index]}`;
}

/**
 * Whether a symlink target stays inside the extraction root.
 *
 * Archives are untrusted input: a link pointing outside (`../../etc`) could
 * redirect everything extracted after it, so such links are not created.
 */
export function isSafeLink(
  outputDir: string,
  outputPath: string,
  target: string,
): boolean {
  if (isAbsolute(target)) return false;
  const root = resolve(outputDir);
  const resolved = resolve(dirname(outputPath), target);
  return resolved === root || resolved.startsWith(root + sep);
}
