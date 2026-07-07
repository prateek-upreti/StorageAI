import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import type { ScanProgress } from "../types/progress";

export function useScanProgress() {
  const [progress, setProgress] = useState<ScanProgress | null>(null);

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    listen<ScanProgress>("scan-progress", (event) => {
      setProgress(event.payload);
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten?.();
    };
  }, []);

  return progress;
}