import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ThemeCreator } from "./ThemeCreator";

const mockInvoke = vi.fn().mockResolvedValue([]);
const mockApplyCustomTheme = vi.fn((_: unknown) => true);

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

vi.mock("../../lib/focusTrap", () => ({
  trapFocus: vi.fn(() => vi.fn()),
}));

vi.mock("../../lib/themes", () => ({
  applyCustomTheme: (config: unknown) => mockApplyCustomTheme(config),
  applyTheme: vi.fn(),
  createDefaultCustomThemeConfig: () => ({
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
  }),
  sanitizeCustomThemeConfig: (config: Record<string, string>) =>
    Object.values(config).every((value) => /^#[0-9a-f]{6}$/i.test(value))
      ? config
      : null,
  themeVarNames: [
    "--bg",
    "--text",
    "--text-dim",
    "--text-bright",
    "--accent",
    "--error",
    "--combat",
    "--dialogue",
    "--input",
    "--system",
    "--border",
    "--panel-bg",
    "--hp-high",
    "--hp-mid",
    "--hp-low",
  ],
}));

describe("ThemeCreator", () => {
  beforeEach(() => {
    mockInvoke.mockClear();
    mockInvoke.mockResolvedValue([]);
    mockApplyCustomTheme.mockClear();
    mockApplyCustomTheme.mockReturnValue(true);
  });

  it("renders heading", async () => {
    render(<ThemeCreator onClose={vi.fn()} />);
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByText("Theme Creator")).toBeInTheDocument();
  });

  it("renders color pickers for all theme vars", async () => {
    render(<ThemeCreator onClose={vi.fn()} />);
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByText("Background")).toBeInTheDocument();
    expect(screen.getByText("Accent")).toBeInTheDocument();
    expect(screen.getByText("Error")).toBeInTheDocument();
  });

  it("renders preview section", async () => {
    render(<ThemeCreator onClose={vi.fn()} />);
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByText("Normal text sample.")).toBeInTheDocument();
    expect(screen.getByText("Combat text.")).toBeInTheDocument();
  });

  it("has dialog role", async () => {
    render(<ThemeCreator onClose={vi.fn()} />);
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });

  it("renders save and reset buttons", async () => {
    render(<ThemeCreator onClose={vi.fn()} />);
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByText("Save")).toBeInTheDocument();
    expect(screen.getByText("Reset")).toBeInTheDocument();
  });

  it("shows a validation message for invalid theme names", async () => {
    const user = userEvent.setup();
    render(<ThemeCreator onClose={vi.fn()} />);
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());

    await user.type(screen.getByPlaceholderText("Theme name"), "bad/theme");
    await user.click(screen.getByText("Save"));

    expect(
      screen.getByText(
        "Theme name can only use letters, numbers, spaces, '-' and '_'.",
      ),
    ).toBeInTheDocument();
    expect(mockInvoke).not.toHaveBeenCalledWith(
      "save_custom_theme",
      expect.anything(),
    );
  });

  it("loads a saved theme only when the config is valid", async () => {
    const user = userEvent.setup();
    mockInvoke.mockResolvedValueOnce([
      {
        name: "Amber Night",
        config: JSON.stringify({
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
        }),
      },
    ]);

    render(<ThemeCreator onClose={vi.fn()} />);
    await user.click(
      await screen.findByRole("button", { name: "Amber Night" }),
    );

    expect(screen.getByDisplayValue("Amber Night")).toBeInTheDocument();
  });
});
