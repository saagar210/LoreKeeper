import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import type { LineType } from "../../store/types";
import { OutputLine } from "./OutputLine";

describe("OutputLine", () => {
  const lineTypes: LineType[] = [
    "narration",
    "system",
    "error",
    "playerInput",
    "combat",
    "dialogue",
  ];

  it.each(lineTypes)("renders with correct class for %s", (lineType) => {
    const { container } = render(
      <OutputLine line={{ text: "Test text", lineType }} />,
    );
    const div = container.firstChild as HTMLElement;
    expect(div.textContent).toBe("Test text");
    expect(div.className).toContain("whitespace-pre-wrap");
  });

  it("renders narration with text color", () => {
    const { container } = render(
      <OutputLine line={{ text: "A dark room.", lineType: "narration" }} />,
    );
    expect((container.firstChild as HTMLElement).className).toContain("text-[var(--text)]");
  });

  it("renders system with italic", () => {
    const { container } = render(
      <OutputLine line={{ text: "Game saved.", lineType: "system" }} />,
    );
    expect((container.firstChild as HTMLElement).className).toContain("italic");
  });

  it("renders error with error color", () => {
    const { container } = render(
      <OutputLine line={{ text: "Oops!", lineType: "error" }} />,
    );
    expect((container.firstChild as HTMLElement).className).toContain("text-[var(--error)]");
  });

  it("renders playerInput with bold", () => {
    const { container } = render(
      <OutputLine line={{ text: "> go north", lineType: "playerInput" }} />,
    );
    expect((container.firstChild as HTMLElement).className).toContain("font-bold");
  });

  it("renders combat with bold", () => {
    const { container } = render(
      <OutputLine line={{ text: "You strike!", lineType: "combat" }} />,
    );
    expect((container.firstChild as HTMLElement).className).toContain("font-bold");
    expect((container.firstChild as HTMLElement).className).toContain("text-[var(--combat)]");
  });

  it("renders spacer div for empty text", () => {
    const { container } = render(
      <OutputLine line={{ text: "", lineType: "narration" }} />,
    );
    const div = container.firstChild as HTMLElement;
    expect(div.className).toContain("h-2");
    expect(div.textContent).toBe("");
  });
});
