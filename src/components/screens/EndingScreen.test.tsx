import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { createPlayer } from "../../test/mocks";
import { EndingScreen } from "./EndingScreen";

describe("EndingScreen", () => {
  it("shows peace victory title and description", () => {
    render(
      <EndingScreen
        endingType="victoryPeace"
        player={createPlayer()}
        onPlayAgain={vi.fn()}
      />,
    );
    expect(screen.getByText("Victory Through Wisdom")).toBeInTheDocument();
    expect(screen.getByText(/wisdom and compassion/)).toBeInTheDocument();
  });

  it("shows combat victory title and description", () => {
    render(
      <EndingScreen
        endingType="victoryCombat"
        player={createPlayer()}
        onPlayAgain={vi.fn()}
      />,
    );
    expect(screen.getByText("Victory Through Strength")).toBeInTheDocument();
    expect(screen.getByText(/blade and determination/)).toBeInTheDocument();
  });

  it("shows stats", () => {
    render(
      <EndingScreen
        endingType="victoryPeace"
        player={createPlayer({ turnsElapsed: 30, visitedLocations: ["a", "b"] })}
        onPlayAgain={vi.fn()}
      />,
    );
    expect(screen.getByText("Turns taken: 30")).toBeInTheDocument();
    expect(screen.getByText("Rooms explored: 2")).toBeInTheDocument();
  });

  it("calls onPlayAgain when Play Again clicked", async () => {
    const user = userEvent.setup();
    const onPlayAgain = vi.fn();
    render(
      <EndingScreen
        endingType="victoryPeace"
        player={createPlayer()}
        onPlayAgain={onPlayAgain}
      />,
    );
    await user.click(screen.getByText("Play Again"));
    expect(onPlayAgain).toHaveBeenCalledOnce();
  });
});
