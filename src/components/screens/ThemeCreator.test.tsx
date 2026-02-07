import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ThemeCreator } from "./ThemeCreator";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue([]),
}));

vi.mock("../../lib/focusTrap", () => ({
  trapFocus: vi.fn(() => vi.fn()),
}));

vi.mock("../../lib/themes", () => ({
  applyCustomTheme: vi.fn(),
  applyTheme: vi.fn(),
  themes: {
    greenTerminal: {
      "--bg": "#0a0a0a",
      "--text": "#33ff33",
      "--text-dim": "#1a8c1a",
      "--text-bright": "#66ff66",
      "--accent": "#00cc00",
      "--error": "#ff4444",
      "--combat": "#ff6666",
      "--dialogue": "#66cccc",
      "--input": "#ffaa33",
      "--system": "#888888",
      "--border": "#1a3a1a",
      "--panel-bg": "#0d0d0d",
      "--hp-high": "#33ff33",
      "--hp-mid": "#ffcc00",
      "--hp-low": "#ff4444",
    },
  },
  themeVarNames: [
    "--bg", "--text", "--text-dim", "--text-bright", "--accent",
    "--error", "--combat", "--dialogue", "--input", "--system",
    "--border", "--panel-bg", "--hp-high", "--hp-mid", "--hp-low",
  ],
}));

describe("ThemeCreator", () => {
  it("renders heading", () => {
    render(<ThemeCreator onClose={vi.fn()} />);
    expect(screen.getByText("Theme Creator")).toBeInTheDocument();
  });

  it("renders color pickers for all theme vars", () => {
    render(<ThemeCreator onClose={vi.fn()} />);
    expect(screen.getByText("Background")).toBeInTheDocument();
    expect(screen.getByText("Accent")).toBeInTheDocument();
    expect(screen.getByText("Error")).toBeInTheDocument();
  });

  it("renders preview section", () => {
    render(<ThemeCreator onClose={vi.fn()} />);
    expect(screen.getByText("Normal text sample.")).toBeInTheDocument();
    expect(screen.getByText("Combat text.")).toBeInTheDocument();
  });

  it("has dialog role", () => {
    render(<ThemeCreator onClose={vi.fn()} />);
    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });

  it("renders save and reset buttons", () => {
    render(<ThemeCreator onClose={vi.fn()} />);
    expect(screen.getByText("Save")).toBeInTheDocument();
    expect(screen.getByText("Reset")).toBeInTheDocument();
  });
});
