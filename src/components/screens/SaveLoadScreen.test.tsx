import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { createSaveSlot, mockInvoke } from "../../test/mocks";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

const { SaveLoadScreen } = await import("./SaveLoadScreen");

describe("SaveLoadScreen", () => {
  it("has role=dialog", async () => {
    mockInvoke.mockResolvedValueOnce([]);
    render(<SaveLoadScreen mode="load" onClose={vi.fn()} />);
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
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
    const onLoad = vi.fn().mockResolvedValue({ ok: true, slotName: "mysave" });
    const onClose = vi.fn();
    mockInvoke.mockResolvedValueOnce([createSaveSlot({ slotName: "mysave" })]);

    render(<SaveLoadScreen mode="load" onLoad={onLoad} onClose={onClose} />);

    await waitFor(() => {
      expect(screen.getByText("mysave")).toBeInTheDocument();
    });

    await user.click(screen.getByText("Load"));

    expect(onLoad).toHaveBeenCalledWith("mysave");
    await waitFor(() => expect(onClose).toHaveBeenCalled());
  });

  it("requires confirmation before deleting", async () => {
    const user = userEvent.setup();
    const saves = [createSaveSlot({ slotName: "old_save" })];
    mockInvoke.mockResolvedValueOnce(saves); // list_saves

    render(<SaveLoadScreen mode="load" onClose={vi.fn()} />);

    await waitFor(() => {
      expect(screen.getByText("old_save")).toBeInTheDocument();
    });

    // First click shows confirmation
    await user.click(screen.getByText("Delete"));
    expect(screen.getByText("Confirm?")).toBeInTheDocument();

    // Second click actually deletes
    mockInvoke.mockResolvedValueOnce(undefined); // delete_save
    mockInvoke.mockResolvedValueOnce([]); // refresh after delete
    await user.click(screen.getByText("Confirm?"));

    expect(mockInvoke).toHaveBeenCalledWith("delete_save", {
      slotName: "old_save",
    });
  });

  it("shows save input in save mode", async () => {
    mockInvoke.mockResolvedValueOnce([]);
    render(<SaveLoadScreen mode="save" onClose={vi.fn()} />);
    await waitFor(() => expect(mockInvoke).toHaveBeenCalled());
    expect(screen.getByPlaceholderText("Save name...")).toBeInTheDocument();
  });

  it("calls onSave when Save button clicked", async () => {
    const user = userEvent.setup();
    const onSave = vi.fn().mockResolvedValue({ ok: true, slotName: "my_save" });
    const onClose = vi.fn();
    mockInvoke.mockResolvedValueOnce([]);

    render(<SaveLoadScreen mode="save" onSave={onSave} onClose={onClose} />);

    const input = screen.getByPlaceholderText("Save name...");
    await user.type(input, "my_save");
    await user.click(screen.getByText("Save"));

    expect(onSave).toHaveBeenCalledWith("my_save");
    await waitFor(() => expect(onClose).toHaveBeenCalled());
  });

  it("shows inline validation errors for invalid save names", async () => {
    const user = userEvent.setup();
    const onSave = vi.fn();
    mockInvoke.mockResolvedValueOnce([]);

    render(<SaveLoadScreen mode="save" onSave={onSave} onClose={vi.fn()} />);

    await user.type(screen.getByPlaceholderText("Save name..."), "bad/save");
    await user.click(screen.getByText("Save"));

    expect(
      screen.getByText(
        "Save name can only use letters, numbers, spaces, '-' and '_'.",
      ),
    ).toBeInTheDocument();
    expect(onSave).not.toHaveBeenCalled();
  });

  it("keeps the dialog open when load fails", async () => {
    const user = userEvent.setup();
    const onLoad = vi.fn().mockResolvedValue({
      ok: false,
      message: "Save data is corrupted and could not be loaded.",
    });
    const onClose = vi.fn();
    mockInvoke.mockResolvedValueOnce([
      createSaveSlot({ slotName: "broken_save" }),
    ]);

    render(<SaveLoadScreen mode="load" onLoad={onLoad} onClose={onClose} />);

    await waitFor(() => {
      expect(screen.getByText("broken_save")).toBeInTheDocument();
    });

    await user.click(screen.getByText("Load"));

    expect(
      screen.getByText("Save data is corrupted and could not be loaded."),
    ).toBeInTheDocument();
    expect(onClose).not.toHaveBeenCalled();
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
