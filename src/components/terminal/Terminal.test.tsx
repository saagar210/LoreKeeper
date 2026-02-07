import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import type { OutputLine, WorldState } from "../../store/types";
import { createWorldState } from "../../test/mocks";

const mockInitializeGame = vi.fn();
const mockSendCommand = vi.fn();
const mockNewGame = vi.fn();

vi.mock("../../hooks/useGame", () => ({
  useGame: () => ({
    history: mockHistory,
    worldState: mockWorldState,
    isReady: mockIsReady,
    isNarrating: mockIsNarrating,
    initializeGame: mockInitializeGame,
    sendCommand: mockSendCommand,
    newGame: mockNewGame,
  }),
}));

let mockHistory: OutputLine[] = [];
let mockWorldState: WorldState | null = null;
let mockIsReady = false;
let mockIsNarrating = false;

const { Terminal } = await import("./Terminal");

describe("Terminal", () => {
  beforeEach(() => {
    mockHistory = [{ text: "Welcome.", lineType: "narration" }];
    mockWorldState = createWorldState();
    mockIsReady = true;
    mockIsNarrating = false;
  });

  it("calls initializeGame on mount", () => {
    render(<Terminal />);
    expect(mockInitializeGame).toHaveBeenCalled();
  });

  it("disables input when not ready", () => {
    mockIsReady = false;
    render(<Terminal />);
    expect(screen.getByLabelText("Game command input")).toBeDisabled();
  });

  it("disables input when narrating", () => {
    mockIsNarrating = true;
    render(<Terminal />);
    expect(screen.getByLabelText("Game command input")).toBeDisabled();
  });

  it("disables input when game is over", () => {
    mockWorldState = createWorldState({
      gameMode: { gameOver: "death" },
    });
    render(<Terminal />);
    expect(screen.getByLabelText("Game command input")).toBeDisabled();
  });

  it("wires sendCommand to input", async () => {
    const user = userEvent.setup();
    render(<Terminal />);
    const input = screen.getByLabelText("Game command input");
    await user.type(input, "look{Enter}");
    expect(mockSendCommand).toHaveBeenCalledWith("look");
  });

  it("calls onWorldStateChange when worldState changes", () => {
    const callback = vi.fn();
    render(<Terminal onWorldStateChange={callback} />);
    expect(callback).toHaveBeenCalledWith(mockWorldState);
  });
});
