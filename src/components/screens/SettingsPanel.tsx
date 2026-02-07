import { useEffect, useRef } from "react";
import { trapFocus } from "../../lib/focusTrap";
import { useSettings } from "../../hooks/useSettings";
import type { Difficulty, ThemeName } from "../../store/types";

interface Props {
  onClose: () => void;
  onThemeChange: (theme: ThemeName) => void;
  onOpenThemeCreator?: () => void;
}

const themeOptions: { value: ThemeName; label: string }[] = [
  { value: "greenTerminal", label: "Green Terminal" },
  { value: "amberTerminal", label: "Amber Terminal" },
  { value: "parchment", label: "Parchment" },
  { value: "darkModern", label: "Dark Modern" },
];

export function SettingsPanel({ onClose, onThemeChange, onOpenThemeCreator }: Props) {
  const { settings, updateSettings, ollamaStatus, checkOllama, models, getModels } =
    useSettings();
  const dialogRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    checkOllama();
  }, [checkOllama]);

  useEffect(() => {
    if (dialogRef.current) {
      return trapFocus(dialogRef.current);
    }
  }, []);

  useEffect(() => {
    if (ollamaStatus.connected) {
      getModels();
    }
  }, [ollamaStatus.connected, getModels]);

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 font-mono"
      role="dialog"
      aria-modal="true"
      aria-labelledby="settings-heading"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div ref={dialogRef} className="w-[480px] bg-[var(--panel-bg)] border border-[var(--border)] p-6">
        <div className="flex items-center justify-between mb-6">
          <h2 id="settings-heading" className="text-lg font-bold text-[var(--accent)]">Settings</h2>
          <button
            onClick={onClose}
            className="text-[var(--text-dim)] hover:text-[var(--text)]"
          >
            [X]
          </button>
        </div>

        <div className="space-y-6">
          {/* Theme */}
          <div>
            <label className="text-sm text-[var(--text)] font-bold block mb-2">Theme</label>
            <div className="flex gap-2">
              {themeOptions.map((t) => (
                <button
                  key={t.value}
                  onClick={() => {
                    updateSettings({ theme: t.value });
                    onThemeChange(t.value);
                  }}
                  className={`border px-3 py-1 text-xs transition-colors ${
                    settings.theme === t.value
                      ? "border-[var(--accent)] text-[var(--accent)]"
                      : "border-[var(--border)] text-[var(--text-dim)] hover:text-[var(--text)]"
                  }`}
                >
                  {t.label}
                </button>
              ))}
            </div>
            {onOpenThemeCreator && (
              <button
                onClick={onOpenThemeCreator}
                className="mt-2 border border-[var(--border)] px-3 py-1 text-xs text-[var(--text-dim)] hover:text-[var(--text)]"
              >
                Create Custom Theme
              </button>
            )}
          </div>

          {/* Typewriter */}
          <div>
            <label className="text-sm text-[var(--text)] font-bold block mb-2">
              Typewriter Speed: {settings.typewriterSpeed}ms
            </label>
            <input
              type="range"
              min={0}
              max={100}
              value={settings.typewriterSpeed}
              onChange={(e) =>
                updateSettings({ typewriterSpeed: Number(e.target.value) })
              }
              className="w-full"
            />
            <div className="flex justify-between text-xs text-[var(--text-dim)]">
              <span>Instant</span>
              <span>Slow</span>
            </div>
          </div>

          {/* Sound */}
          <div>
            <label className="flex items-center gap-2 text-sm text-[var(--text)] font-bold mb-2">
              <input
                type="checkbox"
                checked={settings.soundEnabled}
                onChange={(e) =>
                  updateSettings({ soundEnabled: e.target.checked })
                }
              />
              Sound Effects
            </label>
            {settings.soundEnabled && (
              <div>
                <label className="text-xs text-[var(--text-dim)] block mb-1">
                  Volume: {Math.round(settings.soundVolume * 100)}%
                </label>
                <input
                  type="range"
                  min={0}
                  max={1}
                  step={0.05}
                  value={settings.soundVolume}
                  onChange={(e) =>
                    updateSettings({ soundVolume: Number(e.target.value) })
                  }
                  className="w-full"
                />
              </div>
            )}
          </div>

          {/* Difficulty */}
          <div>
            <label className="text-sm text-[var(--text)] font-bold block mb-2">Difficulty</label>
            <div className="flex gap-2">
              {(["easy", "normal", "hard"] as Difficulty[]).map((d) => (
                <button
                  key={d}
                  onClick={() => updateSettings({ difficulty: d })}
                  className={`border px-3 py-1 text-xs capitalize transition-colors ${
                    settings.difficulty === d
                      ? "border-[var(--accent)] text-[var(--accent)]"
                      : "border-[var(--border)] text-[var(--text-dim)] hover:text-[var(--text)]"
                  }`}
                >
                  {d}
                </button>
              ))}
            </div>
            <p className="text-xs text-[var(--text-dim)] mt-1">
              {settings.difficulty === "easy" && "More damage dealt, less taken, more hints."}
              {settings.difficulty === "normal" && "Balanced experience."}
              {settings.difficulty === "hard" && "Less damage dealt, more taken, fewer hints."}
            </p>
          </div>

          {/* Ollama */}
          <div className="border-t border-[var(--border)] pt-4">
            <div className="flex items-center justify-between mb-2">
              <label className="text-sm text-[var(--text)] font-bold">Ollama AI Narration</label>
              <span
                className={`text-xs ${ollamaStatus.connected ? "text-[var(--hp-high)]" : "text-[var(--error)]"}`}
              >
                {ollamaStatus.connected
                  ? `Connected (v${ollamaStatus.version})`
                  : "Disconnected"}
              </span>
            </div>
            <div className="space-y-3">
              <label className="flex items-center gap-2 text-xs text-[var(--text)]">
                <input
                  type="checkbox"
                  checked={settings.ollamaEnabled}
                  onChange={(e) =>
                    updateSettings({ ollamaEnabled: e.target.checked })
                  }
                />
                Enable LLM Narration
              </label>
              {settings.ollamaEnabled && (
                <>
                  <div>
                    <label className="text-xs text-[var(--text-dim)] block mb-1">Model</label>
                    <select
                      value={settings.ollamaModel}
                      onChange={(e) =>
                        updateSettings({ ollamaModel: e.target.value })
                      }
                      className="w-full bg-[var(--bg)] border border-[var(--border)] px-2 py-1 text-xs text-[var(--text)]"
                    >
                      {models.length > 0 ? (
                        models.map((m) => (
                          <option key={m.name} value={m.name}>
                            {m.name}
                          </option>
                        ))
                      ) : (
                        <option value={settings.ollamaModel}>
                          {settings.ollamaModel}
                        </option>
                      )}
                    </select>
                  </div>
                  <div>
                    <label className="text-xs text-[var(--text-dim)] block mb-1">
                      Temperature: {settings.temperature.toFixed(1)}
                    </label>
                    <input
                      type="range"
                      min={0}
                      max={2}
                      step={0.1}
                      value={settings.temperature}
                      onChange={(e) =>
                        updateSettings({ temperature: Number(e.target.value) })
                      }
                      className="w-full"
                    />
                  </div>
                  <div>
                    <label className="text-xs text-[var(--text-dim)] block mb-1">Narration Verbosity</label>
                    <select
                      value={settings.narrationVerbosity}
                      onChange={(e) =>
                        updateSettings({ narrationVerbosity: e.target.value })
                      }
                      className="w-full bg-[var(--bg)] border border-[var(--border)] px-2 py-1 text-xs text-[var(--text)]"
                    >
                      <option value="brief">Brief (1 sentence)</option>
                      <option value="normal">Normal (2-3 sentences)</option>
                      <option value="verbose">Verbose (4-6 sentences)</option>
                    </select>
                  </div>
                  <div>
                    <label className="text-xs text-[var(--text-dim)] block mb-1">Ollama URL</label>
                    <input
                      type="text"
                      value={settings.ollamaUrl}
                      onChange={(e) =>
                        updateSettings({ ollamaUrl: e.target.value })
                      }
                      className="w-full bg-transparent border border-[var(--border)] px-2 py-1 text-xs text-[var(--text)] outline-none"
                    />
                  </div>
                </>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
