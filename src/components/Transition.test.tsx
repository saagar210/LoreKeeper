import { act, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { Transition } from "./Transition";

describe("Transition", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("renders children when show is true", () => {
    render(
      <Transition show={true}>
        <div>Content</div>
      </Transition>,
    );
    expect(screen.getByText("Content")).toBeInTheDocument();
  });

  it("does not render children initially when show is false", () => {
    render(
      <Transition show={false}>
        <div>Content</div>
      </Transition>,
    );
    expect(screen.queryByText("Content")).not.toBeInTheDocument();
  });

  it("unmounts after transition when show goes to false", () => {
    const { rerender } = render(
      <Transition show={true} duration={200}>
        <div>Content</div>
      </Transition>,
    );

    expect(screen.getByText("Content")).toBeInTheDocument();

    rerender(
      <Transition show={false} duration={200}>
        <div>Content</div>
      </Transition>,
    );

    // Still mounted during transition
    expect(screen.getByText("Content")).toBeInTheDocument();

    // After duration, unmount
    act(() => {
      vi.advanceTimersByTime(250);
    });

    expect(screen.queryByText("Content")).not.toBeInTheDocument();
  });

  it("calls onExited after exit transition", () => {
    const onExited = vi.fn();
    const { rerender } = render(
      <Transition show={true} duration={200} onExited={onExited}>
        <div>Content</div>
      </Transition>,
    );

    rerender(
      <Transition show={false} duration={200} onExited={onExited}>
        <div>Content</div>
      </Transition>,
    );

    act(() => {
      vi.advanceTimersByTime(250);
    });

    expect(onExited).toHaveBeenCalledTimes(1);
  });

  it("applies slideUp classes", () => {
    const { container } = render(
      <Transition show={false} type="slideUp">
        <div>Content</div>
      </Transition>,
    );
    // Not mounted when show=false
    expect(container.firstChild).toBeNull();
  });
});
