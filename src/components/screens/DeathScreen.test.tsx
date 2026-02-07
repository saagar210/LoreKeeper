import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { createPlayer } from "../../test/mocks";
import { DeathScreen } from "./DeathScreen";

describe("DeathScreen", () => {
  it("renders heading", () => {
    render(
      <DeathScreen
        player={createPlayer()}
        onLoadSave={vi.fn()}
        onNewGame={vi.fn()}
      />,
    );
    expect(screen.getByText("You Have Perished")).toBeInTheDocument();
  });

  it("shows turns survived", () => {
    render(
      <DeathScreen
        player={createPlayer({ turnsElapsed: 42 })}
        onLoadSave={vi.fn()}
        onNewGame={vi.fn()}
      />,
    );
    expect(screen.getByText("Turns survived: 42")).toBeInTheDocument();
  });

  it("shows rooms explored", () => {
    render(
      <DeathScreen
        player={createPlayer({ visitedLocations: ["a", "b", "c"] })}
        onLoadSave={vi.fn()}
        onNewGame={vi.fn()}
      />,
    );
    expect(screen.getByText("Rooms explored: 3")).toBeInTheDocument();
  });

  it("calls onLoadSave when Load Save clicked", async () => {
    const user = userEvent.setup();
    const onLoadSave = vi.fn();
    render(
      <DeathScreen
        player={createPlayer()}
        onLoadSave={onLoadSave}
        onNewGame={vi.fn()}
      />,
    );
    await user.click(screen.getByText("Load Save"));
    expect(onLoadSave).toHaveBeenCalledOnce();
  });

  it("calls onNewGame when New Game clicked", async () => {
    const user = userEvent.setup();
    const onNewGame = vi.fn();
    render(
      <DeathScreen
        player={createPlayer()}
        onLoadSave={vi.fn()}
        onNewGame={onNewGame}
      />,
    );
    await user.click(screen.getByText("New Game"));
    expect(onNewGame).toHaveBeenCalledOnce();
  });
});
