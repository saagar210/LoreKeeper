import { render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ModuleSelectScreen } from "./ModuleSelectScreen";

const mockInvoke = vi.fn().mockResolvedValue([]);

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

vi.mock("../../lib/focusTrap", () => ({
  trapFocus: vi.fn(() => vi.fn()),
}));

describe("ModuleSelectScreen", () => {
  beforeEach(() => {
    mockInvoke.mockClear();
    mockInvoke.mockResolvedValue([]);
  });

  it("renders heading", async () => {
    render(
      <ModuleSelectScreen
        onModuleLoaded={vi.fn()}
        onClose={vi.fn()}
      />,
    );
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByText("Game Modules")).toBeInTheDocument();
  });

  it("renders close button", async () => {
    render(
      <ModuleSelectScreen
        onModuleLoaded={vi.fn()}
        onClose={vi.fn()}
      />,
    );
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByText("[X]")).toBeInTheDocument();
  });

  it("has dialog role", async () => {
    render(
      <ModuleSelectScreen
        onModuleLoaded={vi.fn()}
        onClose={vi.fn()}
      />,
    );
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });
});
