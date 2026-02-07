import "@testing-library/jest-dom/vitest";
import { afterEach, vi } from "vitest";

// jsdom doesn't implement scrollIntoView
Element.prototype.scrollIntoView = vi.fn();

// jsdom doesn't implement matchMedia
Object.defineProperty(window, "matchMedia", {
  writable: true,
  value: vi.fn().mockImplementation((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

afterEach(() => {
  vi.clearAllMocks();
});
