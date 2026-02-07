interface Props {
  onNewGame: () => void;
  onLoadGame: () => void;
  onSettings: () => void;
  onStats?: () => void;
  onModules?: () => void;
  onAchievements?: () => void;
  onReplays?: () => void;
  onEditor?: () => void;
}

export function TitleScreen({ onNewGame, onLoadGame, onSettings, onStats, onModules, onAchievements, onReplays, onEditor }: Props) {
  return (
    <div className="flex h-full flex-col items-center justify-center bg-[var(--bg)] font-mono">
      <pre className="mb-8 text-[var(--accent)] text-xs leading-tight" aria-hidden="true">
{`
 _                   _  __                         
| |    ___  _ __ ___| |/ /___  ___ _ __   ___ _ __ 
| |   / _ \\| '__/ _ \\ ' // _ \\/ _ \\ '_ \\ / _ \\ '__|
| |__| (_) | | |  __/ . \\  __/  __/ |_) |  __/ |   
|_____\\___/|_|  \\___|_|\\_\\___|\\___| .__/ \\___|_|   
                                  |_|               
`}
      </pre>
      <h1 className="mb-12 text-[var(--text-dim)] text-base">The Depths of Thornhold</h1>
      <div className="flex flex-col gap-3">
        <button
          onClick={onNewGame}
          className="w-48 border border-[var(--accent)] px-6 py-2 text-[var(--accent)] transition-colors hover:bg-[var(--accent)] hover:text-[var(--bg)]"
        >
          New Game
        </button>
        <button
          onClick={onLoadGame}
          className="w-48 border border-[var(--text-dim)] px-6 py-2 text-[var(--text-dim)] transition-colors hover:bg-[var(--text-dim)] hover:text-[var(--bg)]"
        >
          Load Game
        </button>
        <button
          onClick={onSettings}
          className="w-48 border border-[var(--text-dim)] px-6 py-2 text-[var(--text-dim)] transition-colors hover:bg-[var(--text-dim)] hover:text-[var(--bg)]"
        >
          Settings
        </button>
        {onStats && (
          <button
            onClick={onStats}
            className="w-48 border border-[var(--text-dim)] px-6 py-2 text-[var(--text-dim)] transition-colors hover:bg-[var(--text-dim)] hover:text-[var(--bg)]"
          >
            Statistics
          </button>
        )}
        {onModules && (
          <button
            onClick={onModules}
            className="w-48 border border-[var(--text-dim)] px-6 py-2 text-[var(--text-dim)] transition-colors hover:bg-[var(--text-dim)] hover:text-[var(--bg)]"
          >
            Game Modules
          </button>
        )}
        {onAchievements && (
          <button
            onClick={onAchievements}
            className="w-48 border border-[var(--text-dim)] px-6 py-2 text-[var(--text-dim)] transition-colors hover:bg-[var(--text-dim)] hover:text-[var(--bg)]"
          >
            Achievements
          </button>
        )}
        {onReplays && (
          <button
            onClick={onReplays}
            className="w-48 border border-[var(--text-dim)] px-6 py-2 text-[var(--text-dim)] transition-colors hover:bg-[var(--text-dim)] hover:text-[var(--bg)]"
          >
            Replays
          </button>
        )}
        {onEditor && (
          <button
            onClick={onEditor}
            className="w-48 border border-[var(--text-dim)] px-6 py-2 text-[var(--text-dim)] transition-colors hover:bg-[var(--text-dim)] hover:text-[var(--bg)]"
          >
            Map Editor
          </button>
        )}
      </div>
    </div>
  );
}
