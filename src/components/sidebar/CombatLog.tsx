import { useState } from "react";
import type { CombatLogEntry } from "../../store/types";

interface Props {
  combatLog: CombatLogEntry[];
}

export function CombatLog({ combatLog }: Props) {
  const [expanded, setExpanded] = useState(false);

  if (combatLog.length === 0) return null;

  const displayed = expanded ? combatLog : combatLog.slice(-5);

  return (
    <div role="region" aria-label="Combat log">
      <button
        className="mb-2 flex w-full items-center justify-between text-left text-xs font-bold uppercase tracking-wider text-[var(--muted)]"
        onClick={() => setExpanded(!expanded)}
        aria-expanded={expanded}
      >
        <span>Combat Log ({combatLog.length})</span>
        <span>{expanded ? "\u25B2" : "\u25BC"}</span>
      </button>
      <ul className="space-y-1 text-xs">
        {displayed.map((entry, i) => (
          <li
            key={`${entry.turn}-${entry.attacker}-${i}`}
            className={
              entry.isPlayerAttack
                ? "text-[var(--accent)]"
                : "text-[var(--error)]"
            }
          >
            <span className="text-[var(--muted)]">[T{entry.turn}]</span>{" "}
            {entry.attacker} hit {entry.defender} for {entry.damage} dmg{" "}
            <span className="text-[var(--muted)]">
              ({entry.defenderHpAfter} HP)
            </span>
          </li>
        ))}
      </ul>
      {!expanded && combatLog.length > 5 && (
        <button
          className="mt-1 text-xs text-[var(--muted)] underline"
          onClick={() => setExpanded(true)}
        >
          Show all {combatLog.length} entries
        </button>
      )}
    </div>
  );
}
