import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useRef, useState } from "react";
import { trapFocus } from "../../lib/focusTrap";
import { formatRelativeTime } from "../../lib/format";
import type { SaveSlotInfo } from "../../store/types";

interface Props {
  mode: "save" | "load";
  onSave?: (slot: string) => void;
  onLoad?: (slot: string) => void;
  onClose: () => void;
}

export function SaveLoadScreen({ mode, onSave, onLoad, onClose }: Props) {
  const [saves, setSaves] = useState<SaveSlotInfo[]>([]);
  const [newSlotName, setNewSlotName] = useState("");
  const dialogRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (dialogRef.current) {
      return trapFocus(dialogRef.current);
    }
  }, []);

  const refreshSaves = useCallback(async () => {
    try {
      const result = await invoke<SaveSlotInfo[]>("list_saves");
      setSaves(result);
    } catch {
      setSaves([]);
    }
  }, []);

  useEffect(() => {
    refreshSaves();
  }, [refreshSaves]);

  const handleSave = (slot: string) => {
    onSave?.(slot);
    onClose();
  };

  const handleLoad = (slot: string) => {
    onLoad?.(slot);
    onClose();
  };

  const handleDelete = async (slot: string) => {
    try {
      await invoke("delete_save", { slotName: slot });
      refreshSaves();
    } catch (err) {
      console.error("Delete failed:", err);
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 font-mono"
      role="dialog"
      aria-labelledby="saveload-heading"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div ref={dialogRef} className="w-96 bg-[var(--panel-bg)] border border-[var(--border)] p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 id="saveload-heading" className="text-lg font-bold text-[var(--accent)]">
            {mode === "save" ? "Save Game" : "Load Game"}
          </h2>
          <button
            onClick={onClose}
            className="text-[var(--text-dim)] hover:text-[var(--text)]"
          >
            [X]
          </button>
        </div>

        {mode === "save" && (
          <div className="flex gap-2 mb-4">
            <input
              type="text"
              value={newSlotName}
              onChange={(e) => setNewSlotName(e.target.value)}
              placeholder="Save name..."
              className="flex-1 bg-transparent border border-[var(--border)] px-2 py-1 text-sm text-[var(--text)] outline-none"
              onKeyDown={(e) => {
                if (e.key === "Enter" && newSlotName.trim()) {
                  handleSave(newSlotName.trim());
                }
              }}
            />
            <button
              onClick={() => newSlotName.trim() && handleSave(newSlotName.trim())}
              className="border border-[var(--accent)] px-3 py-1 text-sm text-[var(--accent)] hover:bg-[var(--accent)] hover:text-[var(--bg)]"
            >
              Save
            </button>
          </div>
        )}

        <div className="max-h-64 overflow-y-auto space-y-2">
          {saves.length === 0 ? (
            <p className="text-[var(--text-dim)] text-sm">No saves found.</p>
          ) : (
            saves.map((save) => (
              <div
                key={save.slotName}
                className="flex items-center justify-between border border-[var(--border)] p-2 text-xs"
              >
                <div className="min-w-0 flex-1">
                  <div className="flex items-center justify-between">
                    <span className="text-[var(--text)] font-bold">{save.slotName}</span>
                    <span className="text-[var(--text-dim)] ml-2 shrink-0">{formatRelativeTime(save.savedAt)}</span>
                  </div>
                  <div className="text-[var(--text-dim)]">
                    {save.playerLocation} | HP: {save.playerHealth} | Turns: {save.turnsElapsed} | Quests: {save.questsCompleted ?? 0}
                  </div>
                </div>
                <div className="flex gap-2">
                  <button
                    onClick={() =>
                      mode === "save"
                        ? handleSave(save.slotName)
                        : handleLoad(save.slotName)
                    }
                    className="text-[var(--accent)] hover:underline"
                  >
                    {mode === "save" ? "Overwrite" : "Load"}
                  </button>
                  <button
                    onClick={() => handleDelete(save.slotName)}
                    className="text-[var(--error)] hover:underline"
                  >
                    Delete
                  </button>
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}
