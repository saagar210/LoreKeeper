import { invoke } from "@tauri-apps/api/core";
import { useCallback } from "react";

export function useNarrationControls() {
  const rate = useCallback(
    async (promptHash: string, rating: number, model: string) => {
      try {
        await invoke("rate_narration", { promptHash, rating, model });
      } catch {
        // Rating is non-critical — silently ignore failures
      }
    },
    [],
  );

  const retry = useCallback(async () => {
    try {
      await invoke("retry_narration");
    } catch (err) {
      console.error("Retry narration failed:", err);
    }
  }, []);

  return { rate, retry };
}
