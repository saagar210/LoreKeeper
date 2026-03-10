import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useRef, useState } from "react";
import { trapFocus } from "../../lib/focusTrap";
import {
  applyCustomTheme,
  applyTheme,
  createDefaultCustomThemeConfig,
  sanitizeCustomThemeConfig,
  themeVarNames,
} from "../../lib/themes";
import { normalizeThemeNameInput } from "../../lib/inputValidation";
import { TAURI_COMMANDS } from "../../lib/tauriCommands";
import type { CustomThemeInfo, ThemeConfig } from "../../store/types";

interface Props {
  onClose: () => void;
}

const varLabels: Record<string, string> = {
  "--bg": "Background",
  "--text": "Text",
  "--text-dim": "Text Dim",
  "--text-bright": "Text Bright",
  "--accent": "Accent",
  "--error": "Error",
  "--combat": "Combat",
  "--dialogue": "Dialogue",
  "--input": "Input",
  "--system": "System",
  "--border": "Border",
  "--panel-bg": "Panel BG",
  "--hp-high": "HP High",
  "--hp-mid": "HP Mid",
  "--hp-low": "HP Low",
};

export function ThemeCreator({ onClose }: Props) {
  const [config, setConfig] = useState<ThemeConfig>(
    createDefaultCustomThemeConfig,
  );
  const [themeName, setThemeName] = useState("");
  const [savedThemes, setSavedThemes] = useState<CustomThemeInfo[]>([]);
  const [message, setMessage] = useState<string | null>(null);
  const dialogRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (dialogRef.current) {
      return trapFocus(dialogRef.current);
    }
  }, []);

  const fetchThemes = useCallback(async () => {
    try {
      const result = await invoke<CustomThemeInfo[]>(
        TAURI_COMMANDS.listCustomThemes,
      );
      setSavedThemes(result);
    } catch {
      // ignore
    }
  }, []);

  useEffect(() => {
    fetchThemes();
  }, [fetchThemes]);

  // Live preview
  useEffect(() => {
    if (!applyCustomTheme(config)) {
      setMessage("Theme preview is invalid. Reset to continue.");
    }
    return () => {
      // Revert on unmount
      applyTheme("greenTerminal");
    };
  }, [config]);

  const handleColorChange = (varName: string, value: string) => {
    setConfig((prev) => ({ ...prev, [varName]: value }));
  };

  const handleSave = async () => {
    const nameResult = normalizeThemeNameInput(themeName);
    if (!nameResult.ok) {
      setMessage(nameResult.message);
      return;
    }

    const sanitizedConfig = sanitizeCustomThemeConfig(config);
    if (!sanitizedConfig) {
      setMessage("Theme colors must use six-digit hex values.");
      return;
    }
    try {
      await invoke(TAURI_COMMANDS.saveCustomTheme, {
        name: nameResult.value,
        config: JSON.stringify(sanitizedConfig),
      });
      setThemeName(nameResult.value);
      setMessage(`Saved '${nameResult.value}'.`);
      await fetchThemes();
    } catch (err) {
      setMessage(`Save failed: ${err}`);
    }
  };

  const handleLoadSaved = (info: CustomThemeInfo) => {
    try {
      const parsed = sanitizeCustomThemeConfig(JSON.parse(info.config));
      if (!parsed) {
        setMessage(`'${info.name}' is invalid and was skipped.`);
        return;
      }
      setConfig(parsed);
      setThemeName(info.name);
    } catch {
      setMessage("Failed to parse theme config.");
    }
  };

  const handleDelete = async (name: string) => {
    try {
      await invoke(TAURI_COMMANDS.deleteCustomTheme, { name });
      await fetchThemes();
      setMessage(`Deleted '${name}'.`);
    } catch (err) {
      setMessage(`Delete failed: ${err}`);
    }
  };

  const handleReset = () => {
    setConfig(createDefaultCustomThemeConfig());
    setThemeName("");
    setMessage(null);
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 font-mono"
      role="dialog"
      aria-modal="true"
      aria-labelledby="theme-creator-heading"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div
        ref={dialogRef}
        className="max-h-[80vh] w-[560px] overflow-y-auto bg-[var(--panel-bg)] border border-[var(--border)] p-6"
      >
        <div className="flex items-center justify-between mb-4">
          <h2
            id="theme-creator-heading"
            className="text-lg font-bold text-[var(--accent)]"
          >
            Theme Creator
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

        <div className="grid grid-cols-2 gap-2 mb-4">
          {themeVarNames.map((varName) => (
            <label key={varName} className="flex items-center gap-2 text-xs">
              <input
                type="color"
                value={config[varName] ?? "#000000"}
                onChange={(e) => handleColorChange(varName, e.target.value)}
                className="h-6 w-8 cursor-pointer border border-[var(--border)] bg-transparent"
              />
              <span className="text-[var(--text)]">
                {varLabels[varName] ?? varName}
              </span>
            </label>
          ))}
        </div>

        {/* Preview */}
        <div className="mb-4 border border-[var(--border)] p-3 text-xs">
          <p className="text-[var(--text)]">Normal text sample.</p>
          <p className="text-[var(--accent)]">Accent text.</p>
          <p className="text-[var(--error)]">Error text.</p>
          <p className="text-[var(--combat)] font-bold">Combat text.</p>
          <p className="text-[var(--dialogue)]">Dialogue text.</p>
          <p className="text-[var(--system)] italic">System text.</p>
        </div>

        {/* Save / Load */}
        <div className="flex gap-2 mb-4">
          <input
            type="text"
            value={themeName}
            onChange={(e) => setThemeName(e.target.value)}
            placeholder="Theme name"
            className="flex-1 border border-[var(--border)] bg-transparent px-2 py-1 text-xs text-[var(--text)] outline-none"
          />
          <button
            onClick={handleSave}
            className="border border-[var(--accent)] px-3 py-1 text-xs text-[var(--accent)] hover:bg-[var(--accent)] hover:text-[var(--bg)]"
          >
            Save
          </button>
          <button
            onClick={handleReset}
            className="border border-[var(--border)] px-3 py-1 text-xs text-[var(--text-dim)] hover:text-[var(--text)]"
          >
            Reset
          </button>
        </div>

        {savedThemes.length > 0 && (
          <div>
            <h3 className="mb-2 text-xs font-bold uppercase tracking-wider text-[var(--muted)]">
              Saved Themes
            </h3>
            <div className="space-y-1">
              {savedThemes.map((t) => (
                <div
                  key={t.name}
                  className="flex items-center justify-between text-xs"
                >
                  <button
                    onClick={() => handleLoadSaved(t)}
                    className="text-[var(--text)] hover:text-[var(--accent)]"
                  >
                    {t.name}
                  </button>
                  <button
                    onClick={() => handleDelete(t.name)}
                    className="text-[var(--text-dim)] hover:text-[var(--error)]"
                  >
                    [del]
                  </button>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
