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
import { decode, File, guess } from './wasm'
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

const free = () => {}

export function createFiles(dir: string): File[] {
  const files: File[] = []
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
        const file = {
          path: relativePath,
          buffer,
          mode: stat.mode,
          isDir: false,
          free,
        }
        files.push(file)
      }
    }
  }
  dfs(dir)
  return files
}

export function extractToByShell(
  compressedFilePath: string,
  outputDir?: string,
): undefined | { outputDir: string; files: File[] } {
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
      cmd: `tar -xJf "${compressedFilePath}" -C "${outputDir}"`,
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
  const files = createFiles(oriDir)
  if (needCopy && tmpDir !== oriDir) {
    cpSync(tmpDir, oriDir, { recursive: true })
  }
  return { outputDir: oriDir, files }
}

// 256mb
const MAX_SIZE = 1024 * 1024 * 256
export function extractToByWasm(
  compressedFilePath: string,
  outputDir?: string,
): undefined | { outputDir: string; files: File[] } {
  const buf = new Uint8Array(readFileSync(compressedFilePath))
  if (buf.length > MAX_SIZE) {
    return
  }
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
  const files = decode(fmt, buf)
  if (!files) {
    return undefined
  }
  const jsFiles: File[] = []
  for (const file of files) {
    const { path, mode, isDir, buffer } = file
    jsFiles.push({ path, buffer, mode, isDir, free })
    const outputPath = join(outputDir, path)
    if (path.endsWith('/') || isDir) {
      mkdirSync(outputPath, { recursive: true })
      continue
    }
    const dir = dirname(outputPath)
    if (!existsSync(dir)) {
      mkdirSync(dir, { recursive: true })
    }
    writeFileSync(outputPath, buffer)

    if (mode && process.platform !== 'win32') {
      chmodSync(outputPath, mode)
    }
  }
  return { outputDir, files: jsFiles }
}

export function extractTo(compressedFilePath: string, outputDir?: string): {
  outputDir: string
  files: File[]
  type: 'wasm' | 'shell'
} | undefined {
  try {
    const r = extractToByWasm(compressedFilePath, outputDir)
    if (r) {
      return { ...r, type: 'wasm' }
    }
  } catch {
  }
  const r = extractToByShell(compressedFilePath, outputDir)
  if (r) {
    return { ...r, type: 'shell' }
  }
}

export function getFetchOption() {
  const headers: HeadersInit = {
    'User-Agent': 'GitHub Actions',
  }
  if (process.env.GITHUB_TOKEN) {
    headers.Authorization = `token ${process.env.GITHUB_TOKEN}`
  }
  return {
    headers,
  }
}
export async function downloadToFile(url: string, outputPath?: string) {
  if (!outputPath) {
    const name = url.split('/').at(-1)!
    const dir = join(tmpdir(), randomId())
    if (!existsSync(dir)) {
      mkdirSync(dir, { recursive: true })
    }
    outputPath = join(dir, name)
  }
  outputPath = outputPath.replaceAll('\\', '/')
  const dir = outputPath.split('/').slice(0, -1).join('/')
  if (!existsSync(dir)) {
    mkdirSync(dir, { recursive: true })
  }
  const response = await fetch(url, getFetchOption())
  const buf = await response.arrayBuffer()
  writeFileSync(outputPath, Buffer.from(buf))
  return outputPath
}
