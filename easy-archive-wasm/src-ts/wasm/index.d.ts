/* tslint:disable */
/* eslint-disable */
export function guess(name: string): Fmt | undefined;
export function extensions(fmt: Fmt): string[];
export function decode(fmt: Fmt, buffer: Uint8Array): File[] | undefined;
export function encode(fmt: Fmt, files: File[]): Uint8Array | undefined;
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
  constructor(path: string, buffer: Uint8Array, mode: number | null | undefined, is_dir: boolean);
  get mode(): number | undefined;
  set mode(value: number | null | undefined);
  isDir: boolean;
  readonly buffer: Uint8Array;
  readonly path: string;
}

