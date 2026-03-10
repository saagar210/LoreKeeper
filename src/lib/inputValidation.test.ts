import { describe, expect, it } from "vitest";
import {
  normalizeSaveSlotNameInput,
  normalizeThemeNameInput,
} from "./inputValidation";

describe("inputValidation", () => {
  it("normalizes valid save slot names", () => {
    expect(normalizeSaveSlotNameInput("  My Save  ")).toEqual({
      ok: true,
      value: "My Save",
    });
  });

  it("rejects invalid save slot characters", () => {
    expect(normalizeSaveSlotNameInput("bad/save")).toEqual({
      ok: false,
      message: "Save name can only use letters, numbers, spaces, '-' and '_'.",
    });
  });

  it("normalizes valid theme names", () => {
    expect(normalizeThemeNameInput("  Amber Night  ")).toEqual({
      ok: true,
      value: "Amber Night",
    });
  });
});
