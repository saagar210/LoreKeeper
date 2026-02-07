import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import type { Direction } from "../../store/types";
import { createItem, createLocation, createNpc } from "../../test/mocks";
import { RoomInfo } from "./RoomInfo";

describe("RoomInfo", () => {
  it("renders location name", () => {
    const location = createLocation({ name: "Grand Hall" });
    render(<RoomInfo location={location} items={{}} npcs={{}} />);
    expect(screen.getByText("Grand Hall")).toBeInTheDocument();
  });

  it("renders exits", () => {
    const location = createLocation({
      exits: { north: "hall", east: "garden" } as Record<Direction, string>,
    });
    render(<RoomInfo location={location} items={{}} npcs={{}} />);
    expect(screen.getByText(/north/)).toBeInTheDocument();
    expect(screen.getByText(/east/)).toBeInTheDocument();
  });

  it("shows locked indicator on locked exits", () => {
    const location = createLocation({
      exits: { north: "hall", east: "vault" } as Record<Direction, string>,
      lockedExits: { east: "iron_key" } as Record<Direction, string>,
    });
    render(<RoomInfo location={location} items={{}} npcs={{}} />);
    expect(screen.getByText(/east \(locked\)/)).toBeInTheDocument();
  });

  it("renders NPC names", () => {
    const npc = createNpc({ id: "sage", name: "Old Sage" });
    const location = createLocation({ npcs: ["sage"] });
    render(<RoomInfo location={location} items={{}} npcs={{ sage: npc }} />);
    expect(screen.getByText("Old Sage")).toBeInTheDocument();
  });

  it("renders item names", () => {
    const item = createItem({ id: "key", name: "Iron Key" });
    const location = createLocation({ items: ["key"] });
    render(<RoomInfo location={location} items={{ key: item }} npcs={{}} />);
    expect(screen.getByText("Iron Key")).toBeInTheDocument();
  });

  it("handles no exits, NPCs, or items gracefully", () => {
    const location = createLocation({ exits: {} as Record<Direction, string>, npcs: [], items: [] });
    const { container } = render(
      <RoomInfo location={location} items={{}} npcs={{}} />,
    );
    expect(container.querySelector("div")).toBeInTheDocument();
    expect(screen.queryByText("Exits:")).not.toBeInTheDocument();
  });
});
