import { act, renderHook, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import {
  createCommandResponse,
  createOutputLines,
  mockInvoke,
  mockListen,
  mockUnlisten,
} from "../test/mocks";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: (...args: unknown[]) => mockListen(...args),
}));

// Default: listen resolves with unlisten fn
mockListen.mockResolvedValue(mockUnlisten);

const { useGame } = await import("./useGame");

describe("useGame", () => {
  it("initializeGame sets history, worldState, isReady", async () => {
    const response = createCommandResponse();
    mockInvoke.mockResolvedValueOnce(response);

    const { result } = renderHook(() => useGame());

    await act(async () => {
      await result.current.initializeGame();
    });

    expect(mockInvoke).toHaveBeenCalledWith("initialize_game");
    expect(result.current.history).toEqual(response.messages);
    expect(result.current.worldState).toEqual(response.worldState);
    expect(result.current.isReady).toBe(true);
  });

  it("initializeGame shows error on failure", async () => {
    mockInvoke.mockRejectedValueOnce("Backend exploded");

    const { result } = renderHook(() => useGame());

    await act(async () => {
      await result.current.initializeGame();
    });

    expect(result.current.history[0].lineType).toBe("error");
    expect(result.current.history[0].text).toContain("Backend exploded");
    expect(result.current.isReady).toBe(false);
  });

  it("sendCommand echoes input and appends response", async () => {
    const initResponse = createCommandResponse();
    mockInvoke.mockResolvedValueOnce(initResponse);

    const { result } = renderHook(() => useGame());
    await act(async () => {
      await result.current.initializeGame();
    });

    const cmdResponse = createCommandResponse({
      messages: [{ text: "You go north.", lineType: "narration" }],
    });
    mockInvoke.mockResolvedValueOnce(cmdResponse);

    await act(async () => {
      await result.current.sendCommand("go north");
    });

    expect(mockInvoke).toHaveBeenCalledWith("process_command", { input: "go north" });

    // Should have: init message, player echo, command response
    const lines = result.current.history;
    const echoLine = lines.find((l) => l.text === "> go north");
    expect(echoLine).toBeDefined();
    expect(echoLine?.lineType).toBe("playerInput");

    const responseLine = lines.find((l) => l.text === "You go north.");
    expect(responseLine).toBeDefined();
  });

  it("ignores empty input", async () => {
    const { result } = renderHook(() => useGame());

    await act(async () => {
      await result.current.sendCommand("   ");
    });

    expect(mockInvoke).not.toHaveBeenCalledWith("process_command", expect.anything());
  });

  it("shows error on sendCommand failure", async () => {
    const initResponse = createCommandResponse();
    mockInvoke.mockResolvedValueOnce(initResponse);

    const { result } = renderHook(() => useGame());
    await act(async () => {
      await result.current.initializeGame();
    });

    mockInvoke.mockRejectedValueOnce("Parse error");

    await act(async () => {
      await result.current.sendCommand("xyzzy");
    });

    const errorLine = result.current.history.find(
      (l) => l.lineType === "error" && l.text.includes("Parse error"),
    );
    expect(errorLine).toBeDefined();
  });

  it("trims history at MAX_HISTORY_LINES", async () => {
    // Init with lots of lines
    const longMessages = createOutputLines(490);
    mockInvoke.mockResolvedValueOnce(
      createCommandResponse({ messages: longMessages }),
    );

    const { result } = renderHook(() => useGame());
    await act(async () => {
      await result.current.initializeGame();
    });

    // Add more lines via sendCommand to exceed 500
    const moreMessages = createOutputLines(20);
    mockInvoke.mockResolvedValueOnce(
      createCommandResponse({ messages: moreMessages }),
    );

    await act(async () => {
      await result.current.sendCommand("look");
    });

    // 490 init + 1 echo + 20 response = 511, trimmed to 500
    expect(result.current.history.length).toBeLessThanOrEqual(500);
  });

  it("newGame replaces state", async () => {
    const initResponse = createCommandResponse();
    mockInvoke.mockResolvedValueOnce(initResponse);

    const { result } = renderHook(() => useGame());
    await act(async () => {
      await result.current.initializeGame();
    });

    const newResponse = createCommandResponse({
      messages: [{ text: "A new adventure begins.", lineType: "narration" }],
    });
    mockInvoke.mockResolvedValueOnce(newResponse);

    await act(async () => {
      await result.current.newGame();
    });

    expect(mockInvoke).toHaveBeenCalledWith("new_game");
    expect(result.current.history).toEqual(newResponse.messages);
  });

  it("sets up narrative event listener on mount", () => {
    renderHook(() => useGame());
    expect(mockListen).toHaveBeenCalledWith("narrative-event", expect.any(Function));
  });

  it("calls unlisten on unmount", async () => {
    const { unmount } = renderHook(() => useGame());

    // Wait for listen promise to resolve
    await waitFor(() => {
      expect(mockListen).toHaveBeenCalled();
    });

    unmount();
    expect(mockUnlisten).toHaveBeenCalled();
  });
});
