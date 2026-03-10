import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
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

  it("loads a module by moduleId instead of file path", async () => {
    const user = userEvent.setup();
    const onModuleLoaded = vi.fn();
    mockInvoke
      .mockResolvedValueOnce([
        {
          name: "thornhold",
          description: "2 locations, 1 items",
          moduleId: "thornhold.json",
          locationCount: 2,
          itemCount: 1,
        },
      ])
      .mockResolvedValueOnce({ messages: [], worldState: {}, soundCues: [] });

    render(
      <ModuleSelectScreen
        onModuleLoaded={onModuleLoaded}
        onClose={vi.fn()}
      />,
    );

    await waitFor(() => expect(screen.getByText("thornhold")).toBeInTheDocument());
    await user.click(screen.getByText("Load"));

    expect(mockInvoke).toHaveBeenNthCalledWith(2, "load_module", {
      moduleId: "thornhold.json",
    });
    expect(onModuleLoaded).toHaveBeenCalled();
  });
});
