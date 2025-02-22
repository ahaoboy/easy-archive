import React, { useState } from 'react'
import {
  decode,
  guess,
  humanSize,
  modeToString,
} from '@easy-install/easy-archive'
import { Button, Spin, Table, type TableProps } from 'antd'
import { DownloadOutlined, InboxOutlined } from '@ant-design/icons'
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

const columns: TableProps<FileType>['columns'] = [
  {
    title: 'path',
    dataIndex: 'path',
    key: 'path',
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
    title: 'download',
    key: 'path',
    dataIndex: 'download',
    render: (_, { path, buffer, isDir }) => (
      !isDir && (
        <Button
          key={path}
          icon={<DownloadOutlined />}
          onClick={() => {
            console.log(buffer, path)
            downloadBinaryFile(path, buffer)
          }}
        >
        </Button>
      )
    ),
  },
]

export interface FileType {
  path: string
  mode: number | undefined
  buffer: Uint8Array
  size: string
  isDir: boolean
}

async function filesToData(file: File): Promise<FileType[] | undefined> {
  const fileBuffer = new Uint8Array(await file.arrayBuffer())
  const fmt = guess(file.name)
  if (!fmt) {
    return
  }
  const decodeFiles = await decode(fmt, fileBuffer)
  if (!decodeFiles) {
    return
  }
  const v: FileType[] = []
  for (const item of decodeFiles) {
    const { path, mode, isDir, buffer } = item
    const size = humanSize(buffer.length)
    v.push({ path, isDir, mode, buffer, size })
  }
  return v
}

const App: React.FC = () => {
  const [data, setData] = useState<FileType[]>([])
  const [spinning, setSpinning] = React.useState(false)
  return (
    <div className='main'>
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
          const v = await filesToData(file)
          if (v?.length) {
            setData(v)
          }
          setSpinning(false)
        }}
      >
        <p className='ant-upload-drag-icon'>
          <InboxOutlined />
        </p>
        <p className='ant-upload-text'>
          Click or drag file to this area to upload
        </p>
        <p className='ant-upload-hint'>
          Support for a single or bulk upload. Strictly prohibited from
          uploading company data or other banned files.
        </p>
      </Dragger>

      <Table<FileType>
        className='table'
        columns={columns}
        dataSource={data}
      />
      <Spin spinning={spinning} fullscreen />
    </div>
  )
}

export default App
