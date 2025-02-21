import { chmodSync, existsSync, mkdirSync, writeFileSync } from 'fs'
import { extractTo, humanSize, modeToString } from './tool'
import { dirname, join } from 'path'

const path = process.argv[2]
const MAX_FILE_COUNT = 32

if (!path) {
  console.log('usage:\neasy-archive <file> [dir]')
  process.exit()
}

const ret = extractTo(path)
if (!ret) {
  console.log(`failed to decode ${path}`)
  process.exit()
}

const { files, type } = ret
const infoList: string[][] = []
const keys = files.keys()
let totalSize = 0
for (const i of keys) {
  const file = files.get(i)
  if (!file) {
    continue
  }
  const { path, mode, isDir, buffer } = file
  totalSize += buffer.length
  const v = [
    modeToString(mode ?? 0, isDir),
    humanSize(buffer.length),
    path,
  ]
  infoList.push(v)
}
const sizeMaxLen = infoList.reduce(
  (pre, cur) => Math.max(pre, cur[1].length),
  0,
)
console.log(
  `${humanSize(totalSize)} of ${keys.length} files By ${type.toUpperCase()}`,
)

if (keys.length <= MAX_FILE_COUNT) {
  for (const [a, b, c] of infoList) {
    console.log(a, b.padStart(sizeMaxLen, ' '), c)
  }
}

const output = process.argv[3]
if (output) {
  console.log('decompress to', output)
  const pathMaxLen = keys.reduce(
    (pre, cur) => Math.max(pre, cur.length),
    0,
  )
  for (const i of keys) {
    const file = files.get(i)
    if (!file) {
      continue
    }
    const { path, buffer, isDir, mode } = file
    const outputPath = join(output, path).replaceAll('\\', '/')
    const outputDir = dirname(outputPath)
    if (!existsSync(outputDir)) {
      mkdirSync(outputDir, { recursive: true })
    }

    if (isDir && !existsSync(outputPath)) {
      mkdirSync(outputPath, { recursive: true })
    }

    if (!isDir) {
      writeFileSync(outputPath, buffer)
    }

    if (mode && process.platform !== 'win32') {
      chmodSync(outputPath, mode)
    }
    if (keys.length <= MAX_FILE_COUNT) {
      console.log(`${path.padEnd(pathMaxLen, ' ')} -> ${outputPath}`)
    }
  }
  if (keys.length > MAX_FILE_COUNT) {
    console.log(`decompress ${keys.length} files to ${output}`)
  }
}
