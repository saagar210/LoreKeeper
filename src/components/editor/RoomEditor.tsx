import type { EditorRoom, Item, Mood, Npc } from "../../store/types";

const moods: Mood[] = ["peaceful", "tense", "mysterious", "dark", "sacred", "dangerous"];

interface Props {
  room: EditorRoom;
  items: Record<string, Item>;
  npcs: Record<string, Npc>;
  onUpdate: (room: EditorRoom) => void;
  onAddItem: () => void;
  onRemoveItem: (itemId: string) => void;
  onAddNpc: () => void;
  onRemoveNpc: (npcId: string) => void;
}

export function RoomEditor({
  room,
  items,
  npcs,
  onUpdate,
  onAddItem,
  onRemoveItem,
  onAddNpc,
  onRemoveNpc,
}: Props) {
  return (
    <div className="flex flex-col gap-3 border-l border-[var(--border)] bg-[var(--panel-bg)] p-4 w-72 overflow-y-auto">
      <h3 className="text-sm font-bold text-[var(--accent)]">Room Properties</h3>

      <label className="flex flex-col gap-1 text-xs text-[var(--text-dim)]">
        ID
        <input
          type="text"
          value={room.id}
          onChange={(e) => onUpdate({ ...room, id: e.target.value })}
          className="border border-[var(--border)] bg-[var(--bg)] px-2 py-1 text-xs text-[var(--text)]"
        />
      </label>

      <label className="flex flex-col gap-1 text-xs text-[var(--text-dim)]">
        Name
        <input
          type="text"
          value={room.name}
          onChange={(e) => onUpdate({ ...room, name: e.target.value })}
          className="border border-[var(--border)] bg-[var(--bg)] px-2 py-1 text-xs text-[var(--text)]"
        />
      </label>

      <label className="flex flex-col gap-1 text-xs text-[var(--text-dim)]">
        Description
        <textarea
          value={room.description}
          onChange={(e) => onUpdate({ ...room, description: e.target.value })}
          rows={3}
          className="border border-[var(--border)] bg-[var(--bg)] px-2 py-1 text-xs text-[var(--text)] resize-y"
        />
      </label>

      <label className="flex flex-col gap-1 text-xs text-[var(--text-dim)]">
        Mood
        <select
          value={room.mood}
          onChange={(e) => onUpdate({ ...room, mood: e.target.value as Mood })}
          className="border border-[var(--border)] bg-[var(--bg)] px-2 py-1 text-xs text-[var(--text)]"
        >
          {moods.map((m) => (
            <option key={m} value={m}>
              {m}
            </option>
          ))}
        </select>
      </label>

      <label className="flex flex-col gap-1 text-xs text-[var(--text-dim)]">
        Examine Details
        <textarea
          value={room.examineDetails ?? ""}
          onChange={(e) =>
            onUpdate({
              ...room,
              examineDetails: e.target.value || null,
            })
          }
          rows={2}
          className="border border-[var(--border)] bg-[var(--bg)] px-2 py-1 text-xs text-[var(--text)] resize-y"
        />
      </label>

      <div className="flex flex-col gap-1">
        <div className="flex items-center justify-between">
          <span className="text-xs font-bold text-[var(--text-dim)]">Items</span>
          <button
            onClick={onAddItem}
            className="text-xs text-[var(--accent)] hover:underline"
          >
            + Add
          </button>
        </div>
        {room.items.length === 0 ? (
          <span className="text-xs text-[var(--text-dim)] opacity-50">No items</span>
        ) : (
          room.items.map((itemId) => (
            <div
              key={itemId}
              className="flex items-center justify-between border border-[var(--border)] px-2 py-1 text-xs"
            >
              <span className="text-[var(--text)]">
                {items[itemId]?.name ?? itemId}
              </span>
              <button
                onClick={() => onRemoveItem(itemId)}
                className="text-red-400 hover:text-red-300"
              >
                x
              </button>
            </div>
          ))
        )}
      </div>

      <div className="flex flex-col gap-1">
        <div className="flex items-center justify-between">
          <span className="text-xs font-bold text-[var(--text-dim)]">NPCs</span>
          <button
            onClick={onAddNpc}
            className="text-xs text-[var(--accent)] hover:underline"
          >
            + Add
          </button>
        </div>
        {room.npcs.length === 0 ? (
          <span className="text-xs text-[var(--text-dim)] opacity-50">No NPCs</span>
        ) : (
          room.npcs.map((npcId) => (
            <div
              key={npcId}
              className="flex items-center justify-between border border-[var(--border)] px-2 py-1 text-xs"
            >
              <span className="text-[var(--text)]">
                {npcs[npcId]?.name ?? npcId}
              </span>
              <button
                onClick={() => onRemoveNpc(npcId)}
                className="text-red-400 hover:text-red-300"
              >
                x
              </button>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
