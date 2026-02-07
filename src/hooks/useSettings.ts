import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useState } from "react";
import type { GameSettings, ModelInfo, OllamaStatus } from "../store/types";

const DEFAULT_SETTINGS: GameSettings = {
  ollamaEnabled: false,
  ollamaModel: "llama3.2",
  ollamaUrl: "http://localhost:11434",
  temperature: 0.7,
  narratorTone: "atmospheric",
  typewriterSpeed: 30,
  theme: "greenTerminal",
  narrationVerbosity: "normal",
};

export function useSettings() {
  const [settings, setSettings] = useState<GameSettings>(DEFAULT_SETTINGS);
  const [ollamaStatus, setOllamaStatus] = useState<OllamaStatus>({
    connected: false,
    version: null,
  });
  const [models, setModels] = useState<ModelInfo[]>([]);

  useEffect(() => {
    invoke<GameSettings>("get_settings")
      .then(setSettings)
      .catch((err) => console.warn("Failed to load settings:", err));
  }, []);

  const updateSettings = useCallback(
    async (partial: Partial<GameSettings>) => {
      const updated = { ...settings, ...partial };
      try {
        await invoke("update_settings", { settings: updated });
        setSettings(updated);
      } catch (err) {
        console.error("Failed to update settings:", err);
      }
    },
    [settings],
  );

  const checkOllama = useCallback(async () => {
    try {
      const status = await invoke<OllamaStatus>("get_ollama_status");
      setOllamaStatus(status);
      return status;
    } catch {
      setOllamaStatus({ connected: false, version: null });
      return { connected: false, version: null };
    }
  }, []);

  const getModels = useCallback(async () => {
    try {
      const result = await invoke<ModelInfo[]>("get_available_models");
      setModels(result);
      return result;
    } catch {
      setModels([]);
      return [];
    }
  }, []);

  return { settings, updateSettings, ollamaStatus, checkOllama, models, getModels };
}
