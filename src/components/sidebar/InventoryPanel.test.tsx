import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { createItem, createPlayer } from "../../test/mocks";
import { InventoryPanel } from "./InventoryPanel";

describe("InventoryPanel", () => {
  it("shows slot count", () => {
    const player = createPlayer({ inventory: ["sword"], maxInventory: 10 });
    const sword = createItem({ id: "sword", name: "Rusty Sword" });
    render(<InventoryPanel player={player} items={{ sword }} />);
    expect(screen.getByText("Inventory (1/10)")).toBeInTheDocument();
  });

  it("renders item names", () => {
    const player = createPlayer({ inventory: ["sword", "potion"] });
    const items = {
      sword: createItem({ id: "sword", name: "Rusty Sword" }),
      potion: createItem({ id: "potion", name: "Health Potion", itemType: "consumable" }),
    };
    render(<InventoryPanel player={player} items={items} />);
    expect(screen.getByText("Rusty Sword")).toBeInTheDocument();
    expect(screen.getByText("Health Potion")).toBeInTheDocument();
  });

  it("shows [W] tag for equipped weapon", () => {
    const player = createPlayer({
      inventory: ["sword"],
      equippedWeapon: "sword",
    });
    const sword = createItem({ id: "sword", name: "Rusty Sword" });
    render(<InventoryPanel player={player} items={{ sword }} />);
    expect(screen.getByText("[W]")).toBeInTheDocument();
  });

  it("shows [A] tag for equipped armor", () => {
    const player = createPlayer({
      inventory: ["shield"],
      equippedArmor: "shield",
    });
    const shield = createItem({ id: "shield", name: "Iron Shield", itemType: "armor" });
    render(<InventoryPanel player={player} items={{ shield }} />);
    expect(screen.getByText("[A]")).toBeInTheDocument();
  });

  it("shows empty message when no items", () => {
    const player = createPlayer({ inventory: [] });
    render(<InventoryPanel player={player} items={{}} />);
    expect(screen.getByText("Empty")).toBeInTheDocument();
  });
});
