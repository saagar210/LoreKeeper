import { useEffect, useRef, useState } from "react";
import type { OutputLine as OutputLineType } from "../../store/types";
import { useNarrationControls } from "../../hooks/useNarrationControls";
import { OutputLine } from "./OutputLine";

interface KeyedLine {
  id: number;
  line: OutputLineType;
}

interface Props {
  lines: OutputLineType[];
  isNarrating?: boolean;
  ollamaEnabled?: boolean;
  ollamaModel?: string;
}

export function TerminalOutput({ lines, isNarrating, ollamaEnabled, ollamaModel }: Props) {
  const bottomRef = useRef<HTMLDivElement>(null);
  const nextIdRef = useRef(0);
  const [keyedLines, setKeyedLines] = useState<KeyedLine[]>([]);
  const prevLengthRef = useRef(0);
  const [rated, setRated] = useState(false);
  const { rate, retry } = useNarrationControls();

  // Reset rated state when new narration lines arrive
  useEffect(() => {
    if (isNarrating) setRated(false);
  }, [isNarrating]);

  // Assign stable IDs to new lines when the array grows or resets
  useEffect(() => {
    if (lines.length === 0) {
      nextIdRef.current = 0;
      setKeyedLines([]);
      prevLengthRef.current = 0;
      return;
    }

    if (lines.length < prevLengthRef.current) {
      // History was reset (new game / load) — re-key everything
      const newKeyed = lines.map((line) => ({
        id: nextIdRef.current++,
        line,
      }));
      setKeyedLines(newKeyed);
    } else if (lines.length > prevLengthRef.current) {
      // Append new lines with new IDs, keep existing keys
      const newLines = lines.slice(prevLengthRef.current);
      setKeyedLines((prev) => [
        ...prev,
        ...newLines.map((line) => ({
          id: nextIdRef.current++,
          line,
        })),
      ]);
    } else {
      // Same length — last line was mutated (streaming token append)
      setKeyedLines((prev) => {
        if (prev.length === 0) return prev;
        const updated = [...prev];
        updated[updated.length - 1] = {
          ...updated[updated.length - 1],
          line: lines[lines.length - 1],
        };
        return updated;
      });
    }
    prevLengthRef.current = lines.length;
  }, [lines]);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [keyedLines]);

  const hasNarration = keyedLines.length > 0 &&
    keyedLines.some((kl) => kl.line.lineType === "narration");

  return (
    <div
      className="flex-1 overflow-y-auto p-4 font-mono text-sm"
      role="log"
      aria-live="polite"
      aria-label="Game output"
    >
      {keyedLines.map(({ id, line }) => (
        <OutputLine key={id} line={line} />
      ))}
      {ollamaEnabled && !isNarrating && hasNarration && (
        <div className="mt-1 flex items-center gap-2 text-xs text-[var(--muted)]">
          {!rated ? (
            <>
              <button
                onClick={() => {
                  const hash = `narration-${nextIdRef.current}`;
                  rate(hash, 1, ollamaModel ?? "");
                  setRated(true);
                }}
                className="hover:text-[var(--accent)]"
                title="Good narration"
              >
                [+]
              </button>
              <button
                onClick={() => {
                  const hash = `narration-${nextIdRef.current}`;
                  rate(hash, -1, ollamaModel ?? "");
                  setRated(true);
                }}
                className="hover:text-[var(--error)]"
                title="Poor narration"
              >
                [-]
              </button>
            </>
          ) : (
            <span>Rated</span>
          )}
          <button
            onClick={retry}
            className="hover:text-[var(--accent)]"
            title="Retry narration"
          >
            [retry]
          </button>
        </div>
      )}
      <div ref={bottomRef} />
    </div>
  );
}
