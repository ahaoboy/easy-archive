import { useCallback, useState } from "react";
import {
  encode,
  File as WasmFile,
  Fmt,
  guess,
} from "@easy-install/easy-archive";
import type { FileType } from "../types";
import { downloadBinaryFile, filesToData, replaceExt, SUPPORTED_READ_FORMATS, SUPPORTED_WRITE_FORMATS } from "../utils";

export function useArchive() {
  const [data, setData] = useState<FileType[]>([]);
  const [spinning, setSpinning] = useState(false);
  const [filename, setFilename] = useState("");
  const [outputFmt, setOutputFmt] = useState<Fmt>(Fmt.Zip);

  /** Handle file upload — decode archive and populate the table. */
  const handleUpload = useCallback(async (file: File) => {
    setData([]);
    setSpinning(true);
    const fmt = guess(file.name);
    if (!fmt || !SUPPORTED_READ_FORMATS.includes(fmt)) {
      setSpinning(false);
      return;
    }
    const entries = await filesToData(fmt, file);
    if (entries?.length) {
      setData(entries);
    }
    setSpinning(false);
    setFilename(file.name);
  }, []);

  /** Remove a single entry from the table. */
  const removeEntry = useCallback((path: string) => {
    setData((prev) => prev.filter((f) => f.path !== path));
  }, []);

  /** Encode current entries with the selected format and trigger download. */
  const downloadArchive = useCallback(() => {
    if (data.length === 0) return;
    const archive = encode(
      outputFmt,
      data.map(
        (i) => new WasmFile(i.path, i.buffer, i.mode, i.isDir, i.lastModified),
      ),
    );
    if (!archive) return;

    const target = SUPPORTED_WRITE_FORMATS.find((f) => f.fmt === outputFmt)!;
    const outName = replaceExt(filename || "archive", target.ext);
    downloadBinaryFile(outName, archive);
  }, [data, outputFmt, filename]);

  return {
    data,
    spinning,
    filename,
    outputFmt,
    setOutputFmt,
    handleUpload,
    removeEntry,
    downloadArchive,
  };
}
