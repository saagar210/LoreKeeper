import { render, screen, fireEvent } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { CombatLog } from "./CombatLog";
import type { CombatLogEntry } from "../../store/types";

function makeEntry(overrides: Partial<CombatLogEntry> = {}): CombatLogEntry {
  return {
    turn: 1,
    attacker: "Player",
    defender: "Goblin",
    damage: 5,
    defenderHpAfter: 10,
    isPlayerAttack: true,
    ...overrides,
  };
}

describe("CombatLog", () => {
  it("renders nothing when log is empty", () => {
    const { container } = render(<CombatLog combatLog={[]} />);
    expect(container.firstChild).toBeNull();
  });

  it("renders combat log entries", () => {
    const entries = [
      makeEntry(),
      makeEntry({ attacker: "Goblin", defender: "Player", isPlayerAttack: false, damage: 3, defenderHpAfter: 47 }),
    ];
    render(<CombatLog combatLog={entries} />);
    expect(screen.getByText(/Player hit Goblin/)).toBeInTheDocument();
    expect(screen.getByText(/Goblin hit Player/)).toBeInTheDocument();
  });

  it("shows only last 5 entries by default when more than 5 exist", () => {
    const entries = Array.from({ length: 8 }, (_, i) =>
      makeEntry({ turn: i, damage: i + 1 })
    );
    render(<CombatLog combatLog={entries} />);
    // Should show "Show all" link
    expect(screen.getByText("Show all 8 entries")).toBeInTheDocument();
    // Should show 5 entries
    const items = screen.getAllByRole("listitem");
    expect(items).toHaveLength(5);
  });

  it("expands to show all entries on toggle", () => {
    const entries = Array.from({ length: 8 }, (_, i) =>
      makeEntry({ turn: i, damage: i + 1 })
    );
    render(<CombatLog combatLog={entries} />);
    fireEvent.click(screen.getByRole("button", { expanded: false }));
    const items = screen.getAllByRole("listitem");
    expect(items).toHaveLength(8);
  });

  it("shows count in heading", () => {
    const entries = [makeEntry()];
    render(<CombatLog combatLog={entries} />);
    expect(screen.getByText("Combat Log (1)")).toBeInTheDocument();
  });
});
