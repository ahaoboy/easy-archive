import React, { useState } from 'react'
import {
  decode,
  encode,
  extensions,
  File as WasmFile,
  Fmt,
  guess,
  humanSize,
  modeToString,
} from '@easy-install/easy-archive'
import { Button, Flex, Spin, Table, type TableProps } from 'antd'
import {
  DeleteOutlined,
  DownloadOutlined,
  InboxOutlined,
} from '@ant-design/icons'
import { Upload } from 'antd'

const { Dragger } = Upload

function downloadBinaryFile(fileName: string, content: ArrayBuffer): void {
  const blob = new Blob([content], { type: 'application/octet-stream' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  const name = fileName.split('/').at(-1) ?? fileName
  a.download = name
  a.href = url
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
}

const SupportFormat = [
  Fmt.Tar,
  Fmt.TarBz,
  Fmt.TarGz,
  Fmt.TarXz,
  Fmt.TarZstd,
  Fmt.Zip,
].map((i) => extensions(i)).flat()

export interface FileType {
  path: string
  mode: number | undefined
  buffer: Uint8Array
  size: string
  isDir: boolean
  lastModified: bigint | undefined | null
  key: string
  file: WasmFile
}

async function filesToData(
  fmt: Fmt,
  file: File,
): Promise<FileType[] | undefined> {
  const fileBuffer = new Uint8Array(await file.arrayBuffer())
  const decodeFiles = await decode(fmt, fileBuffer)
  if (!decodeFiles) {
    return
  }
  const v: FileType[] = []
  for (const item of decodeFiles) {
    const { path, mode, isDir, lastModified, buffer } = item
    const size = humanSize(buffer.length)
    v.push({
      key: path,
      path,
      isDir,
      mode,
      buffer,
      size,
      file: item,
      lastModified,
    })
  }
  return v
}

const App: React.FC = () => {
  const [data, setData] = useState<FileType[]>([])
  const [spinning, setSpinning] = React.useState(false)
  const [filename, setFilename] = useState('')

  const columns: TableProps<FileType>['columns'] = [
    {
      title: 'path',
      dataIndex: 'path',
      key: 'path',
    },
    {
      title: 'isDir',
      dataIndex: 'isDir',
      key: 'isDir',
      render: (_, { isDir }) => isDir.toString(),
    },
    {
      title: 'size',
      dataIndex: 'size',
      key: 'path',
      render: (_, { isDir, size }) => !isDir ? size : '',
    },
    {
      title: 'mode',
      key: 'path',
      dataIndex: 'mode',
      render: (_, { isDir, mode }) =>
        mode !== undefined
          ? `(0o${mode.toString(8).padStart(3, '0')}) ${
            modeToString(mode, isDir)
          }`
          : '',
    },
    {
      title: 'lastModified',
      key: 'path',
      dataIndex: 'lastModified',
      render: (_, { lastModified }) =>
        lastModified
          ? new Date(Number(lastModified) * 1000).toLocaleString()
          : '',
    },
    {
      title: 'download',
      key: 'path',
      dataIndex: 'download',
      render: (_, { path, buffer, isDir }) => (
        !isDir && (
          <Button
            key={path}
            icon={<DownloadOutlined />}
            onClick={() => {
              // console.log('download', buffer, path)
              downloadBinaryFile(path, buffer)
            }}
          >
          </Button>
        )
      ),
    },
    {
      title: 'delete',
      key: 'path',
      dataIndex: 'delete',
      render: (_, { path, isDir }) => (
        !isDir && (
          <Button
            key={path}
            icon={<DeleteOutlined />}
            onClick={() => {
              // console.log('delete', buffer, path)
              setData(data.filter((i) => i.path !== path))
            }}
          >
          </Button>
        )
      ),
    },
  ]

  const downloadZip = () => {
    // console.log('downloadZip', data)
    let zip = encode(
      Fmt.Zip,
      data.map((i) =>
        new WasmFile(i.path, i.buffer, i.mode, i.isDir, i.lastModified)
      ),
    )
    if (zip) {
      let zipName = filename
      for (const i of extensions(guess(filename)!)) {
        if (zipName.endsWith(i)) {
          zipName = zipName.replace(i, '.zip')
        }
      }

      downloadBinaryFile(filename, zip)
    }
  }
  return (
    <Flex
      className='main'
      vertical
      gap='middle'
      justify='space-around'
      align='center'
    >
      <Dragger
        name='file'
        action='*'
        customRequest={(e) => e.onSuccess?.(true)}
        showUploadList={false}
        onChange={async (info) => {
          setData([])
          setSpinning(true)
          const file = info.file.originFileObj
          if (!file) {
            return
          }
          const fmt = guess(file.name)
          if (!fmt) {
            return
          }
          const v = await filesToData(fmt, file)
          if (v?.length) {
            setData(v)
          }
          setSpinning(false)
          setFilename(file.name)
        }}
      >
        <p className='ant-upload-drag-icon'>
          <InboxOutlined />
        </p>
        <p className='ant-upload-text'>
          Click or drag archive file to this area to upload
        </p>
        <p className='ant-upload-hint'>
          Support format: {SupportFormat.join(', ')}
        </p>
      </Dragger>

      <Table<FileType>
        className='table'
        columns={columns}
        dataSource={data}
      />
      <Spin spinning={spinning} fullscreen />
      <Button
        type='primary'
        icon={<DownloadOutlined />}
        size={'large'}
        onClick={downloadZip}
      >
        Download {filename}
      </Button>
    </Flex>
  )
}

export default App
