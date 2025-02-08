/* tslint:disable */
/* eslint-disable */
export function guess(name: string): Fmt | undefined;
export function decode(fmt: Fmt, buffer: Uint8Array): Files | undefined;
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
  static new(path: string, buffer: Uint8Array, mode?: number | null): File;
  get_buffer(): Uint8Array;
  get_path(): string;
  get_mode(): number | undefined;
}
export class Files {
  private constructor();
  free(): void;
  static new(): Files;
  get(path: string): File | undefined;
  insert(name: string, file: File): File | undefined;
  keys(): string[];
}

