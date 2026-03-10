const SAVE_SLOT_MAX_LEN = 32;
const THEME_NAME_MAX_LEN = 40;
const SAFE_LABEL_PATTERN = /^[A-Za-z0-9 _-]+$/;

function normalizeUserLabel(
  value: string,
  fieldName: string,
  maxLen: number,
): { ok: true; value: string } | { ok: false; message: string } {
  const trimmed = value.trim();
  if (!trimmed) {
    return { ok: false, message: `${fieldName} cannot be empty.` };
  }
  if (trimmed.length > maxLen) {
    return {
      ok: false,
      message: `${fieldName} must be ${maxLen} characters or fewer.`,
    };
  }
  if ([...trimmed].some((char) => /[\u0000-\u001F\u007F]/.test(char))) {
    return {
      ok: false,
      message: `${fieldName} cannot contain control characters.`,
    };
  }
  if (!SAFE_LABEL_PATTERN.test(trimmed)) {
    return {
      ok: false,
      message: `${fieldName} can only use letters, numbers, spaces, '-' and '_'.`,
    };
  }

  return { ok: true, value: trimmed };
}

export function normalizeSaveSlotNameInput(value: string) {
  return normalizeUserLabel(value, "Save name", SAVE_SLOT_MAX_LEN);
}

export function normalizeThemeNameInput(value: string) {
  return normalizeUserLabel(value, "Theme name", THEME_NAME_MAX_LEN);
}
