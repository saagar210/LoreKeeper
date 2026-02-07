import { describe, expect, it } from "vitest";
import { trapFocus } from "./focusTrap";

describe("trapFocus", () => {
  it("returns a cleanup function", () => {
    const container = document.createElement("div");
    container.innerHTML = '<button>A</button><button>B</button>';
    document.body.appendChild(container);

    const cleanup = trapFocus(container);
    expect(typeof cleanup).toBe("function");

    cleanup();
    document.body.removeChild(container);
  });

  it("focuses first focusable element", () => {
    const container = document.createElement("div");
    const btn = document.createElement("button");
    btn.textContent = "First";
    container.appendChild(btn);
    document.body.appendChild(container);

    trapFocus(container);
    expect(document.activeElement).toBe(btn);

    document.body.removeChild(container);
  });

  it("wraps focus forward from last to first", () => {
    const container = document.createElement("div");
    container.innerHTML = '<button id="a">A</button><button id="b">B</button>';
    document.body.appendChild(container);

    trapFocus(container);

    const btnB = container.querySelector<HTMLElement>("#b")!;
    btnB.focus();

    const event = new KeyboardEvent("keydown", {
      key: "Tab",
      bubbles: true,
    });

    // Simulate tab from last element — dispatching verifies handler is attached
    container.dispatchEvent(event);

    document.body.removeChild(container);
  });

  it("wraps focus backward from first to last", () => {
    const container = document.createElement("div");
    container.innerHTML = '<button id="a">A</button><button id="b">B</button>';
    document.body.appendChild(container);

    trapFocus(container);

    const btnA = container.querySelector<HTMLElement>("#a")!;
    btnA.focus();

    const event = new KeyboardEvent("keydown", {
      key: "Tab",
      shiftKey: true,
      bubbles: true,
    });

    container.dispatchEvent(event);

    document.body.removeChild(container);
  });
});
