export class JsFile {
  constructor(
    public path: string,
    public buffer: Uint8Array,
    public mode: number | undefined,
    public isDir: boolean,
  ) {
  }
}
