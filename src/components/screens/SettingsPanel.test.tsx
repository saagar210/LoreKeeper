import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { createSettings } from "../../test/mocks";

const mockUpdateSettings = vi.fn();
const mockCheckOllama = vi.fn().mockResolvedValue({ connected: false, version: null });
const mockGetModels = vi.fn().mockResolvedValue([]);

let mockSettingsData = createSettings();
let mockOllamaStatusData: { connected: boolean; version: string | null } = { connected: false, version: null };
let mockModelsData: { name: string; size: number | null }[] = [];

vi.mock("../../hooks/useSettings", () => ({
  useSettings: () => ({
    settings: mockSettingsData,
    updateSettings: mockUpdateSettings,
    ollamaStatus: mockOllamaStatusData,
    checkOllama: mockCheckOllama,
    models: mockModelsData,
    getModels: mockGetModels,
  }),
}));

const { SettingsPanel } = await import("./SettingsPanel");

describe("SettingsPanel", () => {
  beforeEach(() => {
    mockSettingsData = createSettings();
    mockOllamaStatusData = { connected: false, version: null };
    mockModelsData = [];
  });

  it("has role=dialog", () => {
    render(<SettingsPanel onClose={vi.fn()} onThemeChange={vi.fn()} />);
    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });

  it("renders theme buttons", () => {
    render(<SettingsPanel onClose={vi.fn()} onThemeChange={vi.fn()} />);
    expect(screen.getByText("Green Terminal")).toBeInTheDocument();
    expect(screen.getByText("Amber Terminal")).toBeInTheDocument();
    expect(screen.getByText("Parchment")).toBeInTheDocument();
    expect(screen.getByText("Dark Modern")).toBeInTheDocument();
  });

  it("calls updateSettings and onThemeChange when theme clicked", async () => {
    const user = userEvent.setup();
    const onThemeChange = vi.fn();
    render(<SettingsPanel onClose={vi.fn()} onThemeChange={onThemeChange} />);

    await user.click(screen.getByText("Parchment"));

    expect(mockUpdateSettings).toHaveBeenCalledWith({ theme: "parchment" });
    expect(onThemeChange).toHaveBeenCalledWith("parchment");
  });

  it("shows typewriter speed slider", () => {
    render(<SettingsPanel onClose={vi.fn()} onThemeChange={vi.fn()} />);
    expect(screen.getByText(/Typewriter Speed: 30ms/)).toBeInTheDocument();
  });

  it("shows Ollama disconnected status", () => {
    render(<SettingsPanel onClose={vi.fn()} onThemeChange={vi.fn()} />);
    expect(screen.getByText("Disconnected")).toBeInTheDocument();
  });

  it("shows Ollama connected status with version", () => {
    mockOllamaStatusData = { connected: true, version: "0.5.0" };
    render(<SettingsPanel onClose={vi.fn()} onThemeChange={vi.fn()} />);
    expect(screen.getByText("Connected (v0.5.0)")).toBeInTheDocument();
  });

  it("toggles Ollama enabled", async () => {
    const user = userEvent.setup();
    render(<SettingsPanel onClose={vi.fn()} onThemeChange={vi.fn()} />);

    const checkbox = screen.getByRole("checkbox", { name: /Enable LLM Narration/ });
    await user.click(checkbox);

    expect(mockUpdateSettings).toHaveBeenCalledWith({ ollamaEnabled: true });
  });

  it("shows sound effects toggle", () => {
    render(<SettingsPanel onClose={vi.fn()} onThemeChange={vi.fn()} />);
    expect(screen.getByRole("checkbox", { name: /Sound Effects/ })).toBeInTheDocument();
  });

  it("shows difficulty buttons", () => {
    render(<SettingsPanel onClose={vi.fn()} onThemeChange={vi.fn()} />);
    expect(screen.getByText("easy")).toBeInTheDocument();
    expect(screen.getByText("normal")).toBeInTheDocument();
    expect(screen.getByText("hard")).toBeInTheDocument();
  });

  it("shows model selector when Ollama enabled", () => {
    mockSettingsData = createSettings({ ollamaEnabled: true });
    render(<SettingsPanel onClose={vi.fn()} onThemeChange={vi.fn()} />);
    expect(screen.getByText("Model")).toBeInTheDocument();
  });

  it("closes on backdrop click", async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(<SettingsPanel onClose={onClose} onThemeChange={vi.fn()} />);

    await user.click(screen.getByRole("dialog"));
    expect(onClose).toHaveBeenCalled();
  });

  it("closes on [X] click", async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(<SettingsPanel onClose={onClose} onThemeChange={vi.fn()} />);

    await user.click(screen.getByText("[X]"));
    expect(onClose).toHaveBeenCalled();
  });
});
