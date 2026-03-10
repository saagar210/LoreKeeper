import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { mockInvoke } from "../test/mocks";
import { createSettings } from "../test/mocks";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

// Import after mock setup
const { useSettings } = await import("./useSettings");

describe("useSettings", () => {
  let errorSpy: ReturnType<typeof vi.spyOn>;
  let windowErrorSpy: ReturnType<typeof vi.spyOn> | null = null;

  beforeEach(() => {
    errorSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    if (typeof window !== "undefined" && window.console !== console) {
      windowErrorSpy = vi.spyOn(window.console, "error").mockImplementation(() => {});
    }
    mockInvoke.mockReset();
  });

  afterEach(() => {
    errorSpy.mockRestore();
    windowErrorSpy?.mockRestore();
    windowErrorSpy = null;
  });

  it("loads settings on mount", async () => {
    const saved = createSettings({ theme: "parchment" });
    mockInvoke.mockResolvedValueOnce(saved);

    const { result } = renderHook(() => useSettings());

    await waitFor(() => {
      expect(result.current.settings.theme).toBe("parchment");
    });
    expect(mockInvoke).toHaveBeenCalledWith("get_settings");
  });

  it("falls back to defaults on load failure", async () => {
    mockInvoke.mockRejectedValueOnce(new Error("no db"));

    const { result } = renderHook(() => useSettings());

    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith("get_settings");
    });
    // Defaults remain
    expect(result.current.settings.theme).toBe("greenTerminal");
    expect(result.current.settings.ollamaEnabled).toBe(false);
  });

  it("updateSettings merges partial and invokes", async () => {
    mockInvoke.mockResolvedValueOnce(createSettings()); // initial load

    const { result } = renderHook(() => useSettings());
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith("get_settings");
    });

    mockInvoke.mockResolvedValueOnce(
      createSettings({ typewriterSpeed: 50 }),
    ); // update call
    await act(async () => {
      const updateResult = await result.current.updateSettings({
        typewriterSpeed: 50,
      });
      expect(updateResult).toEqual({
        ok: true,
        settings: expect.objectContaining({ typewriterSpeed: 50 }),
      });
    });

    expect(mockInvoke).toHaveBeenCalledWith("update_settings", {
      settings: expect.objectContaining({ typewriterSpeed: 50, theme: "greenTerminal" }),
    });
    expect(result.current.settings.typewriterSpeed).toBe(50);
  });

  it("checkOllama sets status", async () => {
    mockInvoke.mockResolvedValueOnce(createSettings()); // initial load

    const { result } = renderHook(() => useSettings());
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalled();
    });

    mockInvoke.mockResolvedValueOnce({ connected: true, version: "0.5.0" });
    await act(async () => {
      await result.current.checkOllama();
    });

    expect(result.current.ollamaStatus.connected).toBe(true);
    expect(result.current.ollamaStatus.version).toBe("0.5.0");
  });

  it("getModels sets model list", async () => {
    mockInvoke.mockResolvedValueOnce(createSettings()); // initial load

    const { result } = renderHook(() => useSettings());
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalled();
    });

    const models = [{ name: "llama3.2", size: 4000000000 }];
    mockInvoke.mockResolvedValueOnce(models);
    await act(async () => {
      await result.current.getModels();
    });

    expect(result.current.models).toEqual(models);
  });

  it("getModels returns empty on failure", async () => {
    mockInvoke.mockResolvedValueOnce(createSettings()); // initial load

    const { result } = renderHook(() => useSettings());
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalled();
    });

    mockInvoke.mockRejectedValueOnce(new Error("fail"));
    await act(async () => {
      await result.current.getModels();
    });

    expect(result.current.models).toEqual([]);
  });
});
