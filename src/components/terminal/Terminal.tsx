import { useEffect } from "react";
import { useGame } from "../../hooks/useGame";
import { useSettings } from "../../hooks/useSettings";
import type { GameMode, WorldState } from "../../store/types";
import { TerminalInput } from "./TerminalInput";
import { TerminalOutput } from "./TerminalOutput";

function isGameOver(mode: GameMode): boolean {
  return typeof mode === "object" && "gameOver" in mode;
}

interface Props {
  onWorldStateChange?: (state: WorldState) => void;
}

export function Terminal({ onWorldStateChange }: Props) {
  const { history, worldState, isReady, isNarrating, initializeGame, sendCommand } = useGame();
  const { settings } = useSettings();

  useEffect(() => {
    initializeGame();
  }, [initializeGame]);

  useEffect(() => {
    if (worldState) onWorldStateChange?.(worldState);
  }, [worldState, onWorldStateChange]);

  const handleCommand = (input: string) => {
    sendCommand(input);
  };

  const gameOver = worldState ? isGameOver(worldState.gameMode) : false;

  return (
    <div className="flex h-full flex-col bg-[var(--bg)]">
      <TerminalOutput
        lines={history}
        isNarrating={isNarrating}
        ollamaEnabled={settings.ollamaEnabled}
        ollamaModel={settings.ollamaModel}
      />
      <TerminalInput
        onSubmit={handleCommand}
        disabled={!isReady || isNarrating || gameOver}
      />
    </div>
  );
}
