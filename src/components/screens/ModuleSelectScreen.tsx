import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useRef, useState } from "react";
import { trapFocus } from "../../lib/focusTrap";
import type { CommandResponse, ModuleInfo } from "../../store/types";

interface Props {
  onModuleLoaded: (response: CommandResponse) => void;
  onClose: () => void;
}

export function ModuleSelectScreen({ onModuleLoaded, onClose }: Props) {
  const [modules, setModules] = useState<ModuleInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const dialogRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (dialogRef.current) {
      return trapFocus(dialogRef.current);
    }
  }, []);

  const fetchModules = useCallback(async () => {
    setError(null);
    try {
      const result = await invoke<ModuleInfo[]>("list_modules");
      setModules(result);
    } catch (err) {
      setError(`Failed to list modules: ${err}`);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchModules();
  }, [fetchModules]);

  const [loadingModule, setLoadingModule] = useState(false);

  const handleLoad = async (path: string) => {
    if (loadingModule) return;
    setError(null);
    setLoadingModule(true);
    try {
      const response = await invoke<CommandResponse>("load_module", { path });
      onModuleLoaded(response);
    } catch (err) {
      setError(`Failed to load module: ${err}`);
    } finally {
      setLoadingModule(false);
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 font-mono"
      role="dialog"
      aria-modal="true"
      aria-labelledby="modules-heading"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div
        ref={dialogRef}
        className="w-[520px] bg-[var(--panel-bg)] border border-[var(--border)] p-6"
      >
        <div className="flex items-center justify-between mb-6">
          <h2
            id="modules-heading"
            className="text-lg font-bold text-[var(--accent)]"
          >
            Game Modules
          </h2>
          <button
            onClick={onClose}
            className="text-[var(--text-dim)] hover:text-[var(--text)]"
          >
            [X]
          </button>
        </div>

        {loading && (
          <p className="text-xs text-[var(--muted)]">Loading modules...</p>
        )}
        {error && <p className="text-xs text-[var(--error)]">{error}</p>}

        {!loading && modules.length === 0 && (
          <div className="text-xs text-[var(--muted)]">
            <p className="mb-2">No modules found.</p>
            <p>
              Place <code>.json</code> module files in your app data{" "}
              <code>modules/</code> directory.
            </p>
          </div>
        )}

        {modules.length > 0 && (
          <div className="space-y-3 max-h-80 overflow-y-auto">
            {modules.map((mod_) => (
              <div
                key={mod_.path}
                className="flex items-center justify-between border border-[var(--border)] p-3"
              >
                <div>
                  <p className="text-sm font-bold text-[var(--text)]">
                    {mod_.name}
                  </p>
                  <p className="text-xs text-[var(--muted)]">
                    {mod_.description}
                  </p>
                </div>
                <button
                  onClick={() => handleLoad(mod_.path)}
                  disabled={loadingModule}
                  className="border border-[var(--border)] px-3 py-1 text-xs text-[var(--accent)] hover:border-[var(--accent)] disabled:opacity-30"
                >
                  {loadingModule ? "Loading..." : "Load"}
                </button>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
