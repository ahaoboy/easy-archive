/* tslint:disable */
/* eslint-disable */
export function guess(name: string): Fmt | undefined;
export function extensions(fmt: Fmt): string[];
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
  free(): void;
  constructor(path: string, buffer: Uint8Array, mode?: number | null);
  isDir(): boolean;
  readonly buffer: Uint8Array;
  readonly path: string;
  readonly mode: number | undefined;
}
export class Files {
  private constructor();
  free(): void;
  static new(): Files;
  get(path: string): File | undefined;
  insert(name: string, file: File): File | undefined;
  keys(): string[];
}

