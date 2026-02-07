import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { StatsScreen } from "./StatsScreen";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue({
    rooms_explored: 5,
    enemies_defeated: 2,
    items_collected: 10,
    quests_completed: 1,
    commands_entered: 42,
    deaths: 0,
    games_started: 3,
    total_turns: 100,
  }),
}));

describe("StatsScreen", () => {
  it("renders with dialog role", () => {
    render(<StatsScreen onClose={() => {}} />);
    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });

  it("shows Statistics heading", () => {
    render(<StatsScreen onClose={() => {}} />);
    expect(screen.getByText("Statistics")).toBeInTheDocument();
  });

  it("has a Reset Stats button", () => {
    render(<StatsScreen onClose={() => {}} />);
    expect(screen.getByText("Reset Stats")).toBeInTheDocument();
  });
});
