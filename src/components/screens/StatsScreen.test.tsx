import { render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { StatsScreen } from "./StatsScreen";

const mockInvoke = vi.fn().mockResolvedValue({
  rooms_explored: 5,
  enemies_defeated: 2,
  items_collected: 10,
  quests_completed: 1,
  commands_entered: 42,
  deaths: 0,
  games_started: 3,
  total_turns: 100,
});

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

describe("StatsScreen", () => {
  beforeEach(() => {
    mockInvoke.mockClear();
    mockInvoke.mockResolvedValue({
      rooms_explored: 5,
      enemies_defeated: 2,
      items_collected: 10,
      quests_completed: 1,
      commands_entered: 42,
      deaths: 0,
      games_started: 3,
      total_turns: 100,
    });
  });

  it("renders with dialog role", async () => {
    render(<StatsScreen onClose={() => {}} />);
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });

  it("shows Statistics heading", async () => {
    render(<StatsScreen onClose={() => {}} />);
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByText("Statistics")).toBeInTheDocument();
  });

  it("has a Reset Stats button", async () => {
    render(<StatsScreen onClose={() => {}} />);
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByText("Reset Stats")).toBeInTheDocument();
  });
});
