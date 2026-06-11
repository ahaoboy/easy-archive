import { Button, Select, Space, Typography } from "antd";
import { DownloadOutlined } from "@ant-design/icons";
import type { Fmt } from "@easy-install/easy-archive";
import { SUPPORTED_WRITE_FORMATS, stripArchiveExt } from "../utils";

const { Text } = Typography;

interface DownloadButtonProps {
  filename: string;
  outputFmt: Fmt;
  onFmtChange: (fmt: Fmt) => void;
  onDownload: () => void;
}

export function DownloadButton({
  filename,
  outputFmt,
  onFmtChange,
  onDownload,
}: DownloadButtonProps) {
  const displayName = filename ? stripArchiveExt(filename) : "archive";

  return (
    <Space.Compact size="large">
      <Button
        disabled={!filename}
        style={{
          maxWidth: 320,
          overflow: "hidden",
          textOverflow: "ellipsis",
          whiteSpace: "nowrap",
        }}
        title={displayName}
      >
        <Text ellipsis style={{ maxWidth: 280 }}>
          {displayName}
        </Text>
      </Button>
      <Select
        value={outputFmt}
        onChange={onFmtChange}
        popupMatchSelectWidth={false}
        options={SUPPORTED_WRITE_FORMATS.map((f) => ({
          value: f.fmt,
          label: f.ext,
        }))}
      />
      <Button
        type="primary"
        icon={<DownloadOutlined />}
        onClick={onDownload}
      >
      </Button>
    </Space.Compact>
  );
}
