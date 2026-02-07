import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useRef, useState } from "react";
import { trapFocus } from "../../lib/focusTrap";
import { formatDate } from "../../lib/format";
import type { CommandLogEntry, ReplayDetail, ReplayInfo } from "../../store/types";

interface Props {
  onClose: () => void;
}

function formatEndingType(ending: string | null): string {
  if (!ending) return "Unknown";
  switch (ending) {
    case "VictoryPeace":
      return "Victory (Peace)";
    case "VictoryCombat":
      return "Victory (Combat)";
    case "Death":
      return "Death";
    default:
      return ending;
  }
}

function ReplayViewer({
  detail,
  onBack,
}: {
  detail: ReplayDetail;
  onBack: () => void;
}) {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [playing, setPlaying] = useState(false);
  const [speed, setSpeed] = useState(1000);
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const listRef = useRef<HTMLDivElement>(null);

  const commands = detail.commands;

  useEffect(() => {
    if (playing && currentIndex < commands.length - 1) {
      intervalRef.current = setInterval(() => {
        setCurrentIndex((prev) => {
          if (prev >= commands.length - 1) {
            setPlaying(false);
            return prev;
          }
          return prev + 1;
        });
      }, speed);
    }
    return () => {
      if (intervalRef.current) clearInterval(intervalRef.current);
    };
  }, [playing, speed, commands.length, currentIndex]);

  useEffect(() => {
    if (listRef.current) {
      const active = listRef.current.querySelector("[data-active='true']");
      if (active) {
        active.scrollIntoView({ block: "nearest", behavior: "smooth" });
      }
    }
  }, [currentIndex]);

  const togglePlay = () => {
    if (currentIndex >= commands.length - 1) {
      setCurrentIndex(0);
      setPlaying(true);
    } else {
      setPlaying((p) => !p);
    }
  };

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center justify-between">
        <button
          onClick={onBack}
          className="text-xs text-[var(--text-dim)] hover:text-[var(--text)]"
        >
          &lt; Back to list
        </button>
        <span className="text-xs text-[var(--text-dim)]">
          {formatEndingType(detail.info.endingType)} | {detail.info.turnsTaken ?? 0} turns | {detail.info.questsCompleted ?? 0} quests
        </span>
      </div>

      <div className="flex items-center gap-3">
        <button
          onClick={togglePlay}
          className="border border-[var(--accent)] px-3 py-1 text-xs text-[var(--accent)] hover:bg-[var(--accent)] hover:text-[var(--bg)]"
        >
          {playing ? "Pause" : currentIndex >= commands.length - 1 ? "Restart" : "Play"}
        </button>
        <button
          onClick={() => {
            setPlaying(false);
            setCurrentIndex((p) => Math.max(0, p - 1));
          }}
          disabled={currentIndex <= 0}
          className="border border-[var(--border)] px-2 py-1 text-xs text-[var(--text-dim)] hover:text-[var(--text)] disabled:opacity-30"
        >
          Prev
        </button>
        <button
          onClick={() => {
            setPlaying(false);
            setCurrentIndex((p) => Math.min(commands.length - 1, p + 1));
          }}
          disabled={currentIndex >= commands.length - 1}
          className="border border-[var(--border)] px-2 py-1 text-xs text-[var(--text-dim)] hover:text-[var(--text)] disabled:opacity-30"
        >
          Next
        </button>
        <select
          value={speed}
          onChange={(e) => setSpeed(Number(e.target.value))}
          className="border border-[var(--border)] bg-[var(--panel-bg)] px-2 py-1 text-xs text-[var(--text-dim)]"
        >
          <option value={2000}>0.5x</option>
          <option value={1000}>1x</option>
          <option value={500}>2x</option>
          <option value={250}>4x</option>
        </select>
        <span className="text-xs text-[var(--text-dim)]">
          {currentIndex + 1} / {commands.length}
        </span>
      </div>

      <div
        ref={listRef}
        className="max-h-[50vh] overflow-y-auto border border-[var(--border)] bg-[var(--bg)]"
      >
        {commands.map((cmd: CommandLogEntry, i: number) => (
          <div
            key={i}
            data-active={i === currentIndex}
            className={`flex items-baseline gap-3 px-3 py-1 text-xs font-mono ${
              i === currentIndex
                ? "bg-[var(--accent)]/10 text-[var(--accent)]"
                : i < currentIndex
                  ? "text-[var(--text-dim)]"
                  : "text-[var(--text-dim)] opacity-40"
            }`}
          >
            <span className="w-8 shrink-0 text-right opacity-50">
              {cmd.turn}
            </span>
            <span className="w-28 shrink-0 truncate opacity-60">
              {cmd.location}
            </span>
            <span className="font-bold">&gt; {cmd.input}</span>
          </div>
        ))}
      </div>
    </div>
  );
}

export function ReplayScreen({ onClose }: Props) {
  const [replays, setReplays] = useState<ReplayInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedReplay, setSelectedReplay] = useState<ReplayDetail | null>(null);
  const [error, setError] = useState<string | null>(null);
  const dialogRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    invoke<ReplayInfo[]>("list_replays")
      .then(setReplays)
      .catch((err) => setError(`Failed to load replays: ${err}`))
      .finally(() => setLoading(false));
  }, []);

  useEffect(() => {
    if (dialogRef.current) {
      return trapFocus(dialogRef.current);
    }
  }, []);

  const handleSelectReplay = useCallback(async (id: number) => {
    try {
      const detail = await invoke<ReplayDetail>("get_replay", { id });
      setSelectedReplay(detail);
    } catch (err) {
      setError(`Failed to load replay: ${err}`);
    }
  }, []);

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 font-mono"
      role="dialog"
      aria-modal="true"
      aria-labelledby="replays-heading"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div
        ref={dialogRef}
        className="w-[600px] max-h-[80vh] bg-[var(--panel-bg)] border border-[var(--border)] p-6 overflow-y-auto"
      >
        <div className="flex items-center justify-between mb-4">
          <h2
            id="replays-heading"
            className="text-lg font-bold text-[var(--accent)]"
          >
            Replays
          </h2>
          <button
            onClick={onClose}
            className="text-[var(--text-dim)] hover:text-[var(--text)]"
          >
            [X]
          </button>
        </div>

        {error && (
          <p className="text-xs text-red-400 mb-3">{error}</p>
        )}

        {loading ? (
          <p className="text-xs text-[var(--text-dim)]">Loading...</p>
        ) : selectedReplay ? (
          <ReplayViewer
            detail={selectedReplay}
            onBack={() => setSelectedReplay(null)}
          />
        ) : replays.length === 0 ? (
          <p className="text-xs text-[var(--text-dim)]">
            No completed playthroughs yet. Finish a game to see its replay here.
          </p>
        ) : (
          <div className="flex flex-col gap-2">
            {replays.map((replay) => (
              <button
                key={replay.id}
                onClick={() => handleSelectReplay(replay.id)}
                className="flex items-center justify-between border border-[var(--border)] px-4 py-3 text-left transition-colors hover:border-[var(--accent)] hover:bg-[var(--accent)]/5"
              >
                <div>
                  <span className="text-sm text-[var(--text)]">
                    {formatEndingType(replay.endingType)}
                  </span>
                  <span className="ml-3 text-xs text-[var(--text-dim)]">
                    {replay.turnsTaken ?? 0} turns | {replay.questsCompleted ?? 0} quests | {replay.commandCount} commands
                  </span>
                </div>
                <span className="text-xs text-[var(--text-dim)]">
                  {formatDate(replay.endedAt)}
                </span>
              </button>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
