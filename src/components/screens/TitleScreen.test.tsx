import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { TitleScreen } from "./TitleScreen";

describe("TitleScreen", () => {
  it("renders three buttons", () => {
    render(
      <TitleScreen onNewGame={vi.fn()} onLoadGame={vi.fn()} onSettings={vi.fn()} />,
    );
    expect(screen.getByText("New Game")).toBeInTheDocument();
    expect(screen.getByText("Load Game")).toBeInTheDocument();
    expect(screen.getByText("Settings")).toBeInTheDocument();
  });

  it("displays subtitle", () => {
    render(
      <TitleScreen onNewGame={vi.fn()} onLoadGame={vi.fn()} onSettings={vi.fn()} />,
    );
    expect(screen.getByText("The Depths of Thornhold")).toBeInTheDocument();
  });

  it("calls onNewGame when New Game clicked", async () => {
    const user = userEvent.setup();
    const onNewGame = vi.fn();
    render(
      <TitleScreen onNewGame={onNewGame} onLoadGame={vi.fn()} onSettings={vi.fn()} />,
    );
    await user.click(screen.getByText("New Game"));
    expect(onNewGame).toHaveBeenCalledOnce();
  });

  it("calls onLoadGame when Load Game clicked", async () => {
    const user = userEvent.setup();
    const onLoadGame = vi.fn();
    render(
      <TitleScreen onNewGame={vi.fn()} onLoadGame={onLoadGame} onSettings={vi.fn()} />,
    );
    await user.click(screen.getByText("Load Game"));
    expect(onLoadGame).toHaveBeenCalledOnce();
  });

  it("calls onSettings when Settings clicked", async () => {
    const user = userEvent.setup();
    const onSettings = vi.fn();
    render(
      <TitleScreen onNewGame={vi.fn()} onLoadGame={vi.fn()} onSettings={onSettings} />,
    );
    await user.click(screen.getByText("Settings"));
    expect(onSettings).toHaveBeenCalledOnce();
  });
});
