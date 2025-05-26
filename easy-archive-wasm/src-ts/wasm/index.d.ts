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
  constructor(path: string, buffer: Uint8Array, mode: number | null | undefined, is_dir: boolean, last_modified?: bigint | null);
  get mode(): number | undefined;
  set mode(value: number | null | undefined);
  isDir: boolean;
  get lastModified(): bigint | undefined;
  set lastModified(value: bigint | null | undefined);
  buffer: Uint8Array;
  path: string;
}

