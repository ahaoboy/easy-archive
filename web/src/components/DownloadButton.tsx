import { Button, ButtonGroup, MenuItem, TextField, Typography } from "@mui/material";
import { Download } from "@mui/icons-material";
import type { Fmt } from "@easy-install/easy-archive";
import { SUPPORTED_WRITE_FORMATS, stripArchiveExt } from "../utils";

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
    <ButtonGroup variant="outlined" size="large" sx={{ mt: 2 }}>
      <Button disabled={!filename} sx={{ textTransform: "none", px: 2 }} component="span">
        <Typography
          noWrap
          sx={{ maxWidth: 260, fontSize: "inherit", fontWeight: 400 }}
        >
          {displayName}
        </Typography>
      </Button>
      <TextField
        select
        value={outputFmt}
        onChange={(e) => onFmtChange(Number(e.target.value) as Fmt)}
        size="small"
        sx={{ minWidth: 90, "& .MuiOutlinedInput-notchedOutline": { borderRadius: 0 } }}
      >
        {SUPPORTED_WRITE_FORMATS.map((f) => (
          <MenuItem key={f.fmt} value={f.fmt}>
            {f.ext}
          </MenuItem>
        ))}
      </TextField>
      <Button
        variant="contained"
        startIcon={<Download />}
        onClick={onDownload}
        sx={{ textTransform: "none" }}
      >
        Download
      </Button>
    </ButtonGroup>
  );
}
