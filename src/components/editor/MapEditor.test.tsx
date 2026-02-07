import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { MapEditor } from "./MapEditor";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue({ valid: true, errors: [], warnings: [] }),
}));

describe("MapEditor", () => {
  it("renders with dialog role", () => {
    render(<MapEditor onClose={() => {}} />);
    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });

  it("shows Map Editor heading", () => {
    render(<MapEditor onClose={() => {}} />);
    expect(screen.getByText("Map Editor")).toBeInTheDocument();
  });

  it("shows toolbar buttons", () => {
    render(<MapEditor onClose={() => {}} />);
    expect(screen.getByText("Select")).toBeInTheDocument();
    expect(screen.getByText("Add Room")).toBeInTheDocument();
    expect(screen.getByText("Connect")).toBeInTheDocument();
    expect(screen.getByText("Delete")).toBeInTheDocument();
    expect(screen.getByText("Validate")).toBeInTheDocument();
    expect(screen.getByText("Export")).toBeInTheDocument();
    expect(screen.getByText("Clear")).toBeInTheDocument();
  });

  it("shows room count", () => {
    render(<MapEditor onClose={() => {}} />);
    expect(screen.getByText("0 rooms")).toBeInTheDocument();
  });

  it("calls onClose when X button clicked", async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(<MapEditor onClose={onClose} />);
    await user.click(screen.getByText("[X]"));
    expect(onClose).toHaveBeenCalledOnce();
  });

  it("calls onClose when backdrop clicked", async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(<MapEditor onClose={onClose} />);
    const dialog = screen.getByRole("dialog");
    await user.click(dialog);
    expect(onClose).toHaveBeenCalledOnce();
  });

  it("shows empty state message in SVG", () => {
    render(<MapEditor onClose={() => {}} />);
    expect(
      screen.getByText(/click to place rooms/),
    ).toBeInTheDocument();
  });
});
