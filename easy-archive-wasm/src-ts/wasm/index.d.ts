/* tslint:disable */
/* eslint-disable */

export class File {
  /**
   * Create a new File entry
   *
   * # Arguments
   * * `path` - The relative path within the archive
   * * `buffer` - The file content
   * * `mode` - Optional Unix permissions
   * * `is_dir` - Whether this is a directory
   * * `last_modified` - Optional modification timestamp
   */
  constructor(path: string, buffer: Uint8Array, mode: number | null | undefined, is_dir: boolean, last_modified?: bigint | null);
  /**
   * Clone the File (WASM only)
   */
  clone(): File;
  /**
   * Unix file permissions (e.g., 0o755 for rwxr-xr-x)
   */
  get mode(): number | undefined;
  /**
   * Unix file permissions (e.g., 0o755 for rwxr-xr-x)
   */
  set mode(value: number | null | undefined);
  /**
   * Whether this entry represents a directory
   */
  isDir: boolean;
  /**
   * Last modification time as Unix timestamp (seconds since epoch)
   */
  get lastModified(): bigint | undefined;
  /**
   * Last modification time as Unix timestamp (seconds since epoch)
   */
  set lastModified(value: bigint | null | undefined);
  /**
   * Get the file buffer (WASM only)
   *
   * Note: This consumes the File to reduce memory consumption
   */
  buffer: Uint8Array;
  /**
   * Get the buffer size in bytes (WASM only)
   */
  readonly bufferSize: number;
  /**
   * Get the file path (WASM only)
   */
  path: string;
}

/**
 * Archive format enumeration
 *
 * Represents the supported archive formats. Each variant is conditionally
 * compiled based on the corresponding feature flag.
 */
export enum Fmt {
  /**
   * Plain tar archive format
   */
  Tar = 0,
  /**
   * Gzip-compressed tar archive (.tar.gz, .tgz)
   */
  TarGz = 1,
  /**
   * XZ-compressed tar archive (.tar.xz, .txz)
   */
  TarXz = 2,
  /**
   * Bzip2-compressed tar archive (.tar.bz2, .tbz2)
   */
  TarBz = 3,
  /**
   * Zstd-compressed tar archive (.tar.zst, .tzst, .tzstd)
   */
  TarZstd = 4,
  /**
   * ZIP archive format
   */
  Zip = 5,
  /**
   * 7z archive format
   */
  SevenZip = 6,
}

/**
 * Compresses multiple entries into a 7z archive in WebAssembly environment.
 *
 * This function creates a compressed archive from multiple file entries,
 * designed specifically for WASM targets.
 *
 * # Arguments
 * * `entries` - Vector of JavaScript strings representing file names/paths
 * * `datas` - Vector of Uint8Arrays containing the file data corresponding to entries
 */
export function compress(entries: string[], datas: Uint8Array[]): Uint8Array;

export function decode(fmt: Fmt, buffer: Uint8Array): File[] | undefined;

/**
 * Decompresses a 7z archive in WebAssembly environment.
 *
 * This function is specifically designed for WASM targets and uses JavaScript interop
 * to handle the decompression process with a callback function.
 *
 * # Arguments
 * * `src` - Uint8Array containing the compressed archive data
 * * `pwd` - Password string for encrypted archives (use empty string for unencrypted)
 * * `f` - JavaScript callback function to handle extracted entries
 */
export function decompress(src: Uint8Array, pwd: string, f: Function): void;

export function encode(fmt: Fmt, files: File[]): Uint8Array | undefined;

export function extensions(fmt: Fmt): string[];

export function guess(name: string): Fmt | undefined;

