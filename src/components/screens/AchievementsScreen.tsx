import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";
import { trapFocus } from "../../lib/focusTrap";
import { formatDate } from "../../lib/format";
import type { AchievementInfo } from "../../store/types";

const iconMap: Record<string, string> = {
  sword: "\u2694",
  compass: "\u{1F9ED}",
  scroll: "\u{1F4DC}",
  clock: "\u23F0",
  bag: "\u{1F392}",
  book: "\u{1F4D6}",
  heart: "\u2764",
  handshake: "\u{1F91D}",
};

interface Props {
  onClose: () => void;
}

export function AchievementsScreen({ onClose }: Props) {
  const [achievements, setAchievements] = useState<AchievementInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const dialogRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    invoke<AchievementInfo[]>("get_achievements")
      .then(setAchievements)
      .catch((err) => console.error("Failed to load achievements:", err))
      .finally(() => setLoading(false));
  }, []);

  useEffect(() => {
    if (dialogRef.current) {
      return trapFocus(dialogRef.current);
    }
  }, []);

  const unlocked = achievements.filter((a) => a.unlocked).length;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 font-mono"
      role="dialog"
      aria-modal="true"
      aria-labelledby="achievements-heading"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div ref={dialogRef} className="w-[520px] max-h-[80vh] bg-[var(--panel-bg)] border border-[var(--border)] p-6 overflow-y-auto">
        <div className="flex items-center justify-between mb-4">
          <h2 id="achievements-heading" className="text-lg font-bold text-[var(--accent)]">
            Achievements ({unlocked}/{achievements.length})
          </h2>
          <button
            onClick={onClose}
            className="text-[var(--text-dim)] hover:text-[var(--text)]"
          >
            [X]
          </button>
        </div>

        {loading ? (
          <p className="text-xs text-[var(--text-dim)]">Loading...</p>
        ) : (
          <div className="grid grid-cols-2 gap-3">
            {achievements.map((ach) => (
              <div
                key={ach.id}
                className={`border p-3 ${
                  ach.unlocked
                    ? "border-[var(--accent)] bg-[var(--accent)]/5"
                    : "border-[var(--border)] opacity-50"
                }`}
              >
                <div className="flex items-center gap-2 mb-1">
                  <span className="text-lg">{iconMap[ach.icon] ?? "?"}</span>
                  <span className={`text-sm font-bold ${ach.unlocked ? "text-[var(--accent)]" : "text-[var(--text-dim)]"}`}>
                    {ach.name}
                  </span>
                </div>
                <p className="text-xs text-[var(--text-dim)]">{ach.description}</p>
                {ach.unlocked && ach.unlockedAt && (
                  <p className="text-xs text-[var(--text-dim)] mt-1">
                    Unlocked: {formatDate(ach.unlockedAt)}
                  </p>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
