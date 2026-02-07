import type { Quest } from "../../store/types";

interface Props {
  quests: Record<string, Quest>;
}

export function QuestLog({ quests }: Props) {
  const active = Object.values(quests).filter((q) => q.active && !q.completed);
  const completed = Object.values(quests).filter((q) => q.completed);

  return (
    <div>
      <h3 className="mb-2 font-bold text-[var(--accent)]">Quests</h3>
      {active.length === 0 && completed.length === 0 ? (
        <p className="text-[var(--text-dim)] text-xs">No quests yet</p>
      ) : (
        <div className="space-y-2 text-xs">
          {active.map((q) => (
            <div key={q.id} className="text-[var(--text)]">
              <span className="font-bold">{q.name}</span>
              <p className="text-[var(--text-dim)]">{q.description}</p>
            </div>
          ))}
          {completed.map((q) => (
            <div key={q.id} className="text-[var(--text-dim)] line-through">
              {q.name}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
