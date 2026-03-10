import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useRef, useState } from "react";
import { trapFocus } from "../../lib/focusTrap";
import { formatRelativeTime } from "../../lib/format";
import { normalizeSaveSlotNameInput } from "../../lib/inputValidation";
import { TAURI_COMMANDS } from "../../lib/tauriCommands";
import type { SaveSlotInfo } from "../../store/types";

export type SaveLoadActionResult =
  | { ok: true; slotName: string }
  | { ok: false; message: string };

interface Props {
  mode: "save" | "load";
  onSave?: (slot: string) => Promise<SaveLoadActionResult>;
  onLoad?: (slot: string) => Promise<SaveLoadActionResult>;
  onClose: () => void;
}

export function SaveLoadScreen({ mode, onSave, onLoad, onClose }: Props) {
  const [saves, setSaves] = useState<SaveSlotInfo[]>([]);
  const [newSlotName, setNewSlotName] = useState("");
  const [confirmDelete, setConfirmDelete] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const dialogRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (dialogRef.current) {
      return trapFocus(dialogRef.current);
    }
  }, []);

  const refreshSaves = useCallback(async () => {
    try {
      const result = await invoke<SaveSlotInfo[]>(TAURI_COMMANDS.listSaves);
      setSaves(result);
    } catch {
      setSaves([]);
    }
  }, []);

  useEffect(() => {
    refreshSaves();
  }, [refreshSaves]);

  const handleSave = async (slotInput: string) => {
    const result = normalizeSaveSlotNameInput(slotInput);
    if (!result.ok) {
      setMessage(result.message);
      return;
    }

    const outcome = await onSave?.(result.value);
    if (!outcome) {
      onClose();
      return;
    }
    if (outcome.ok) {
      setMessage(null);
      onClose();
      return;
    }

    setMessage(outcome.message);
  };

  const handleLoad = async (slotInput: string) => {
    const result = normalizeSaveSlotNameInput(slotInput);
    if (!result.ok) {
      setMessage(result.message);
      return;
    }

    const outcome = await onLoad?.(result.value);
    if (!outcome) {
      onClose();
      return;
    }
    if (outcome.ok) {
      setMessage(null);
      onClose();
      return;
    }

    setMessage(outcome.message);
  };

  const handleDelete = async (slot: string) => {
    if (confirmDelete !== slot) {
      setConfirmDelete(slot);
      return;
    }
    setConfirmDelete(null);
    try {
      await invoke(TAURI_COMMANDS.deleteSave, { slotName: slot });
      setMessage(`Deleted '${slot}'.`);
      await refreshSaves();
    } catch (err) {
      setMessage(
        err instanceof Error ? err.message : `Delete failed: ${String(err)}`,
      );
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 font-mono"
      role="dialog"
      aria-modal="true"
      aria-labelledby="saveload-heading"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div
        ref={dialogRef}
        className="w-96 bg-[var(--panel-bg)] border border-[var(--border)] p-6"
      >
        <div className="flex items-center justify-between mb-4">
          <h2
            id="saveload-heading"
            className="text-lg font-bold text-[var(--accent)]"
          >
            {mode === "save" ? "Save Game" : "Load Game"}
          </h2>
          <button
            onClick={onClose}
            className="text-[var(--text-dim)] hover:text-[var(--text)]"
          >
            [X]
          </button>
        </div>

        {message && (
          <p className="mb-3 text-xs text-[var(--system)]">{message}</p>
        )}

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
                  void handleSave(newSlotName);
                }
              }}
            />
            <button
              onClick={() => newSlotName.trim() && void handleSave(newSlotName)}
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
                    <span className="text-[var(--text)] font-bold">
                      {save.slotName}
                    </span>
                    <span className="text-[var(--text-dim)] ml-2 shrink-0">
                      {formatRelativeTime(save.savedAt)}
                    </span>
                  </div>
                  <div className="text-[var(--text-dim)]">
                    {save.playerLocation} | HP: {save.playerHealth} | Turns:{" "}
                    {save.turnsElapsed} | Quests: {save.questsCompleted ?? 0}
                  </div>
                </div>
                <div className="flex gap-2">
                  <button
                    onClick={() =>
                      mode === "save"
                        ? void handleSave(save.slotName)
                        : void handleLoad(save.slotName)
                    }
                    className="text-[var(--accent)] hover:underline"
                  >
                    {mode === "save" ? "Overwrite" : "Load"}
                  </button>
                  <button
                    onClick={() => handleDelete(save.slotName)}
                    className="text-[var(--error)] hover:underline"
                  >
                    {confirmDelete === save.slotName ? "Confirm?" : "Delete"}
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
