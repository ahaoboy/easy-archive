import * as path from 'path'
import * as fs from 'fs'
import { homedir, tmpdir } from 'os'
import {
  createFiles,
  downloadToFile,
  extractTo,
  toMsysPath,
} from '../src-ts/tool'
import { expect, test } from 'vitest'
import { join } from 'path'

test('extractTo zip', async () => {
  const url =
    'https://github.com/ahaoboy/ansi2/releases/download/v0.2.11/ansi2-x86_64-pc-windows-msvc.zip'
  const filePath = path.join(tmpdir(), 'ansi2-x86_64-pc-windows-msvc.zip')
  const testDir = 'easy-setup-test'
  const installDir = path.join(homedir(), testDir)
  await downloadToFile(url, filePath)
  extractTo(filePath, installDir)
  const ansi2Path = path.join(homedir(), testDir, 'ansi2.exe')
  expect(fs.existsSync(ansi2Path)).toEqual(true)
}, 100_000)

test('extractTo tar.gz', async () => {
  // only test on linux
  if (process.platform === 'win32') return
  const url =
    'https://github.com/ahaoboy/ansi2/releases/download/v0.2.11/ansi2-aarch64-apple-darwin.tar.gz'
  const filePath = path.join(tmpdir(), 'ansi2-aarch64-apple-darwin.tar.gz')
  const testDir = 'easy-setup-test'
  const installDir = path.join(homedir(), testDir)
  await downloadToFile(url, filePath)
  extractTo(filePath, installDir)
  const ansi2Path = path.join(homedir(), testDir, 'ansi2')
  expect(fs.existsSync(ansi2Path)).toEqual(true)
}, 100_000)

test('extractTo', async () => {
  const url =
    'https://github.com/ahaoboy/mujs-build/archive/refs/tags/v0.0.4.zip'
  const tmpPath = await downloadToFile(url)
  const tmpDir = extractTo(tmpPath)!.outputDir
  expect(fs.existsSync(join(tmpDir, 'mujs-build-0.0.4', 'dist-manifest.json')))
    .toEqual(true)
})
test('toMsysPath', () => {
  for (
    const [a, b] of [
      ['c:\\a\\b', '/c/a/b'],
      ['c:/a/b', '/c/a/b'],
      ['C:/a/b', '/c/a/b'],
    ]
  ) {
    expect(toMsysPath(a)).toEqual(b)
  }
})
test('createFiles', () => {
  const files = createFiles('src-ts')
  expect(files.keys().length > 0).toEqual(true)
  const ei = files.get('wasm/index.d.ts')!.buffer
  const txt = Buffer.from(ei).toString()
  expect(txt.includes('decode')).toEqual(true)
})

test('download mujs', async () => {
  const url =
    'https://github.com/ahaoboy/mujs-build/releases/download/v0.0.1/mujs-x86_64-unknown-linux-gnu.tar.gz'
  const tmpPath = await downloadToFile(url)
  const tmpDir = extractTo(tmpPath)!.outputDir
  for (
    const i of [
      'mujs',
      'libmujs.a',
      'mujs.pc',
      'mujs-pp',
    ]
  ) {
    const p = join(tmpDir, i)
    expect(fs.existsSync(p)).toEqual(true)
  }
})
