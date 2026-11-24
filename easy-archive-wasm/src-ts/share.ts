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
 *
 * The path handling is self-contained instead of using `node:path`, because
 * this module is also bundled for the browser (where `path` does not exist).
 */
export function isSafeLink(
  outputDir: string,
  outputPath: string,
  target: string,
): boolean {
  if (isAbsolutePath(target)) return false;
  const root = normalizePath(outputDir);
  const dir = outputPath.slice(0, lastSeparator(outputPath) + 1);
  const resolved = normalizePath(`${dir}${target}`);
  return resolved === root || resolved.startsWith(`${root}/`);
}

/** A leading separator or a drive letter makes a path absolute. */
function isAbsolutePath(path: string): boolean {
  return /^([/\\]|[A-Za-z]:)/.test(path);
}

/** Index of the last `/` or `\`, or `-1` when there is none. */
function lastSeparator(path: string): number {
  return Math.max(path.lastIndexOf("/"), path.lastIndexOf("\\"));
}

/** Resolve `.` and `..` segments, using `/` as the separator. */
function normalizePath(path: string): string {
  const absolute = /^[/\\]/.test(path);
  const parts: string[] = [];
  for (const segment of path.replaceAll("\\", "/").split("/")) {
    if (segment === "" || segment === ".") continue;
    if (segment === "..") {
      if (parts.length > 0 && parts.at(-1) !== "..") parts.pop();
      else if (!absolute) parts.push("..");
      continue;
    }
    parts.push(segment);
  }
  return (absolute ? "/" : "") + parts.join("/");
}
