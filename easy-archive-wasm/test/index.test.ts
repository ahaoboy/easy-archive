import { readdirSync, readFileSync } from 'fs'
import { join } from 'path'
import { expect, test } from 'vitest'
import { decode, extensions, File, Files, Fmt, guess } from '../src-ts'

const assetsDir = '../assets'
const distKey = 'mujs-build-0.0.11/dist-manifest.json'
test('decode', () => {
  for (const name of readdirSync(assetsDir)) {
    const p = join(assetsDir, name)
    const buffer = readFileSync(p)
    const fmt = guess(name)!
    const files = decode(fmt, buffer)!
    expect(files.keys().includes(distKey))
    const jsonBuf = files.get(distKey)?.buffer!
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

test('files', () => {
  const files = new Files()
  files.insert('a', new File('a', new Uint8Array(), 0, false))
  expect(files.keys()).toEqual(['a'])
  files.insert('b', new File('b', new Uint8Array(), 0, false))
  expect(files.keys()).toEqual(['a', 'b'])
})
