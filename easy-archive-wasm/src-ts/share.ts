export function modeToString(mode: number, isDir: boolean): string {
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

export function humanSize(bytes: number): string {
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
