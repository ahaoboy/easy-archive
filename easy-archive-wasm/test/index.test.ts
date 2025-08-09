import { readdirSync, readFileSync } from 'fs'
import { join } from 'path'
import { expect, test } from 'vitest'
import { decode, encode, extensions, File, Fmt, guess } from '../src-ts'
import { createFiles } from '../src-ts/tool'

const assetsDir = '../assets'
const distKey = 'mujs-build-0.0.11/dist-manifest.json'
test('decode', () => {
  for (const name of readdirSync(assetsDir)) {
    const p = join(assetsDir, name)
    const buffer = readFileSync(p)
    const fmt = guess(name)!
    const files = decode(fmt, buffer)!
    const jsonBuf = files.find((i) => i.path === distKey)?.buffer!
    const str = Buffer.from(jsonBuf).toString()
    const json = JSON.parse(str)
    expect(json['artifacts']['mujs-x86_64-unknown-linux-gnu.tar.xz']['name'])
      .toEqual('mujs-x86_64-unknown-linux-gnu.tar.xz')
  }
})

test('extension', () => {
  for (
    const i of [
      Fmt.Tar,
      Fmt.TarBz,
      Fmt.TarGz,
      Fmt.TarXz,
      Fmt.TarZstd,
      Fmt.Zip,
    ]
  ) {
    for (const ext of extensions(i)) {
      expect(guess(ext)).toEqual(i)
    }
  }
})

test('encode decode', () => {
  // FIXME: support encode bz
  for (const i of [Fmt.Zip, Fmt.Tar, Fmt.TarGz, Fmt.TarXz, Fmt.TarZstd]) {
    const v: File[] = createFiles(assetsDir).map(i => {
      return new File(i.path, i.buffer, i.mode, i.isDir, i.lastModified)
    })

    const compress = encode(i, v)
    expect(compress?.length).toBeTruthy()

    const decodeFiles = decode(i, compress!)
    console.log(extensions(i), compress?.length)
    expect(decodeFiles?.length).toBeTruthy()
  }
})

