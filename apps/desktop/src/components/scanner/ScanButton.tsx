interface ScanButtonProps {
  onClick: () => void;
  disabled?: boolean;
  loading?: boolean;
}

export default function ScanButton({
  onClick,
  disabled = false,
  loading = false,
}: ScanButtonProps) {
  return (
    <button onClick={onClick} disabled={disabled || loading}>
      {loading ? "Scanning..." : "Start First Scan"}
    </button>
  );
}