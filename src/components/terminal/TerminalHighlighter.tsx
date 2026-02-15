import React, { useMemo } from "react";

interface HighlightedToken {
  text: string;
  type: "command" | "target" | "preposition" | "text" | "direction";
}

const COMMAND_KEYWORDS = [
  "look",
  "go",
  "take",
  "drop",
  "use",
  "equip",
  "unequip",
  "attack",
  "flee",
  "talk",
  "inventory",
  "inv",
  "i",
  "map",
  "help",
  "journal",
  "craft",
  "save",
  "load",
];

const PREPOSITIONS = ["at", "to", "with", "on", "from", "in", "the", "a", "an"];
const DIRECTIONS = ["north", "south", "east", "west", "up", "down", "n", "s", "e", "w"];

export function highlightCommand(input: string): HighlightedToken[] {
  if (!input.trim()) return [];

  const tokens = input.split(/\s+/);
  return tokens.map((token, i) => {
    const lowerToken = token.toLowerCase();

    if (i === 0 && COMMAND_KEYWORDS.includes(lowerToken)) {
      return { text: token, type: "command" as const };
    }

    if (DIRECTIONS.includes(lowerToken)) {
      return { text: token, type: "direction" as const };
    }

    if (PREPOSITIONS.includes(lowerToken)) {
      return { text: token, type: "preposition" as const };
    }

    if (i > 0) {
      return { text: token, type: "target" as const };
    }

    return { text: token, type: "text" as const };
  });
}

interface HighlightedTerminalInputProps {
  value: string;
  onChange: (value: string) => void;
  onSubmit: () => void;
  placeholder?: string;
}

export function HighlightedTerminalInput({
  value,
  onChange,
  onSubmit,
  placeholder,
}: HighlightedTerminalInputProps) {
  const tokens = useMemo(() => highlightCommand(value), [value]);

  const getTokenClass = (type: HighlightedToken["type"]) => {
    switch (type) {
      case "command":
        return "text-green-400 font-bold";
      case "direction":
        return "text-yellow-300 font-semibold";
      case "preposition":
        return "text-cyan-300";
      case "target":
        return "text-purple-300";
      default:
        return "text-gray-300";
    }
  };

  return (
    <div className="relative w-full">
      {/* Highlighted display layer (non-interactive) */}
      <div
        className="absolute inset-0 flex items-center gap-1 pointer-events-none px-3 py-2
                   bg-transparent text-sm md:text-base font-mono whitespace-nowrap overflow-hidden"
      >
        <span className="text-gray-600">$</span>
        <div className="flex gap-1">
          {tokens.map((token, i) => (
            <span key={i} className={getTokenClass(token.type)}>
              {token.text}
            </span>
          ))}
        </div>
      </div>

      {/* Interactive input layer (transparent) */}
      <input
        type="text"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        onKeyDown={(e) => {
          if (e.key === "Enter") {
            onSubmit();
          }
        }}
        placeholder={placeholder || "Enter command..."}
        aria-label="Game command input"
        className="relative w-full bg-transparent text-transparent caret-white outline-none
                   px-3 py-2 text-sm md:text-base font-mono
                   focus:ring-1 focus:ring-green-500"
        autoCapitalize="off"
        autoCorrect="off"
        spellCheck="false"
        autoFocus
      />
    </div>
  );
}
