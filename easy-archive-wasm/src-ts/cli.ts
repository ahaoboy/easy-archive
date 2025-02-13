import { chmodSync, existsSync, mkdirSync, writeFileSync } from 'fs'
import { extractTo } from './tool'
import { dirname, join } from 'path'

function modeToString(mode: number, isDir: boolean): string {
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
  const owner = rwxMapping[(mode >> 6) & 0b111]
  const group = rwxMapping[(mode >> 3) & 0b111]
  const others = rwxMapping[mode & 0b111]
  const d = isDir ? 'd' : '-'
  return `${d}${owner}${group}${others}`
}

function humanSize(bytes: number): string {
  if (bytes < 0) {
    throw new Error('Size must be non-negative')
  }

  const units = ['', 'K', 'M', 'G', 'T', 'P', 'E', 'Z', 'Y']
  let index = 0
  let size = bytes

  while (size >= 1024 && index < units.length - 1) {
    size /= 1024
    index++
  }

  return `${parseFloat(size.toPrecision(2))}${units[index]}`
}

const path = process.argv[2]

if (!path) {
  console.log('usage:\neasy-archive <file> [dir]')
  process.exit()
}

const ret = extractTo(path)
if (!ret) {
  console.log(`failed to decode ${path}`)
  process.exit()
}

const { files } = ret
const infoList: string[][] = []
for (const i of files.keys()) {
  const file = files.get(i)
  if (!file) {
    continue
  }
  const { path, buffer, mode } = file
  const v = [
    modeToString(mode ?? 0, file.isDir),
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
  console.log(a, b.padStart(sizeMaxLen, ' '), c)
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

    if (file.isDir && !existsSync(outputPath)) {
      mkdirSync(outputPath, { recursive: true })
    }

    if (buffer.length && !file.isDir) {
      writeFileSync(outputPath, buffer)
    }

    if (mode && process.platform !== 'win32') {
      chmodSync(outputPath, mode)
    }

    console.log(`${path} -> ${outputPath}`)
  }
}
