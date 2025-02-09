import {
  chmodSync,
  existsSync,
  mkdirSync,
  readFileSync,
  writeFileSync,
} from 'fs'
import { decode, guess } from './index'
import { basename, dirname, join } from 'path'

function modeToString(mode: number, isDir: boolean): string {
  if (mode < 0 || mode > 0o777) {
    throw new Error('Invalid mode: must be in range 0 to 0o777')
  }

  const rwxMapping = [
    '---',
    '--x',
    '-w-',
    '-wx',
    'r--',
    'r-x',
    'rw-',
    'rwx',
  ]

  const owner = rwxMapping[(mode >> 6) & 0b111] // Owner permissions
  const group = rwxMapping[(mode >> 3) & 0b111] // Group permissions
  const others = rwxMapping[mode & 0b111] // Others permissions
  const d = isDir ? 'd' : '-'
  return `${d}${owner}${group}${others}`
}

function humanSize(bytes: number): string {
  if (bytes < 0) {
    throw new Error('Size must be non-negative')
  }

  const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB']
  let index = 0
  let size = bytes

  while (size >= 1024 && index < units.length - 1) {
    size /= 1024
    index++
  }

  return `${size.toFixed(2)} ${units[index]}`
}

const path = process.argv[2]

if (!path) {
  console.log('usage:\neasy-archive <file> [dir]')
  process.exit()
}

const name = basename(path)
const buffer = new Uint8Array(readFileSync(path))
const files = decode(guess(name)!, buffer)!

const infoList: string[][] = []
for (const i of files.keys()) {
  const file = files.get(i)
  if (!file) {
    continue
  }
  const { path, buffer, mode } = file
  const v = [
    modeToString(mode ?? 0, file.isDir()),
    humanSize(buffer.length),
    path,
  ]
  infoList.push(v)
}
const sizeMaxLen = infoList.reduce(
  (pre, cur) => Math.max(pre, cur[1].length),
  0,
)
for (const [a, b, c] of infoList) {
  console.log(a, b.padEnd(sizeMaxLen, ' '), c)
}

const output = process.argv[3]

if (output) {
  console.log('decompress to', output)
  for (const i of files.keys()) {
    const file = files.get(i)
    if (!file) {
      continue
    }
    const { path, buffer, mode } = file
    const outputPath = join(output, path)
    const outputDir = dirname(outputPath)
    if (!existsSync(outputDir)) {
      mkdirSync(outputDir, { recursive: true })
    }

    if (file.isDir() && !existsSync(outputPath)) {
      mkdirSync(outputPath, { recursive: true })
    }

    if (buffer.length && !file.isDir()) {
      writeFileSync(outputPath, buffer)
    }

    if (mode && process.platform !== 'win32') {
      chmodSync(outputPath, mode)
    }

    console.log(`${path} -> ${outputPath}`)
  }
}
