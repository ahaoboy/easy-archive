import type { File as WasmFile } from "@easy-install/easy-archive";

export interface FileType {
  path: string;
  mode: number | undefined;
  buffer: Uint8Array;
  size: string;
  isDir: boolean;
  lastModified: bigint | undefined | null;
  key: string;
  file: WasmFile;
}
