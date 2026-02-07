import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { TerminalInput } from "./TerminalInput";

describe("TerminalInput", () => {
  it("submits on Enter and clears input", async () => {
    const user = userEvent.setup();
    const onSubmit = vi.fn();
    render(<TerminalInput onSubmit={onSubmit} />);

    const input = screen.getByLabelText("Game command input");
    await user.type(input, "go north{Enter}");

    expect(onSubmit).toHaveBeenCalledWith("go north");
    expect(input).toHaveValue("");
  });

  it("does not submit empty input", async () => {
    const user = userEvent.setup();
    const onSubmit = vi.fn();
    render(<TerminalInput onSubmit={onSubmit} />);

    const input = screen.getByLabelText("Game command input");
    await user.type(input, "{Enter}");

    expect(onSubmit).not.toHaveBeenCalled();
  });

  it("disables input when disabled prop is true", () => {
    render(<TerminalInput onSubmit={vi.fn()} disabled />);
    const input = screen.getByLabelText("Game command input");
    expect(input).toBeDisabled();
    expect(input).toHaveAttribute("placeholder", "...");
  });

  it("shows correct placeholder when enabled", () => {
    render(<TerminalInput onSubmit={vi.fn()} />);
    const input = screen.getByLabelText("Game command input");
    expect(input).toHaveAttribute("placeholder", "Type a command...");
  });

  it("navigates history with ArrowUp", async () => {
    const user = userEvent.setup();
    const onSubmit = vi.fn();
    render(<TerminalInput onSubmit={onSubmit} />);

    const input = screen.getByLabelText("Game command input");

    // Submit two commands to build history
    await user.type(input, "go north{Enter}");
    await user.type(input, "look{Enter}");

    // Press up to get last command
    await user.keyboard("{ArrowUp}");
    expect(input).toHaveValue("look");

    await user.keyboard("{ArrowUp}");
    expect(input).toHaveValue("go north");
  });

  it("navigates history with ArrowDown after ArrowUp", async () => {
    const user = userEvent.setup();
    const onSubmit = vi.fn();
    render(<TerminalInput onSubmit={onSubmit} />);

    const input = screen.getByLabelText("Game command input");

    await user.type(input, "first{Enter}");
    await user.type(input, "second{Enter}");

    await user.keyboard("{ArrowUp}"); // second
    await user.keyboard("{ArrowUp}"); // first
    await user.keyboard("{ArrowDown}"); // second
    expect(input).toHaveValue("second");
  });
});
