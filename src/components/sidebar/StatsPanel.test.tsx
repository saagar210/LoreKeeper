import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { createItem, createPlayer } from "../../test/mocks";
import { StatsPanel } from "./StatsPanel";

describe("StatsPanel", () => {
  it("displays health values", () => {
    const player = createPlayer({ health: 75, maxHealth: 100 });
    render(<StatsPanel player={player} items={{}} />);
    expect(screen.getByText("75/100")).toBeInTheDocument();
  });

  it("shows green HP bar when health > 60%", () => {
    const player = createPlayer({ health: 80, maxHealth: 100 });
    const { container } = render(<StatsPanel player={player} items={{}} />);
    const bar = container.querySelector("[style]") as HTMLElement;
    expect(bar.style.backgroundColor).toBe("var(--hp-high)");
  });

  it("shows yellow HP bar when health 26-60%", () => {
    const player = createPlayer({ health: 40, maxHealth: 100 });
    const { container } = render(<StatsPanel player={player} items={{}} />);
    const bar = container.querySelector("[style]") as HTMLElement;
    expect(bar.style.backgroundColor).toBe("var(--hp-mid)");
  });

  it("shows red HP bar when health <= 25%", () => {
    const player = createPlayer({ health: 20, maxHealth: 100 });
    const { container } = render(<StatsPanel player={player} items={{}} />);
    const bar = container.querySelector("[style]") as HTMLElement;
    expect(bar.style.backgroundColor).toBe("var(--hp-low)");
  });

  it("includes weapon bonus in attack", () => {
    const sword = createItem({
      id: "sword",
      modifier: { attack: 5, defense: 0, health: 0 },
    });
    const player = createPlayer({ attack: 10, equippedWeapon: "sword" });
    render(<StatsPanel player={player} items={{ sword }} />);
    expect(screen.getByText(/15/)).toBeInTheDocument();
    expect(screen.getByText(/\(\+5\)/)).toBeInTheDocument();
  });

  it("includes armor bonus in defense", () => {
    const shield = createItem({
      id: "shield",
      itemType: "armor",
      modifier: { attack: 0, defense: 3, health: 0 },
    });
    const player = createPlayer({ defense: 5, equippedArmor: "shield" });
    render(<StatsPanel player={player} items={{ shield }} />);
    expect(screen.getByText(/8/)).toBeInTheDocument();
    expect(screen.getByText(/\(\+3\)/)).toBeInTheDocument();
  });

  it("shows turns elapsed", () => {
    const player = createPlayer({ turnsElapsed: 42 });
    render(<StatsPanel player={player} items={{}} />);
    expect(screen.getByText("42")).toBeInTheDocument();
  });
});
