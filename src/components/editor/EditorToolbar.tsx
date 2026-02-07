import type { ValidationResult } from "../../store/types";

export type EditorTool = "select" | "addRoom" | "connect" | "delete";

interface Props {
  selectedTool: EditorTool;
  onToolChange: (tool: EditorTool) => void;
  onValidate: () => void;
  onExport: () => void;
  onClear: () => void;
  validation: ValidationResult | null;
  roomCount: number;
}

const tools: { id: EditorTool; label: string }[] = [
  { id: "select", label: "Select" },
  { id: "addRoom", label: "Add Room" },
  { id: "connect", label: "Connect" },
  { id: "delete", label: "Delete" },
];

export function EditorToolbar({
  selectedTool,
  onToolChange,
  onValidate,
  onExport,
  onClear,
  validation,
  roomCount,
}: Props) {
  return (
    <div className="flex flex-col gap-2 border-b border-[var(--border)] bg-[var(--panel-bg)] px-4 py-3">
      <div className="flex items-center gap-2 flex-wrap">
        {tools.map((tool) => (
          <button
            key={tool.id}
            onClick={() => onToolChange(tool.id)}
            className={`border px-3 py-1 text-xs transition-colors ${
              selectedTool === tool.id
                ? "border-[var(--accent)] bg-[var(--accent)] text-[var(--bg)]"
                : "border-[var(--border)] text-[var(--text-dim)] hover:border-[var(--accent)] hover:text-[var(--accent)]"
            }`}
          >
            {tool.label}
          </button>
        ))}

        <span className="mx-2 text-[var(--border)]">|</span>

        <button
          onClick={onValidate}
          disabled={roomCount === 0}
          className="border border-[var(--border)] px-3 py-1 text-xs text-[var(--text-dim)] transition-colors hover:border-[var(--accent)] hover:text-[var(--accent)] disabled:opacity-30"
        >
          Validate
        </button>
        <button
          onClick={onExport}
          disabled={roomCount === 0}
          className="border border-[var(--accent)] px-3 py-1 text-xs text-[var(--accent)] transition-colors hover:bg-[var(--accent)] hover:text-[var(--bg)] disabled:opacity-30"
        >
          Export
        </button>
        <button
          onClick={onClear}
          disabled={roomCount === 0}
          className="border border-[var(--border)] px-3 py-1 text-xs text-[var(--text-dim)] transition-colors hover:border-red-400 hover:text-red-400 disabled:opacity-30"
        >
          Clear
        </button>

        <span className="ml-auto text-xs text-[var(--text-dim)]">
          {roomCount} room{roomCount !== 1 ? "s" : ""}
        </span>
      </div>

      {validation && (
        <div className="flex flex-col gap-1 text-xs">
          {validation.valid && (
            <span className="text-green-400">Module is valid.</span>
          )}
          {validation.errors.map((err, i) => (
            <span key={`err-${i}`} className="text-red-400">
              Error: {err}
            </span>
          ))}
          {validation.warnings.map((warn, i) => (
            <span key={`warn-${i}`} className="text-yellow-400">
              Warning: {warn}
            </span>
          ))}
        </div>
      )}
    </div>
  );
}
