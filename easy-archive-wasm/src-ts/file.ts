export class JsFile {
  constructor(
    public path: string,
    public buffer: Uint8Array,
    public mode: number | undefined,
    public isDir: boolean,
  ) {
  }
}
export class JsFiles {
  constructor(public files: Record<string, JsFile> = {}) {}
  get(path: string): JsFile | undefined {
    return this.files[path]
  }
  insert(path: string, file: JsFile): JsFile | undefined {
    return this.files[path] = file
  }
  keys() {
    return Object.keys(this.files)
  }
}
