import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useRef, useState } from "react";
import { MAX_HISTORY_LINES } from "../lib/constants";
import type {
  CommandResponse,
  NarrativeEvent,
  OutputLine,
  WorldState,
} from "../store/types";

function trimHistory(lines: OutputLine[]): OutputLine[] {
  if (lines.length > MAX_HISTORY_LINES) {
    return lines.slice(lines.length - MAX_HISTORY_LINES);
  }
  return lines;
}

export function useGame() {
  const [history, setHistory] = useState<OutputLine[]>([]);
  const [worldState, setWorldState] = useState<WorldState | null>(null);
  const [isReady, setIsReady] = useState(false);
  const [isNarrating, setIsNarrating] = useState(false);
  const unlistenRef = useRef<(() => void) | null>(null);
  // Tracks whether we've started a new LLM narration line in the current stream
  const streamLineStartedRef = useRef(false);

  // Listen for narrative streaming events
  useEffect(() => {
    let cancelled = false;
    listen<NarrativeEvent>("narrative-event", (event) => {
      if (cancelled) return;
      const payload = event.payload;
      if (payload.type === "token") {
        setIsNarrating(true);
        setHistory((prev) => {
          if (!streamLineStartedRef.current) {
            // First token of a new stream — always create a new line
            streamLineStartedRef.current = true;
            return [
              ...prev,
              { text: payload.text, lineType: "narration" as const },
            ];
          }
          // Subsequent tokens — append to the streaming line
          const updated = [...prev];
          const last = updated[updated.length - 1];
          if (last && last.lineType === "narration") {
            updated[updated.length - 1] = {
              ...last,
              text: last.text + payload.text,
            };
            return updated;
          }
          // Safety fallback: start a new line
          return [
            ...prev,
            { text: payload.text, lineType: "narration" as const },
          ];
        });
      } else if (payload.type === "complete" || payload.type === "fallback") {
        streamLineStartedRef.current = false;
        setIsNarrating(false);
      }
    }).then((unlisten) => {
      if (!cancelled) {
        unlistenRef.current = unlisten;
      } else {
        unlisten();
      }
    }).catch((err) => {
      console.error("Failed to listen for narrative events:", err);
    });

    return () => {
      cancelled = true;
      unlistenRef.current?.();
    };
  }, []);

  const initializeGame = useCallback(async () => {
    try {
      const response = await invoke<CommandResponse>("initialize_game");
      setHistory(response.messages);
      setWorldState(response.worldState);
      setIsReady(true);
    } catch (err) {
      console.error("Failed to initialize game:", err);
      setHistory([
        { text: `Error: ${err}`, lineType: "error" },
      ]);
    }
  }, []);

  const sendCommand = useCallback(
    async (input: string) => {
      if (!input.trim()) return;

      // Echo player input
      setHistory((prev) => [
        ...prev,
        { text: `> ${input}`, lineType: "playerInput" },
      ]);

      try {
        const response = await invoke<CommandResponse>("process_command", {
          input,
        });
        setWorldState(response.worldState);
        if (response.messages.length > 0) {
          setHistory((prev) => trimHistory([...prev, ...response.messages]));
        }
      } catch (err) {
        setHistory((prev) => [
          ...prev,
          { text: `Error: ${err}`, lineType: "error" },
        ]);
      }
    },
    [],
  );

  const newGame = useCallback(async () => {
    try {
      const response = await invoke<CommandResponse>("new_game");
      setHistory(response.messages);
      setWorldState(response.worldState);
    } catch (err) {
      console.error("Failed to start new game:", err);
    }
  }, []);

  return {
    history,
    worldState,
    isReady,
    isNarrating,
    initializeGame,
    sendCommand,
    newGame,
  };
}
