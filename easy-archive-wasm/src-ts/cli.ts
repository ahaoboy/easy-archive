import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'fs'
import { decode } from './index'
import { basename, dirname, join } from 'path'

const path = process.argv[2]
const name = basename(path)
const buffer = new Uint8Array(readFileSync(path))
const files = decode(name, buffer)!
for (const i of files.keys()) {
  const file = files.get(i)
  if (!file) {
    continue
  }
  const path = file.get_path()
  const buffer = file.get_buffer()
  console.log(`${path}: ${buffer.length}`)
}

const output = process.argv[3]

if (output) {
  console.log("decompress to", output)
  for (const i of files.keys()) {
    const file = files.get(i)
    if (!file) {
      continue
    }
    const path = file.get_path()
    const buffer = file.get_buffer()
    const outputPath = join(output, path)
    const outputDir = dirname(outputPath)
    if (!existsSync(outputDir)) {
      mkdirSync(outputDir, { recursive: true })
    }

    if (buffer.length) {
      writeFileSync(outputPath, buffer)
    }

    console.log(`${path} -> ${outputPath}`)
  }
}
