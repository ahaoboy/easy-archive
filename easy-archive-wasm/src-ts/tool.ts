import {
  chmodSync,
  cpSync,
  existsSync,
  mkdirSync,
  readdirSync,
  readFileSync,
  statSync,
  writeFileSync,
} from 'fs'
import { decode, File, Files, guess } from './wasm'
import { dirname, join, relative } from 'path'
import { tmpdir } from 'os'
import { execSync } from 'child_process'

export function isMsys() {
  return !!process.env['MSYSTEM']
}

export function toMsysPath(s: string): string {
  s = s.replaceAll('\\', '/')
  s = s.replace(/^([A-Za-z]):\//, (_, drive) => `/${drive.toLowerCase()}/`)
  return s
}

export function randomId() {
  return Math.random().toString(36).slice(2)
}

export function createFiles(dir: string): Files {
  const files = new Files()
  async function dfs(currentPath: string) {
    const entries = readdirSync(currentPath)
    for (const entry of entries) {
      const fullPath = join(currentPath, entry)
      const stat = statSync(fullPath)
      if (stat.isDirectory()) {
        // ignore empty dir
        dfs(fullPath)
      } else if (stat.isFile()) {
        const relativePath = relative(dir, fullPath).replaceAll('\\', '/')
        const buffer = readFileSync(fullPath)
        const file = new File(relativePath, buffer, stat.mode, false)
        files.insert(relativePath, file)
      }
    }
  }
  dfs(dir)
  return files
}

export function extractToByShell(
  compressedFilePath: string,
  outputDir?: string,
): undefined | { outputDir: string; files: Files } {
  const tmpDir = join(tmpdir(), randomId())
  let oriDir = outputDir ?? tmpDir
  const needCopy = !!outputDir

  outputDir = tmpDir
  if (!existsSync(outputDir)) {
    mkdirSync(outputDir, { recursive: true })
  }

  if (isMsys() && !compressedFilePath.endsWith('.zip')) {
    compressedFilePath = toMsysPath(compressedFilePath)
    outputDir = toMsysPath(outputDir)
  }
  if (!existsSync(oriDir)) {
    mkdirSync(oriDir, { recursive: true })
  }
  const rules = [
    {
      ext: ['.zip'],
      cmd: process.platform !== 'win32'
        ? `unzip -o "${compressedFilePath}" -d "${outputDir}"`
        : `powershell -c "Expand-Archive -Path ${compressedFilePath} -DestinationPath  ${outputDir} -Force"`,
    },
    {
      ext: ['.tar', '.tar.xz'],
      cmd: `tar -xf "${compressedFilePath}" -C "${outputDir}"`,
    },
    {
      ext: ['.tar.gz', '.tgz'],
      cmd: `tar -xzvf "${compressedFilePath}" -C "${outputDir}"`,
    },
    {
      ext: ['.tar.bz2'],
      cmd: `tar -xjf "${compressedFilePath}" -C "${outputDir}"`,
    },
    { ext: ['.7z'], cmd: `7z x "${compressedFilePath}" -o"${outputDir}"` },
    { ext: ['.rar'], cmd: `unrar x "${compressedFilePath}" "${outputDir}"` },
    { ext: ['.rar'], cmd: `unrar x "${compressedFilePath}" "${outputDir}"` },
  ] as const

  for (const { ext, cmd } of rules) {
    for (const e of ext) {
      if (compressedFilePath.endsWith(e)) {
        execSync(cmd)
      }
    }
  }
  const files = createFiles(outputDir)
  if (needCopy && tmpDir !== oriDir) {
    cpSync(tmpDir, oriDir, { recursive: true })
  }
  return { outputDir: oriDir, files }
}

export function extractToByWasm(
  compressedFilePath: string,
  outputDir?: string,
): undefined | { outputDir: string; files: Files } {
  const fmt = guess(compressedFilePath)
  if (!outputDir) {
    outputDir = join(tmpdir(), randomId())
    if (!existsSync(outputDir)) {
      mkdirSync(outputDir, { recursive: true })
    }
  }
  if (!fmt) {
    console.log('extractTo not support this file type')
    return undefined
  }
  if (!existsSync(outputDir)) {
    mkdirSync(outputDir, { recursive: true })
  }
  const buf = new Uint8Array(readFileSync(compressedFilePath))
  const files = decode(fmt, buf)
  if (!files) {
    console.log('failed to decode')
    return undefined
  }
  for (const i of files.keys()) {
    const file = files.get(i)
    if (!file) {
      continue
    }
    const { path, mode, buffer } = file

    if (path.endsWith('/') || !buffer.length) {
      continue
    }

    const outputPath = join(outputDir, path)
    const dir = dirname(outputPath)
    if (!existsSync(dir)) {
      mkdirSync(dir, { recursive: true })
    }
    writeFileSync(outputPath, buffer)

    if (mode && process.platform !== 'win32') {
      chmodSync(outputPath, mode)
    }
  }
  return { outputDir, files }
}

export function extractTo(compressedFilePath: string, outputDir?: string): {
  outputDir: string
  files: Files
} | undefined {
  try {
    return extractToByWasm(compressedFilePath, outputDir)
  } catch {
    return extractToByShell(compressedFilePath, outputDir)
  }
}
