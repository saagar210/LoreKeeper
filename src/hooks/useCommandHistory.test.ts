import { act, renderHook } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { useCommandHistory } from "./useCommandHistory";

describe("useCommandHistory", () => {
  it("returns null for getPrevious on empty history", () => {
    const { result } = renderHook(() => useCommandHistory());
    expect(result.current.getPrevious()).toBeNull();
  });

  it("returns null for getNext on empty history", () => {
    const { result } = renderHook(() => useCommandHistory());
    expect(result.current.getNext()).toBeNull();
  });

  it("navigates up through history", () => {
    const { result } = renderHook(() => useCommandHistory());

    act(() => result.current.addCommand("go north"));
    act(() => result.current.addCommand("look"));

    expect(result.current.getPrevious()).toBe("look");
    expect(result.current.getPrevious()).toBe("go north");
  });

  it("stays at oldest when pressing up at start", () => {
    const { result } = renderHook(() => useCommandHistory());

    act(() => result.current.addCommand("go north"));

    expect(result.current.getPrevious()).toBe("go north");
    expect(result.current.getPrevious()).toBe("go north");
  });

  it("navigates down after going up", () => {
    const { result } = renderHook(() => useCommandHistory());

    act(() => result.current.addCommand("go north"));
    act(() => result.current.addCommand("look"));

    // Go up to top
    result.current.getPrevious(); // look
    result.current.getPrevious(); // go north

    // Go back down
    expect(result.current.getNext()).toBe("look");
    // Past the end returns empty string (clears input)
    expect(result.current.getNext()).toBe("");
  });

  it("does not add whitespace-only commands", () => {
    const { result } = renderHook(() => useCommandHistory());

    act(() => result.current.addCommand("   "));
    act(() => result.current.addCommand(""));

    expect(result.current.getPrevious()).toBeNull();
  });

  it("resets index after addCommand", () => {
    const { result } = renderHook(() => useCommandHistory());

    act(() => result.current.addCommand("go north"));
    result.current.getPrevious(); // go north

    act(() => result.current.addCommand("look"));

    // After adding, index resets — getPrevious returns the latest
    expect(result.current.getPrevious()).toBe("look");
  });
});
