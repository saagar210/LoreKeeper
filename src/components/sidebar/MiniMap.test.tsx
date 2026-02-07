import { render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { createLocation, createPlayer } from "../../test/mocks";
import type { MapData } from "../../store/types";

const mockInvoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

const { MiniMap } = await import("./MiniMap");

describe("MiniMap", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  it("renders SVG map with current location highlighted", async () => {
    const mapData: MapData = {
      nodes: [
        { id: "entrance_hall", name: "Entrance Hall", x: 40, y: 40, visited: true, current: true },
        { id: "great_hall", name: "Great Hall", x: 140, y: 40, visited: true, current: false },
      ],
      edges: [{ from: "entrance_hall", to: "great_hall", locked: false }],
    };
    mockInvoke.mockResolvedValueOnce(mapData);

    const player = createPlayer({ location: "entrance_hall", visitedLocations: ["entrance_hall", "great_hall"] });
    const locations = {
      entrance_hall: createLocation({ id: "entrance_hall", name: "Entrance Hall" }),
      great_hall: createLocation({ id: "great_hall", name: "Great Hall" }),
    };
    render(<MiniMap locations={locations} player={player} />);

    await waitFor(() => {
      expect(screen.getByRole("img", { name: "Game map" })).toBeInTheDocument();
    });
    expect(screen.getByText("Entrance Hall")).toBeInTheDocument();
    expect(screen.getByText("Great Hall")).toBeInTheDocument();
  });

  it("shows loading state before data arrives", () => {
    mockInvoke.mockReturnValue(new Promise(() => {})); // never resolves

    const player = createPlayer({ location: "a" });
    const locations = { a: createLocation({ id: "a", name: "Room A" }) };
    render(<MiniMap locations={locations} player={player} />);

    expect(screen.getByText("Loading...")).toBeInTheDocument();
  });

  it("shows locked edges as dashed lines", async () => {
    const mapData: MapData = {
      nodes: [
        { id: "a", name: "Room A", x: 40, y: 40, visited: true, current: true },
        { id: "b", name: "Room B", x: 140, y: 40, visited: true, current: false },
      ],
      edges: [{ from: "a", to: "b", locked: true }],
    };
    mockInvoke.mockResolvedValueOnce(mapData);

    const player = createPlayer({ location: "a", visitedLocations: ["a", "b"] });
    const locations = {
      a: createLocation({ id: "a", name: "Room A" }),
      b: createLocation({ id: "b", name: "Room B" }),
    };
    const { container } = render(<MiniMap locations={locations} player={player} />);

    await waitFor(() => {
      expect(screen.getByRole("img", { name: "Game map" })).toBeInTheDocument();
    });

    const line = container.querySelector("line");
    expect(line).toHaveAttribute("stroke-dasharray", "4 2");
  });
});
