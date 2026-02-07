import { useCallback, useRef, useState } from "react";

const MAX_COMMAND_HISTORY = 100;

export function useCommandHistory() {
  const [history, setHistory] = useState<string[]>([]);
  const indexRef = useRef(-1);

  const addCommand = useCallback((cmd: string) => {
    if (cmd.trim()) {
      setHistory((prev) => {
        const next = [...prev, cmd];
        return next.length > MAX_COMMAND_HISTORY
          ? next.slice(next.length - MAX_COMMAND_HISTORY)
          : next;
      });
      indexRef.current = -1;
    }
  }, []);

  const getPrevious = useCallback((): string | null => {
    if (history.length === 0) return null;
    if (indexRef.current === -1) {
      indexRef.current = history.length - 1;
    } else if (indexRef.current > 0) {
      indexRef.current -= 1;
    }
    return history[indexRef.current] ?? null;
  }, [history]);

  const getNext = useCallback((): string | null => {
    if (history.length === 0 || indexRef.current === -1) return null;
    if (indexRef.current < history.length - 1) {
      indexRef.current += 1;
      return history[indexRef.current] ?? null;
    }
    indexRef.current = -1;
    return "";
  }, [history]);

  return { addCommand, getPrevious, getNext };
}
