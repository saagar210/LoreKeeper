import type { LineType, OutputLine as OutputLineType } from "../../store/types";

const lineStyles: Record<LineType, string> = {
  narration: "text-[var(--text)]",
  system: "text-[var(--system)] italic",
  error: "text-[var(--error)]",
  playerInput: "text-[var(--input)] font-bold",
  combat: "text-[var(--combat)] font-bold",
  dialogue: "text-[var(--dialogue)]",
};

export function OutputLine({ line }: { line: OutputLineType }) {
  if (!line.text) return <div className="h-2" />;
  const isHint = line.text.startsWith("[Hint]");
  const style = lineStyles[line.lineType] ?? "text-[var(--text)]";
  return (
    <div
      className={`whitespace-pre-wrap break-words leading-relaxed ${style} ${isHint ? "opacity-60 italic" : ""}`}
    >
      {line.text}
    </div>
  );
}
