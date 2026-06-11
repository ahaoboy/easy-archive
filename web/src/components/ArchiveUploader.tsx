import { useCallback, useState } from "react";
import { Paper, Typography } from "@mui/material";
import { CloudUpload } from "@mui/icons-material";
import { SUPPORTED_EXTENSIONS } from "../utils";

interface ArchiveUploaderProps {
  onUpload: (file: File) => void;
}

export function ArchiveUploader({ onUpload }: ArchiveUploaderProps) {
  const [dragOver, setDragOver] = useState(false);

  const handleDrop = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      setDragOver(false);
      const file = e.dataTransfer.files[0];
      if (file) onUpload(file);
    },
    [onUpload],
  );

  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const file = e.target.files?.[0];
      if (file) onUpload(file);
    },
    [onUpload],
  );

  return (
    <Paper
      variant="outlined"
      onDragOver={(e) => { e.preventDefault(); setDragOver(true); }}
      onDragLeave={() => setDragOver(false)}
      onDrop={handleDrop}
      sx={{
        width: "100%",
        maxWidth: 600,
        p: 4,
        textAlign: "center",
        cursor: "pointer",
        borderStyle: "dashed",
        borderColor: dragOver ? "primary.main" : "divider",
        bgcolor: dragOver ? "action.hover" : "background.paper",
        transition: "border-color 0.2s, background-color 0.2s",
      }}
      component="label"
    >
      <input
        type="file"
        hidden
        onChange={handleChange}
        onClick={(e) => { (e.target as HTMLInputElement).value = ""; }}
      />
      <CloudUpload sx={{ fontSize: 48, color: "text.secondary", mb: 1 }} />
      <Typography variant="body1" color="text.primary">
        Click or drag archive file to this area to upload
      </Typography>
      <Typography variant="body2" color="text.secondary" sx={{ mt: 0.5 }}>
        Support format: {SUPPORTED_EXTENSIONS}
      </Typography>
    </Paper>
  );
}
