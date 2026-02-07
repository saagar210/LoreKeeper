import { act, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  createCommandResponse,
  createPlayer,
  createWorldState,
  mockInvoke,
} from "./test/mocks";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

// Stub heavy child components to isolate App logic
vi.mock("./components/terminal/Terminal", () => ({
  Terminal: ({ onWorldStateChange }: { onWorldStateChange?: (s: unknown) => void }) => {
    // Expose a way for tests to call onWorldStateChange
    (globalThis as Record<string, unknown>).__terminalOnWSChange = onWorldStateChange;
    return <div data-testid="terminal">Terminal</div>;
  },
}));

vi.mock("./components/sidebar/SidePanel", () => ({
  SidePanel: () => <div data-testid="sidepanel">SidePanel</div>,
}));

// Mock useSettings used by SettingsPanel
vi.mock("./hooks/useSettings", () => ({
  useSettings: () => ({
    settings: {
      ollamaEnabled: false,
      ollamaModel: "llama3.2",
      ollamaUrl: "http://localhost:11434",
      temperature: 0.7,
      narratorTone: "atmospheric",
      typewriterSpeed: 30,
      theme: "greenTerminal",
      narrationVerbosity: "normal",
    },
    updateSettings: vi.fn(),
    ollamaStatus: { connected: false, version: null },
    checkOllama: vi.fn().mockResolvedValue({ connected: false, version: null }),
    models: [],
    getModels: vi.fn().mockResolvedValue([]),
  }),
}));

// Mock Tauri listen (used by SaveLoadScreen indirectly - not needed but prevents errors)
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(vi.fn()),
}));

const { default: App } = await import("./App");

describe("App", () => {
  beforeEach(() => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("renders title screen by default", () => {
    render(<App />);
    expect(screen.getByText("New Game")).toBeInTheDocument();
    expect(screen.getByText("The Depths of Thornhold")).toBeInTheDocument();
  });

  it("transitions to game screen on New Game", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const response = createCommandResponse();
    mockInvoke.mockResolvedValueOnce(response);

    render(<App />);
    await user.click(screen.getByText("New Game"));

    await waitFor(() => {
      expect(screen.getByTestId("terminal")).toBeInTheDocument();
    });
    expect(mockInvoke).toHaveBeenCalledWith("new_game");
  });

  it("opens settings overlay from title screen", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    render(<App />);
    await user.click(screen.getByText("Settings"));
    expect(screen.getByRole("dialog", { name: "Settings" })).toBeInTheDocument();
  });

  it("opens load overlay from title screen", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    mockInvoke.mockResolvedValueOnce([]); // list_saves
    render(<App />);
    await user.click(screen.getByText("Load Game"));
    await waitFor(() => {
      expect(screen.getByRole("dialog", { name: "Load Game" })).toBeInTheDocument();
    });
  });

  it("toggles settings with Escape in game screen", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const response = createCommandResponse();
    mockInvoke.mockResolvedValueOnce(response);

    render(<App />);
    await user.click(screen.getByText("New Game"));

    await waitFor(() => {
      expect(screen.getByTestId("terminal")).toBeInTheDocument();
    });

    // Open settings
    await user.keyboard("{Escape}");
    expect(screen.getByRole("dialog", { name: "Settings" })).toBeInTheDocument();

    // Close settings
    await user.keyboard("{Escape}");
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  });

  it("quick saves with Ctrl+S in game screen", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const response = createCommandResponse();
    mockInvoke.mockResolvedValueOnce(response); // new_game

    render(<App />);
    await user.click(screen.getByText("New Game"));

    await waitFor(() => {
      expect(screen.getByTestId("terminal")).toBeInTheDocument();
    });

    mockInvoke.mockResolvedValueOnce(undefined); // save_game
    await user.keyboard("{Control>}s{/Control}");

    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith("save_game", { slotName: "quicksave" });
    });
  });

  it("shows and auto-clears status message", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const response = createCommandResponse();
    mockInvoke.mockResolvedValueOnce(response); // new_game

    render(<App />);
    await user.click(screen.getByText("New Game"));

    await waitFor(() => {
      expect(screen.getByTestId("terminal")).toBeInTheDocument();
    });

    mockInvoke.mockResolvedValueOnce(undefined); // save_game
    await user.keyboard("{Control>}s{/Control}");

    await waitFor(() => {
      expect(screen.getByText("Game saved.")).toBeInTheDocument();
    });

    // Status clears after 3 seconds
    await act(async () => {
      vi.advanceTimersByTime(3000);
    });
    expect(screen.queryByText("Game saved.")).not.toBeInTheDocument();
  });

  it("shows death screen when game mode is death", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const response = createCommandResponse({
      worldState: createWorldState({ gameMode: { gameOver: "death" } }),
    });
    mockInvoke.mockResolvedValueOnce(response);

    render(<App />);
    await user.click(screen.getByText("New Game"));

    await waitFor(() => {
      expect(screen.getByTestId("terminal")).toBeInTheDocument();
    });

    // Simulate worldState update from Terminal
    const callback = (globalThis as Record<string, unknown>).__terminalOnWSChange as (
      s: unknown,
    ) => void;
    if (callback) {
      act(() => {
        callback(createWorldState({ gameMode: { gameOver: "death" } }));
      });
    }

    expect(screen.getByText("You Have Perished")).toBeInTheDocument();
  });

  it("shows ending screen for victory", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const response = createCommandResponse();
    mockInvoke.mockResolvedValueOnce(response);

    render(<App />);
    await user.click(screen.getByText("New Game"));

    await waitFor(() => {
      expect(screen.getByTestId("terminal")).toBeInTheDocument();
    });

    const callback = (globalThis as Record<string, unknown>).__terminalOnWSChange as (
      s: unknown,
    ) => void;
    if (callback) {
      act(() => {
        callback(
          createWorldState({
            gameMode: { gameOver: "victoryPeace" },
            player: createPlayer({ turnsElapsed: 50, visitedLocations: ["a", "b", "c"] }),
          }),
        );
      });
    }

    expect(screen.getByText("Victory Through Wisdom")).toBeInTheDocument();
  });
});
