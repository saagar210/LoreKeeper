import type { Player } from "../../store/types";

interface Props {
  player: Player;
  onLoadSave: () => void;
  onNewGame: () => void;
}

export function DeathScreen({ player, onLoadSave, onNewGame }: Props) {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 font-mono">
      <div className="flex flex-col items-center gap-6 p-8 text-center">
        <h1 className="text-3xl font-bold text-[var(--error)]">You Have Perished</h1>
        <p className="text-[var(--text-dim)]">
          Darkness claims another soul in the depths of Thornhold...
        </p>
        <div className="space-y-1 text-sm text-[var(--text-dim)]">
          <p>Turns survived: {player.turnsElapsed}</p>
          <p>Rooms explored: {player.visitedLocations.length}</p>
        </div>
        <div className="flex gap-4">
          <button
            onClick={onLoadSave}
            className="border border-[var(--accent)] px-6 py-2 text-[var(--accent)] transition-colors hover:bg-[var(--accent)] hover:text-[var(--bg)]"
          >
            Load Save
          </button>
          <button
            onClick={onNewGame}
            className="border border-[var(--text-dim)] px-6 py-2 text-[var(--text-dim)] transition-colors hover:bg-[var(--text-dim)] hover:text-[var(--bg)]"
          >
            New Game
          </button>
        </div>
      </div>
    </div>
  );
}
