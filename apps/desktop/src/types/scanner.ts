export const ScanStatus = {
  Idle: "Idle",
  Scanning: "Scanning",
  Completed: "Completed",
  Cancelled: "Cancelled",
  Failed: "Failed",
} as const;

export type ScanStatus =
  (typeof ScanStatus)[keyof typeof ScanStatus];

export interface ScanSummary {
  files: number;
  folders: number;
  elapsedTime: number;
  status: ScanStatus;
}