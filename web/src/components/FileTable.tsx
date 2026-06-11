import { Button, type TableProps, Table } from "antd";
import { DeleteOutlined, DownloadOutlined } from "@ant-design/icons";
import { modeToString } from "@easy-install/easy-archive";
import type { FileType } from "../types";
import { downloadBinaryFile } from "../utils";

interface FileTableProps {
  data: FileType[];
  onRemove: (path: string) => void;
}

export function FileTable({ data, onRemove }: FileTableProps) {
  const columns: TableProps<FileType>["columns"] = [
    {
      title: "Path",
      dataIndex: "path",
      key: "path",
    },
    {
      title: "Dir",
      dataIndex: "isDir",
      key: "isDir",
      width: 60,
      render: (_, { isDir }) => (isDir ? "✓" : ""),
    },
    {
      title: "Size",
      dataIndex: "size",
      key: "size",
      width: 90,
      render: (_, { isDir, size }) => (!isDir ? size : ""),
    },
    {
      title: "Mode",
      dataIndex: "mode",
      key: "mode",
      width: 200,
      render: (_, { isDir, mode }) =>
        mode !== undefined
          ? `(0o${mode.toString(8).padStart(3, "0")}) ${modeToString(mode, isDir)}`
          : "",
    },
    {
      title: "Modified",
      dataIndex: "lastModified",
      key: "lastModified",
      width: 180,
      render: (_, { lastModified }) =>
        lastModified
          ? new Date(Number(lastModified) * 1000).toLocaleString()
          : "",
    },
    {
      title: "Actions",
      key: "actions",
      width: 100,
      render: (_, { path, buffer, isDir }) => (
        <>
          {!isDir && (
            <Button
              type="text"
              size="small"
              icon={<DownloadOutlined />}
              onClick={() => downloadBinaryFile(path, buffer)}
            />
          )}
          <Button
            type="text"
            size="small"
            danger
            icon={<DeleteOutlined />}
            onClick={() => onRemove(path)}
          />
        </>
      ),
    },
  ];

  return (
    <Table<FileType>
      className="table"
      columns={columns}
      dataSource={data}
      rowKey="key"
      pagination={false}
      size="small"
      scroll={{ y: "calc(100vh - 360px)" }}
    />
  );
}
