import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import { Button } from "./components/common/Button";
import ScanButton from "./components/scanner/ScanButton";

import { APP_NAME, APP_TAGLINE } from "./constants";
import { useScanProgress } from "./hooks/useScanProgress";

import "./App.css";

interface ScanResult {
  files: number;
  folders: number;
  totalSize: number;
  elapsedMs: number;
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) {
    return `${bytes} B`;
  }

  const units = ["KB", "MB", "GB", "TB"];

  let size = bytes / 1024;
  let unit = 0;

  while (size >= 1024 && unit < units.length - 1) {
    size /= 1024;
    unit++;
  }

  return `${size.toFixed(2)} ${units[unit]}`;
}

function formatTime(ms: number): string {
  if (ms < 1000) {
    return `${ms} ms`;
  }

  return `${(ms / 1000).toFixed(2)} sec`;
}

function App() {
  const [version, setVersion] = useState("Loading...");
  const [selectedFolder, setSelectedFolder] = useState("");
  const [scanResult, setScanResult] = useState<ScanResult | null>(null);

  const progress = useScanProgress();

  useEffect(() => {
    async function loadVersion() {
      try {
        const result = await invoke<string>("app_version");
        setVersion(result);
      } catch (error) {
        console.error(error);
        setVersion("Unable to connect to backend");
      }
    }

    loadVersion();
  }, []);

  async function handleStartScan() {
    try {
      const folder = await invoke<string | null>("select_folder");

      if (!folder) {
        return;
      }

      setSelectedFolder(folder);
      setScanResult(null);

      const [files, folders, totalSize, elapsedMs] =
        await invoke<[number, number, number, number]>("start_scan", {
          path: folder,
        });

      setScanResult({
        files,
        folders,
        totalSize,
        elapsedMs,
      });
    } catch (error) {
      console.error("Scan failed:", error);
    }
  }

  return (
    <main className="app">
      <h1>{APP_NAME}</h1>

      <h2>{APP_TAGLINE}</h2>

      <p>
        Scan drives, detect duplicate files, and organize your storage safely.
      </p>

      <ScanButton onClick={handleStartScan} />

      {selectedFolder && (
        <>
          <h3>Selected Folder</h3>

          <p>{selectedFolder}</p>
        </>
      )}

      {progress && (
        <>
          <h3>Live Progress</h3>

          <p>
            <strong>Files:</strong>{" "}
            {progress.files.toLocaleString()}
          </p>

          <p>
            <strong>Folders:</strong>{" "}
            {progress.folders.toLocaleString()}
          </p>

          <p>
            <strong>Current Folder:</strong>
          </p>

          <p>{progress.currentPath}</p>
        </>
      )}

      {scanResult && (
        <>
          <h3>Scan Summary</h3>

          <p>
            <strong>Files:</strong>{" "}
            {scanResult.files.toLocaleString()}
          </p>

          <p>
            <strong>Folders:</strong>{" "}
            {scanResult.folders.toLocaleString()}
          </p>

          <p>
            <strong>Total Size:</strong>{" "}
            {formatBytes(scanResult.totalSize)}
          </p>

          <p>
            <strong>Scan Time:</strong>{" "}
            {formatTime(scanResult.elapsedMs)}
          </p>
        </>
      )}

      <Button>Settings</Button>

      <small>{version}</small>
    </main>
  );
}

export default App;