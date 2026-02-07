import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useRef, useState } from "react";
import { trapFocus } from "../../lib/focusTrap";
import type { GameStats } from "../../store/types";

interface Props {
  onClose: () => void;
}

const STAT_LABELS: Record<string, string> = {
  rooms_explored: "Rooms Explored",
  enemies_defeated: "Enemies Defeated",
  items_collected: "Items Collected",
  quests_completed: "Quests Completed",
  commands_entered: "Commands Entered",
  deaths: "Deaths",
  games_started: "Games Started",
  total_turns: "Total Turns",
};

export function StatsScreen({ onClose }: Props) {
  const [stats, setStats] = useState<GameStats | null>(null);
  const dialogRef = useRef<HTMLDivElement>(null);

  const fetchStats = useCallback(async () => {
    try {
      const result = await invoke<GameStats>("get_stats");
      setStats(result);
    } catch {
      setStats(null);
    }
  }, []);

  useEffect(() => {
    fetchStats();
  }, [fetchStats]);

  useEffect(() => {
    if (dialogRef.current) {
      return trapFocus(dialogRef.current);
    }
  }, []);

  const handleReset = async () => {
    try {
      await invoke("reset_stats");
      fetchStats();
    } catch (err) {
      console.error("Failed to reset stats:", err);
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 font-mono"
      role="dialog"
      aria-labelledby="stats-heading"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div ref={dialogRef} className="w-96 bg-[var(--panel-bg)] border border-[var(--border)] p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 id="stats-heading" className="text-lg font-bold text-[var(--accent)]">Statistics</h2>
          <button
            onClick={onClose}
            className="text-[var(--text-dim)] hover:text-[var(--text)]"
          >
            [X]
          </button>
        </div>

        {stats ? (
          <div className="space-y-2">
            {Object.entries(STAT_LABELS).map(([key, label]) => (
              <div key={key} className="flex justify-between text-xs">
                <span className="text-[var(--text)]">{label}</span>
                <span className="text-[var(--accent)] font-bold">
                  {stats[key] ?? 0}
                </span>
              </div>
            ))}
          </div>
        ) : (
          <p className="text-[var(--text-dim)] text-sm">Loading...</p>
        )}

        <div className="mt-4 flex justify-end">
          <button
            onClick={handleReset}
            className="border border-[var(--error)] px-3 py-1 text-xs text-[var(--error)] hover:bg-[var(--error)] hover:text-[var(--bg)]"
          >
            Reset Stats
          </button>
        </div>
      </div>
    </div>
  );
}
