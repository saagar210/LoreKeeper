import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import type { OutputLine } from "../../store/types";
import { TerminalOutput } from "./TerminalOutput";

describe("TerminalOutput", () => {
  it("has role=log and aria-live=polite", () => {
    render(<TerminalOutput lines={[]} />);
    const log = screen.getByRole("log");
    expect(log).toHaveAttribute("aria-live", "polite");
  });

  it("renders all lines", () => {
    const lines: OutputLine[] = [
      { text: "Welcome.", lineType: "narration" },
      { text: "You see a door.", lineType: "narration" },
    ];
    render(<TerminalOutput lines={lines} />);
    expect(screen.getByText("Welcome.")).toBeInTheDocument();
    expect(screen.getByText("You see a door.")).toBeInTheDocument();
  });

  it("handles streaming (same-length update)", () => {
    const lines1: OutputLine[] = [{ text: "Hel", lineType: "narration" }];
    const { rerender } = render(<TerminalOutput lines={lines1} />);

    expect(screen.getByText("Hel")).toBeInTheDocument();

    // Same length array, mutated last entry = streaming append
    const lines2: OutputLine[] = [{ text: "Hello World", lineType: "narration" }];
    rerender(<TerminalOutput lines={lines2} />);

    expect(screen.getByText("Hello World")).toBeInTheDocument();
  });

  it("handles reset (shorter array)", () => {
    const lines1: OutputLine[] = [
      { text: "Line 1", lineType: "narration" },
      { text: "Line 2", lineType: "narration" },
      { text: "Line 3", lineType: "narration" },
    ];
    const { rerender } = render(<TerminalOutput lines={lines1} />);
    expect(screen.getByText("Line 3")).toBeInTheDocument();

    // New game — fewer lines
    const lines2: OutputLine[] = [{ text: "New adventure.", lineType: "narration" }];
    rerender(<TerminalOutput lines={lines2} />);

    expect(screen.getByText("New adventure.")).toBeInTheDocument();
    expect(screen.queryByText("Line 3")).not.toBeInTheDocument();
  });

  it("calls scrollIntoView on new lines", () => {
    const lines1: OutputLine[] = [{ text: "First", lineType: "narration" }];
    const { rerender } = render(<TerminalOutput lines={lines1} />);

    const lines2: OutputLine[] = [
      { text: "First", lineType: "narration" },
      { text: "Second", lineType: "narration" },
    ];
    rerender(<TerminalOutput lines={lines2} />);

    expect(Element.prototype.scrollIntoView).toHaveBeenCalled();
  });
});
