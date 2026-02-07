import { invoke } from "@tauri-apps/api/core";
import { useCallback, useRef, useState } from "react";

export function useAutocomplete() {
  const [completions, setCompletions] = useState<string[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(-1);
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const fetchCompletions = useCallback((prefix: string) => {
    if (debounceRef.current) clearTimeout(debounceRef.current);

    if (!prefix.trim()) {
      setCompletions([]);
      setSelectedIndex(-1);
      return;
    }

    debounceRef.current = setTimeout(async () => {
      try {
        const results = await invoke<string[]>("get_completions", { prefix });
        setCompletions(results);
        setSelectedIndex(results.length > 0 ? 0 : -1);
      } catch {
        setCompletions([]);
        setSelectedIndex(-1);
      }
    }, 150);
  }, []);

  const selectNext = useCallback(() => {
    setSelectedIndex((prev) => {
      if (completions.length === 0) return -1;
      return (prev + 1) % completions.length;
    });
  }, [completions.length]);

  const selectPrev = useCallback(() => {
    setSelectedIndex((prev) => {
      if (completions.length === 0) return -1;
      return prev <= 0 ? completions.length - 1 : prev - 1;
    });
  }, [completions.length]);

  const accept = useCallback((): string | null => {
    if (selectedIndex >= 0 && selectedIndex < completions.length) {
      const accepted = completions[selectedIndex];
      setCompletions([]);
      setSelectedIndex(-1);
      return accepted;
    }
    return null;
  }, [selectedIndex, completions]);

  const dismiss = useCallback(() => {
    setCompletions([]);
    setSelectedIndex(-1);
  }, []);

  return {
    completions,
    selectedIndex,
    fetchCompletions,
    selectNext,
    selectPrev,
    accept,
    dismiss,
  };
}
