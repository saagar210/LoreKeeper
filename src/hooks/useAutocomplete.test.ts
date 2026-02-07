import { act, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useAutocomplete } from "./useAutocomplete";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

import { invoke } from "@tauri-apps/api/core";

describe("useAutocomplete", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.mocked(invoke).mockResolvedValue(["look", "load"]);
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it("starts with empty completions", () => {
    const { result } = renderHook(() => useAutocomplete());
    expect(result.current.completions).toEqual([]);
    expect(result.current.selectedIndex).toBe(-1);
  });

  it("fetches completions after debounce", async () => {
    const { result } = renderHook(() => useAutocomplete());

    act(() => {
      result.current.fetchCompletions("lo");
    });

    // Before debounce
    expect(result.current.completions).toEqual([]);

    // After debounce
    await act(async () => {
      vi.advanceTimersByTime(200);
    });

    expect(invoke).toHaveBeenCalledWith("get_completions", { prefix: "lo" });
    expect(result.current.completions).toEqual(["look", "load"]);
    expect(result.current.selectedIndex).toBe(0);
  });

  it("clears completions on empty prefix", () => {
    const { result } = renderHook(() => useAutocomplete());

    act(() => {
      result.current.fetchCompletions("");
    });

    expect(result.current.completions).toEqual([]);
  });

  it("cycles selection with selectNext/selectPrev", async () => {
    const { result } = renderHook(() => useAutocomplete());

    act(() => {
      result.current.fetchCompletions("lo");
    });

    await act(async () => {
      vi.advanceTimersByTime(200);
    });

    expect(result.current.selectedIndex).toBe(0);

    act(() => {
      result.current.selectNext();
    });
    expect(result.current.selectedIndex).toBe(1);

    act(() => {
      result.current.selectNext();
    });
    expect(result.current.selectedIndex).toBe(0); // wraps

    act(() => {
      result.current.selectPrev();
    });
    expect(result.current.selectedIndex).toBe(1); // wraps back
  });

  it("accept returns selected completion and clears", async () => {
    const { result } = renderHook(() => useAutocomplete());

    act(() => {
      result.current.fetchCompletions("lo");
    });

    await act(async () => {
      vi.advanceTimersByTime(200);
    });

    let accepted: string | null = null;
    act(() => {
      accepted = result.current.accept();
    });

    expect(accepted).toBe("look");
    expect(result.current.completions).toEqual([]);
  });

  it("dismiss clears completions", async () => {
    const { result } = renderHook(() => useAutocomplete());

    act(() => {
      result.current.fetchCompletions("lo");
    });

    await act(async () => {
      vi.advanceTimersByTime(200);
    });

    act(() => {
      result.current.dismiss();
    });

    expect(result.current.completions).toEqual([]);
    expect(result.current.selectedIndex).toBe(-1);
  });
});
