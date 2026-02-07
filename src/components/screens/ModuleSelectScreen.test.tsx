import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ModuleSelectScreen } from "./ModuleSelectScreen";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue([]),
}));

vi.mock("../../lib/focusTrap", () => ({
  trapFocus: vi.fn(() => vi.fn()),
}));

describe("ModuleSelectScreen", () => {
  it("renders heading", () => {
    render(
      <ModuleSelectScreen
        onModuleLoaded={vi.fn()}
        onClose={vi.fn()}
      />,
    );
    expect(screen.getByText("Game Modules")).toBeInTheDocument();
  });

  it("renders close button", () => {
    render(
      <ModuleSelectScreen
        onModuleLoaded={vi.fn()}
        onClose={vi.fn()}
      />,
    );
    expect(screen.getByText("[X]")).toBeInTheDocument();
  });

  it("has dialog role", () => {
    render(
      <ModuleSelectScreen
        onModuleLoaded={vi.fn()}
        onClose={vi.fn()}
      />,
    );
    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });
});
