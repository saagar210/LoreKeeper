import { describe, expect, it } from "vitest";
import { applyTheme, themes } from "./themes";

describe("applyTheme", () => {
  it("sets CSS variables on documentElement", () => {
    applyTheme("greenTerminal");
    const style = document.documentElement.style;
    expect(style.getPropertyValue("--bg")).toBe(themes.greenTerminal["--bg"]);
    expect(style.getPropertyValue("--text")).toBe(themes.greenTerminal["--text"]);
    expect(style.getPropertyValue("--accent")).toBe(themes.greenTerminal["--accent"]);
  });

  it("sets data-theme attribute", () => {
    applyTheme("amberTerminal");
    expect(document.documentElement.getAttribute("data-theme")).toBe("amberTerminal");
  });

  it("overrides previous theme values", () => {
    applyTheme("greenTerminal");
    expect(document.documentElement.style.getPropertyValue("--bg")).toBe("#0a0a0a");

    applyTheme("parchment");
    expect(document.documentElement.style.getPropertyValue("--bg")).toBe("#f5e6c8");
    expect(document.documentElement.getAttribute("data-theme")).toBe("parchment");
  });

  it("sets all variables for each theme", () => {
    for (const [name, vars] of Object.entries(themes)) {
      applyTheme(name as keyof typeof themes);
      for (const [key, value] of Object.entries(vars)) {
        expect(document.documentElement.style.getPropertyValue(key)).toBe(value);
      }
    }
  });
});
