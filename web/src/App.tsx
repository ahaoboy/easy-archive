import { Flex, Spin } from "antd";
import { ArchiveUploader } from "./components/ArchiveUploader";
import { DownloadButton } from "./components/DownloadButton";
import { FileTable } from "./components/FileTable";
import { useArchive } from "./hooks/useArchive";

const App: React.FC = () => {
  const {
    data,
    spinning,
    filename,
    outputFmt,
    setOutputFmt,
    handleUpload,
    removeEntry,
    downloadArchive,
  } = useArchive();

  return (
    <Flex
      className="main"
      vertical
      gap="middle"
      justify="space-around"
      align="center"
    >
      <ArchiveUploader onUpload={handleUpload} />
      <FileTable data={data} onRemove={removeEntry} />
      <Spin spinning={spinning} fullscreen />
      <DownloadButton
        filename={filename}
        outputFmt={outputFmt}
        onFmtChange={setOutputFmt}
        onDownload={downloadArchive}
      />
    </Flex>
  );
};

export default App;
