import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { createLocation, createPlayer, createWorldState } from "../../test/mocks";
import { SidePanel } from "./SidePanel";

describe("SidePanel", () => {
  it("renders all five sub-panels", () => {
    const worldState = createWorldState({
      player: createPlayer({
        location: "entrance_hall",
        visitedLocations: ["entrance_hall"],
      }),
      locations: {
        entrance_hall: createLocation({ id: "entrance_hall", name: "Entrance Hall" }),
      },
    });
    render(<SidePanel worldState={worldState} />);

    // RoomInfo heading
    expect(screen.getByText("Entrance Hall")).toBeInTheDocument();
    // StatsPanel heading
    expect(screen.getByText("Stats")).toBeInTheDocument();
    // InventoryPanel heading
    expect(screen.getByText(/Inventory/)).toBeInTheDocument();
    // QuestLog heading
    expect(screen.getByText("Quests")).toBeInTheDocument();
    // MiniMap heading
    expect(screen.getByText("Map")).toBeInTheDocument();
  });

  it("handles missing current location gracefully", () => {
    const worldState = createWorldState({
      player: createPlayer({ location: "nonexistent" }),
      locations: {},
    });
    const { container } = render(<SidePanel worldState={worldState} />);
    // Should still render without crashing
    expect(container.firstChild).toBeInTheDocument();
  });
});
