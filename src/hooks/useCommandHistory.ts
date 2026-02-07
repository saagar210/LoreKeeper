import { useCallback, useRef, useState } from "react";

export function useCommandHistory() {
  const [history, setHistory] = useState<string[]>([]);
  const indexRef = useRef(-1);

  const addCommand = useCallback((cmd: string) => {
    if (cmd.trim()) {
      setHistory((prev) => [...prev, cmd]);
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
