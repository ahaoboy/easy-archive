import { useEffect, useState } from "react";
import {
  Box,
  IconButton,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TablePagination,
  TableRow,
  Paper,
  Tooltip,
} from "@mui/material";
import { Delete, Download } from "@mui/icons-material";
import { modeToString } from "@easy-install/easy-archive";
import type { FileType } from "../types";
import { downloadBinaryFile } from "../utils";

interface FileTableProps {
  data: FileType[];
  onRemove: (path: string) => void;
}

const ROWS_PER_PAGE = 10;

/** Fixed-width spacer so dir rows keep the delete button aligned with file rows. */
const ActionSpacer = () => <Box sx={{ display: "inline-block", width: 34, height: 34 }} />;

export function FileTable({ data, onRemove }: FileTableProps) {
  const [page, setPage] = useState(0);

  const paged = data.slice(page * ROWS_PER_PAGE, (page + 1) * ROWS_PER_PAGE);

  // Reset page when data shrinks below current page
  useEffect(() => {
    if (paged.length === 0 && page > 0) setPage(0);
  }, [paged.length, page]);

  return (
    <Paper
      variant="outlined"
      sx={{
        display: "flex",
        flexDirection: "column",
        height: "100%",
        overflow: "hidden",
      }}
    >
      <TableContainer>
        <Table stickyHeader size="small">
          <TableHead>
            <TableRow>
              <TableCell>Path</TableCell>
              <TableCell align="center" sx={{ width: 50 }}>Dir</TableCell>
              <TableCell align="center" sx={{ width: 80 }}>Size</TableCell>
              <TableCell sx={{ width: 200 }}>Mode</TableCell>
              <TableCell sx={{ width: 170 }}>Modified</TableCell>
              <TableCell align="center" sx={{ width: 96 }}>Actions</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {paged.map((row) => (
              <TableRow key={row.key} hover>
                <TableCell sx={{ wordBreak: "break-all" }}>{row.path}</TableCell>
                <TableCell align="center">{row.isDir ? "✓" : ""}</TableCell>
                <TableCell align="center">{row.isDir ? "" : row.size}</TableCell>
                <TableCell sx={{ whiteSpace: "nowrap", fontFamily: "monospace", fontSize: "0.85rem" }}>
                  {row.mode !== undefined
                    ? `(0o${row.mode.toString(8).padStart(3, "0")}) ${modeToString(row.mode, row.isDir)}`
                    : ""}
                </TableCell>
                <TableCell sx={{ whiteSpace: "nowrap" }}>
                  {row.lastModified
                    ? new Date(Number(row.lastModified) * 1000).toLocaleString()
                    : ""}
                </TableCell>
                <TableCell align="center" sx={{ whiteSpace: "nowrap" }}>
                  {row.isDir ? (
                    <ActionSpacer />
                  ) : (
                    <Tooltip title="Download">
                      <IconButton size="small" onClick={() => downloadBinaryFile(row.path, row.buffer)}>
                        <Download fontSize="small" />
                      </IconButton>
                    </Tooltip>
                  )}
                  <Tooltip title="Remove">
                    <IconButton size="small" color="error" onClick={() => onRemove(row.path)}>
                      <Delete fontSize="small" />
                    </IconButton>
                  </Tooltip>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
      <TablePagination
        component="div"
        count={data.length}
        page={page}
        onPageChange={(_, p) => setPage(p)}
        rowsPerPage={ROWS_PER_PAGE}
        rowsPerPageOptions={[ROWS_PER_PAGE]}
      />
    </Paper>
  );
}
