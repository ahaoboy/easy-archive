/* tslint:disable */
/* eslint-disable */
export function decode(name: string, buf: Uint8Array): Files | undefined;
export enum Fmt {
  Tar = 0,
  TarGz = 1,
  TarXz = 2,
  TarBz = 3,
  TarZstd = 4,
  Zip = 5,
}
export class File {
  private constructor();
  free(): void;
  static new(path: string, buffer: Uint8Array, mode?: string | null): File;
  get_buffer(): Uint8Array;
  get_path(): string;
  get_mode(): string | undefined;
}
export class Files {
  private constructor();
  free(): void;
  static new(): Files;
  get(path: string): File | undefined;
  insert(name: string, file: File): File | undefined;
  keys(): string[];
}

