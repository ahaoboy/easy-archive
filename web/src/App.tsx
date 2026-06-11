import { Backdrop, Box, CircularProgress } from "@mui/material";
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
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        gap: 2,
        height: "100vh",
        overflow: "hidden",
        p: 2,
      }}
    >
      <ArchiveUploader onUpload={handleUpload} />
      <Box sx={{ flex: 1, minHeight: 0, width: "100%", overflow: "hidden" }}>
        <FileTable data={data} onRemove={removeEntry} />
      </Box>
      <Backdrop open={spinning} sx={{ zIndex: (t) => t.zIndex.drawer + 1 }}>
        <CircularProgress />
      </Backdrop>
      <DownloadButton
        filename={filename}
        outputFmt={outputFmt}
        onFmtChange={setOutputFmt}
        onDownload={downloadArchive}
      />
    </Box>
  );
};

export default App;
