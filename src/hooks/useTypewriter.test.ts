import { act, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useTypewriter } from "./useTypewriter";

describe("useTypewriter", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("displays text instantly when speed is 0", () => {
    const { result } = renderHook(() => useTypewriter("Hello", 0));
    expect(result.current.displayedText).toBe("Hello");
    expect(result.current.isTyping).toBe(false);
  });

  it("displays empty string instantly when speed is 0", () => {
    const { result } = renderHook(() => useTypewriter("", 0));
    expect(result.current.displayedText).toBe("");
    expect(result.current.isTyping).toBe(false);
  });

  it("animates one character per tick", () => {
    const { result } = renderHook(() => useTypewriter("Hi!", 50));

    expect(result.current.displayedText).toBe("");
    expect(result.current.isTyping).toBe(true);

    act(() => vi.advanceTimersByTime(50));
    expect(result.current.displayedText).toBe("H");

    act(() => vi.advanceTimersByTime(50));
    expect(result.current.displayedText).toBe("Hi");

    act(() => vi.advanceTimersByTime(50));
    expect(result.current.displayedText).toBe("Hi!");
    expect(result.current.isTyping).toBe(false);
  });

  it("skip() completes immediately", () => {
    const { result } = renderHook(() => useTypewriter("Hello World", 50));

    expect(result.current.isTyping).toBe(true);

    act(() => result.current.skip());

    expect(result.current.displayedText).toBe("Hello World");
    expect(result.current.isTyping).toBe(false);
  });

  it("restarts animation on text change", () => {
    const { result, rerender } = renderHook(
      ({ text, speed }) => useTypewriter(text, speed),
      { initialProps: { text: "AB", speed: 50 } },
    );

    act(() => vi.advanceTimersByTime(50));
    expect(result.current.displayedText).toBe("A");

    rerender({ text: "XY", speed: 50 });

    expect(result.current.displayedText).toBe("");
    expect(result.current.isTyping).toBe(true);

    act(() => vi.advanceTimersByTime(50));
    expect(result.current.displayedText).toBe("X");
  });

  it("clears interval on unmount", () => {
    const clearSpy = vi.spyOn(globalThis, "clearInterval");
    const { unmount } = renderHook(() => useTypewriter("Hello", 50));

    unmount();
    expect(clearSpy).toHaveBeenCalled();
    clearSpy.mockRestore();
  });
});
