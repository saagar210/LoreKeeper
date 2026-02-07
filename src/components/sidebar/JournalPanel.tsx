import type { JournalCategory, JournalEntry } from "../../store/types";

interface Props {
  journal: JournalEntry[];
}

const categoryLabels: Record<JournalCategory, string> = {
  location: "Locations",
  bestiary: "Bestiary",
  item: "Items",
  lore: "Lore",
};

const categoryOrder: JournalCategory[] = ["location", "bestiary", "item", "lore"];

export function JournalPanel({ journal }: Props) {
  if (journal.length === 0) {
    return (
      <div className="p-3 text-xs text-[var(--text-dim)]">
        No entries yet. Explore and examine to discover lore.
      </div>
    );
  }

  return (
    <div className="p-3 space-y-3">
      {categoryOrder.map((cat) => {
        const entries = journal.filter((e) => e.category === cat);
        if (entries.length === 0) return null;
        return (
          <div key={cat}>
            <h4 className="text-xs font-bold text-[var(--accent)] mb-1">{categoryLabels[cat]}</h4>
            {entries.map((entry) => (
              <div key={entry.id} className="text-xs text-[var(--text)] ml-2 mb-1">
                <span className="text-[var(--text-dim)]">{entry.title}:</span> {entry.content}
              </div>
            ))}
          </div>
        );
      })}
    </div>
  );
}
