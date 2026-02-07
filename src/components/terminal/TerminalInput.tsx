import { useCallback, useRef, useState } from "react";
import { useAutocomplete } from "../../hooks/useAutocomplete";
import { useCommandHistory } from "../../hooks/useCommandHistory";

interface Props {
  onSubmit: (input: string) => void;
  disabled?: boolean;
}

export function TerminalInput({ onSubmit, disabled }: Props) {
  const [value, setValue] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);
  const { addCommand, getPrevious, getNext } = useCommandHistory();
  const {
    completions,
    selectedIndex,
    fetchCompletions,
    selectNext,
    selectPrev,
    accept,
    dismiss,
  } = useAutocomplete();

  const handleSubmit = useCallback(() => {
    if (!value.trim() || disabled) return;
    addCommand(value);
    onSubmit(value);
    setValue("");
    dismiss();
  }, [value, disabled, addCommand, onSubmit, dismiss]);

  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const newVal = e.target.value;
      setValue(newVal);
      fetchCompletions(newVal);
    },
    [fetchCompletions],
  );

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === "Enter") {
        e.preventDefault();
        handleSubmit();
      } else if (e.key === "Tab") {
        e.preventDefault();
        if (completions.length > 0) {
          const accepted = accept();
          if (accepted) {
            setValue(accepted);
            fetchCompletions(accepted);
          }
        }
      } else if (e.key === "Escape") {
        if (completions.length > 0) {
          e.preventDefault();
          dismiss();
        }
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        if (completions.length > 0) {
          selectPrev();
        } else {
          const prev = getPrevious();
          if (prev !== null) setValue(prev);
        }
      } else if (e.key === "ArrowDown") {
        e.preventDefault();
        if (completions.length > 0) {
          selectNext();
        } else {
          const next = getNext();
          if (next !== null) setValue(next);
        }
      }
    },
    [
      handleSubmit,
      completions,
      accept,
      dismiss,
      selectPrev,
      selectNext,
      getPrevious,
      getNext,
      fetchCompletions,
    ],
  );

  const showDropdown = completions.length > 0 && !disabled;

  return (
    <div className="relative border-t border-[var(--border)] p-3 font-mono text-sm">
      <div className="flex items-center gap-2">
        <span className="text-[var(--accent)] font-bold">&gt;</span>
        <input
          ref={inputRef}
          type="text"
          value={value}
          onChange={handleChange}
          onKeyDown={handleKeyDown}
          disabled={disabled}
          className="flex-1 bg-transparent text-[var(--text)] outline-none placeholder:text-[var(--text-dim)]"
          placeholder={disabled ? "..." : "Type a command..."}
          aria-label="Game command input"
          aria-autocomplete="list"
          aria-expanded={showDropdown}
          aria-controls={showDropdown ? "autocomplete-list" : undefined}
          aria-activedescendant={
            showDropdown && selectedIndex >= 0
              ? `autocomplete-${selectedIndex}`
              : undefined
          }
          autoFocus
        />
      </div>
      {showDropdown && (
        <ul
          id="autocomplete-list"
          role="listbox"
          className="absolute bottom-full left-0 right-0 border border-[var(--border)] bg-[var(--panel-bg)] text-xs max-h-40 overflow-y-auto"
        >
          {completions.map((completion, i) => (
            <li
              key={completion}
              id={`autocomplete-${i}`}
              role="option"
              aria-selected={i === selectedIndex}
              className={`px-3 py-1 cursor-pointer ${
                i === selectedIndex
                  ? "bg-[var(--accent)] text-[var(--bg)]"
                  : "text-[var(--text)] hover:bg-[var(--border)]"
              }`}
              onMouseDown={(e) => {
                e.preventDefault();
                setValue(completion);
                dismiss();
                inputRef.current?.focus();
              }}
            >
              {completion}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
