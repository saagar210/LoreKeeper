import { describe, expect, it, vi } from "vitest";
import { formatRelativeTime } from "./format";

describe("formatRelativeTime", () => {
  it('returns "just now" for very recent times', () => {
    const now = new Date().toISOString();
    expect(formatRelativeTime(now)).toBe("just now");
  });

  it('returns "just now" for future dates', () => {
    const future = new Date(Date.now() + 60_000).toISOString();
    expect(formatRelativeTime(future)).toBe("just now");
  });

  it("returns minutes ago", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2025-01-15T12:05:00Z"));
    expect(formatRelativeTime("2025-01-15T12:02:00Z")).toBe("3 min ago");
    vi.useRealTimers();
  });

  it("returns singular minute", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2025-01-15T12:01:30Z"));
    expect(formatRelativeTime("2025-01-15T12:00:00Z")).toBe("1 min ago");
    vi.useRealTimers();
  });

  it("returns hours ago", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2025-01-15T14:00:00Z"));
    expect(formatRelativeTime("2025-01-15T12:00:00Z")).toBe("2 hours ago");
    vi.useRealTimers();
  });

  it("returns days ago", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2025-01-18T12:00:00Z"));
    expect(formatRelativeTime("2025-01-15T12:00:00Z")).toBe("3 days ago");
    vi.useRealTimers();
  });

  it("returns months ago", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2025-04-15T12:00:00Z"));
    expect(formatRelativeTime("2025-01-15T12:00:00Z")).toBe("3 months ago");
    vi.useRealTimers();
  });

  it("returns years ago", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2027-01-15T12:00:00Z"));
    expect(formatRelativeTime("2025-01-15T12:00:00Z")).toBe("2 years ago");
    vi.useRealTimers();
  });
});
