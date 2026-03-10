import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ReplayScreen } from "./ReplayScreen";

const mockInvoke = vi.fn().mockResolvedValue([]);

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

describe("ReplayScreen", () => {
  beforeEach(() => {
    mockInvoke.mockClear();
    mockInvoke.mockResolvedValue([]);
  });

  it("renders with dialog role", async () => {
    render(<ReplayScreen onClose={() => {}} />);
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });

  it("shows Replays heading", async () => {
    render(<ReplayScreen onClose={() => {}} />);
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByText("Replays")).toBeInTheDocument();
  });

  it("shows empty state message when no replays", async () => {
    render(<ReplayScreen onClose={() => {}} />);
    expect(
      await screen.findByText(/No completed playthroughs yet/),
    ).toBeInTheDocument();
  });

  it("calls onClose when X button clicked", async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(<ReplayScreen onClose={onClose} />);
    await user.click(screen.getByText("[X]"));
    expect(onClose).toHaveBeenCalledOnce();
  });

  it("calls onClose when backdrop clicked", async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(<ReplayScreen onClose={onClose} />);
    const dialog = screen.getByRole("dialog");
    await user.click(dialog);
    expect(onClose).toHaveBeenCalledOnce();
  });
});
