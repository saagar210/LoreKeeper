import type { EndingType, Player } from "../../store/types";

interface Props {
  endingType: EndingType;
  player: Player;
  onPlayAgain: () => void;
}

export function EndingScreen({ endingType, player, onPlayAgain }: Props) {
  const title =
    endingType === "victoryPeace"
      ? "Victory Through Wisdom"
      : "Victory Through Strength";
  const description =
    endingType === "victoryPeace"
      ? "Through wisdom and compassion, you have freed The Forgotten One. Thornhold's curse is broken, and peace returns to these ancient halls."
      : "By blade and determination, you have defeated The Forgotten One. Thornhold's curse is broken by force, though at great cost.";

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 font-mono">
      <div className="flex max-w-lg flex-col items-center gap-6 p-8 text-center">
        <h1 className="text-3xl font-bold text-[var(--accent)]">{title}</h1>
        <p className="text-[var(--text)]">{description}</p>
        <div className="space-y-1 text-sm text-[var(--text-dim)]">
          <p>Turns taken: {player.turnsElapsed}</p>
          <p>Rooms explored: {player.visitedLocations.length}</p>
        </div>
        <button
          onClick={onPlayAgain}
          className="border border-[var(--accent)] px-6 py-2 text-[var(--accent)] transition-colors hover:bg-[var(--accent)] hover:text-[var(--bg)]"
        >
          Play Again
        </button>
      </div>
    </div>
  );
}
