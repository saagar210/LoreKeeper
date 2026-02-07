import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { createSaveSlot, mockInvoke } from "../../test/mocks";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

const { SaveLoadScreen } = await import("./SaveLoadScreen");

describe("SaveLoadScreen", () => {
  it("has role=dialog", () => {
    mockInvoke.mockResolvedValueOnce([]);
    render(<SaveLoadScreen mode="load" onClose={vi.fn()} />);
    expect(screen.getByRole("dialog")).toBeInTheDocument();
  });

  it("lists saves in load mode", async () => {
    const saves = [
      createSaveSlot({ slotName: "save1" }),
      createSaveSlot({ slotName: "save2" }),
    ];
    mockInvoke.mockResolvedValueOnce(saves);

    render(<SaveLoadScreen mode="load" onClose={vi.fn()} />);

    await waitFor(() => {
      expect(screen.getByText("save1")).toBeInTheDocument();
      expect(screen.getByText("save2")).toBeInTheDocument();
    });
  });

  it("calls onLoad and onClose when Load clicked", async () => {
    const user = userEvent.setup();
    const onLoad = vi.fn();
    const onClose = vi.fn();
    mockInvoke.mockResolvedValueOnce([createSaveSlot({ slotName: "mysave" })]);

    render(
      <SaveLoadScreen mode="load" onLoad={onLoad} onClose={onClose} />,
    );

    await waitFor(() => {
      expect(screen.getByText("mysave")).toBeInTheDocument();
    });

    await user.click(screen.getByText("Load"));

    expect(onLoad).toHaveBeenCalledWith("mysave");
    expect(onClose).toHaveBeenCalled();
  });

  it("calls handleDelete when Delete clicked", async () => {
    const user = userEvent.setup();
    const saves = [createSaveSlot({ slotName: "old_save" })];
    mockInvoke.mockResolvedValueOnce(saves); // list_saves
    mockInvoke.mockResolvedValueOnce(undefined); // delete_save
    mockInvoke.mockResolvedValueOnce([]); // refresh after delete

    render(<SaveLoadScreen mode="load" onClose={vi.fn()} />);

    await waitFor(() => {
      expect(screen.getByText("old_save")).toBeInTheDocument();
    });

    await user.click(screen.getByText("Delete"));

    expect(mockInvoke).toHaveBeenCalledWith("delete_save", { slotName: "old_save" });
  });

  it("shows save input in save mode", async () => {
    mockInvoke.mockResolvedValueOnce([]);
    render(<SaveLoadScreen mode="save" onClose={vi.fn()} />);
    expect(screen.getByPlaceholderText("Save name...")).toBeInTheDocument();
  });

  it("calls onSave when Save button clicked", async () => {
    const user = userEvent.setup();
    const onSave = vi.fn();
    const onClose = vi.fn();
    mockInvoke.mockResolvedValueOnce([]);

    render(
      <SaveLoadScreen mode="save" onSave={onSave} onClose={onClose} />,
    );

    const input = screen.getByPlaceholderText("Save name...");
    await user.type(input, "my_save");
    await user.click(screen.getByText("Save"));

    expect(onSave).toHaveBeenCalledWith("my_save");
    expect(onClose).toHaveBeenCalled();
  });

  it("closes on backdrop click", async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    mockInvoke.mockResolvedValueOnce([]);

    render(<SaveLoadScreen mode="load" onClose={onClose} />);
    const backdrop = screen.getByRole("dialog");

    await user.click(backdrop);
    expect(onClose).toHaveBeenCalled();
  });

  it("shows no saves message when empty", async () => {
    mockInvoke.mockResolvedValueOnce([]);
    render(<SaveLoadScreen mode="load" onClose={vi.fn()} />);

    await waitFor(() => {
      expect(screen.getByText("No saves found.")).toBeInTheDocument();
    });
  });
});
