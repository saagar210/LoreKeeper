import type { ThemeConfig, ThemeName } from "../store/types";

export const themes: Record<ThemeName, Record<string, string>> = {
  greenTerminal: {
    "--bg": "#0a0a0a",
    "--text": "#33ff33",
    "--text-dim": "#1a8c1a",
    "--text-bright": "#66ff66",
    "--accent": "#00cc00",
    "--error": "#ff4444",
    "--combat": "#ff6666",
    "--dialogue": "#66cccc",
    "--input": "#ffaa33",
    "--system": "#888888",
    "--border": "#1a3a1a",
    "--panel-bg": "#0d0d0d",
    "--hp-high": "#33ff33",
    "--hp-mid": "#ffcc00",
    "--hp-low": "#ff4444",
  },
  amberTerminal: {
    "--bg": "#0a0a0a",
    "--text": "#ffaa00",
    "--text-dim": "#8c5c00",
    "--text-bright": "#ffcc44",
    "--accent": "#ff8800",
    "--error": "#ff4444",
    "--combat": "#ff6666",
    "--dialogue": "#66cccc",
    "--input": "#ffdd66",
    "--system": "#888888",
    "--border": "#3a2a0a",
    "--panel-bg": "#0d0d0d",
    "--hp-high": "#ffaa00",
    "--hp-mid": "#ffcc00",
    "--hp-low": "#ff4444",
  },
  parchment: {
    "--bg": "#f5e6c8",
    "--text": "#2c1810",
    "--text-dim": "#6b4c3b",
    "--text-bright": "#1a0e08",
    "--accent": "#8b4513",
    "--error": "#cc0000",
    "--combat": "#8b0000",
    "--dialogue": "#006666",
    "--input": "#8b4513",
    "--system": "#666666",
    "--border": "#c4a882",
    "--panel-bg": "#efe0c8",
    "--hp-high": "#228b22",
    "--hp-mid": "#daa520",
    "--hp-low": "#cc0000",
  },
  darkModern: {
    "--bg": "#1a1a2e",
    "--text": "#e0e0e0",
    "--text-dim": "#888888",
    "--text-bright": "#ffffff",
    "--accent": "#4a9eff",
    "--error": "#ff6b6b",
    "--combat": "#ff8888",
    "--dialogue": "#88dddd",
    "--input": "#ffd700",
    "--system": "#aaaaaa",
    "--border": "#2a2a4e",
    "--panel-bg": "#16162b",
    "--hp-high": "#4caf50",
    "--hp-mid": "#ff9800",
    "--hp-low": "#f44336",
  },
};

export function applyTheme(theme: ThemeName): void {
  const root = document.documentElement;
  const vars = themes[theme];
  for (const [key, value] of Object.entries(vars)) {
    root.style.setProperty(key, value);
  }
  root.setAttribute("data-theme", theme);
}

export function applyCustomTheme(config: ThemeConfig): void {
  const root = document.documentElement;
  for (const [key, value] of Object.entries(config)) {
    root.style.setProperty(key, value);
  }
  root.setAttribute("data-theme", "custom");
}

export const themeVarNames = [
  "--bg",
  "--text",
  "--text-dim",
  "--text-bright",
  "--accent",
  "--error",
  "--combat",
  "--dialogue",
  "--input",
  "--system",
  "--border",
  "--panel-bg",
  "--hp-high",
  "--hp-mid",
  "--hp-low",
] as const;
