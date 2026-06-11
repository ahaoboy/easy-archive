import { Upload } from "antd";
import { InboxOutlined } from "@ant-design/icons";
import { SUPPORTED_EXTENSIONS } from "../utils";

const { Dragger } = Upload;

interface ArchiveUploaderProps {
  onUpload: (file: File) => void;
}

export function ArchiveUploader({ onUpload }: ArchiveUploaderProps) {
  return (
    <Dragger
      name="file"
      action="#"
      customRequest={(e) => e.onSuccess?.(true)}
      showUploadList={false}
      onChange={(info) => {
        const file = info.file.originFileObj;
        if (file) onUpload(file);
      }}
    >
      <p className="ant-upload-drag-icon">
        <InboxOutlined />
      </p>
      <p className="ant-upload-text">
        Click or drag archive file to this area to upload
      </p>
      <p className="ant-upload-hint">
        Support format: {SUPPORTED_EXTENSIONS}
      </p>
    </Dragger>
  );
}
