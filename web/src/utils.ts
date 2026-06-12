import {
  decode,
  extensions,
  Fmt,
  humanSize,
} from "@easy-install/easy-archive";
import type { FileType } from "./types";

/** Supported archive formats for reading (upload). */
export const SUPPORTED_READ_FORMATS: Fmt[] = [
  Fmt.Tar,
  Fmt.TarBz,
  Fmt.TarGz,
  Fmt.TarXz,
  Fmt.TarZstd,
  Fmt.Zip,
  Fmt.SevenZip,
];

/** Supported archive formats for writing (download). */
export const SUPPORTED_WRITE_FORMATS: { fmt: Fmt; label: string; ext: string }[] = [
  { fmt: Fmt.Zip, label: "zip", ext: ".zip" },
  { fmt: Fmt.Tar, label: "tar", ext: ".tar" },
  { fmt: Fmt.TarGz, label: "tar.gz", ext: ".tar.gz" },
  { fmt: Fmt.TarXz, label: "tar.xz", ext: ".tar.xz" },
  { fmt: Fmt.TarBz, label: "tar.bz2", ext: ".tar.bz2" },
  { fmt: Fmt.TarZstd, label: "tar.zst", ext: ".tar.zst" },
  { fmt: Fmt.SevenZip, label: "7z", ext: ".7z" },
];

/** Comma-separated list of supported file extensions for the upload hint. */
export const SUPPORTED_EXTENSIONS = SUPPORTED_READ_FORMATS
  .map((f) => extensions(f))
  .flat()
  .join(", ");

/**
 * Download binary content as a file in the browser.
 */
export function downloadBinaryFile(fileName: string, content: Uint8Array): void {
  const blob = new Blob([new Uint8Array(content)], {
    type: "application/octet-stream",
  });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  const name = fileName.split("/").at(-1) ?? fileName;
  a.download = name;
  a.href = url;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

/**
 * Decode an uploaded archive file into FileType entries.
 */
export async function filesToData(
  fmt: Fmt,
  file: File,
): Promise<FileType[] | undefined> {
  const fileBuffer = new Uint8Array(await file.arrayBuffer());
  const decodeFiles = await decode(fmt, fileBuffer);
  if (!decodeFiles) {
    return;
  }
  return decodeFiles.map((item) => {
    const { path, mode, isDir, lastModified, buffer } = item;
    return {
      key: path,
      path,
      isDir,
      mode: mode ?? undefined,
      buffer,
      size: humanSize(bufferSize),
      file: item,
      lastModified,
    };
  });
}

/**
 * Strip any known archive extension from a filename, returning just the base name.
 */
export function stripArchiveExt(filename: string): string {
  const allExts = SUPPORTED_READ_FORMATS
    .map((f) => extensions(f))
    .flat()
    .sort((a, b) => b.length - a.length);

  for (const ext of allExts) {
    const lower = filename.toLowerCase();
    if (lower.endsWith(ext)) {
      return filename.slice(0, -ext.length);
    }
  }
  // If no known extension, strip any extension after the last dot
  const dot = filename.lastIndexOf(".");
  return dot > 0 ? filename.slice(0, dot) : filename;
}

/**
 * Replace the extension of a filename with the target format's extension,
 * stripping any known archive extension first.
 */
export function replaceExt(filename: string, targetExt: string): string {
  // Collect all known extensions and sort by length descending so we match
  // the longest extension first (e.g. .tar.gz before .gz).
  const allExts = SUPPORTED_READ_FORMATS
    .map((f) => extensions(f))
    .flat()
    .sort((a, b) => b.length - a.length);

  for (const ext of allExts) {
    if (filename.endsWith(ext) || filename.endsWith(ext.toUpperCase())) {
      return filename.slice(0, -ext.length) + targetExt;
    }
  }
  return filename + targetExt;
}
