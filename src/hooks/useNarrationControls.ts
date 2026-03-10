import { invoke } from "@tauri-apps/api/core";
import { useCallback } from "react";
import { TAURI_COMMANDS } from "../lib/tauriCommands";

export function useNarrationControls() {
  const rate = useCallback(
    async (promptHash: string, rating: number, model: string) => {
      try {
        await invoke(TAURI_COMMANDS.rateNarration, { promptHash, rating, model });
      } catch {
        // Rating is non-critical — silently ignore failures
      }
    },
    [],
  );

  const retry = useCallback(async () => {
    try {
      await invoke(TAURI_COMMANDS.retryNarration);
    } catch (err) {
      console.error("Retry narration failed:", err);
    }
  }, []);

  return { rate, retry };
}
