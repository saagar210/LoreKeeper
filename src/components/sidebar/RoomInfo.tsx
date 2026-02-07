import type { Item, Location, Npc } from "../../store/types";

interface Props {
  location: Location;
  items: Record<string, Item>;
  npcs: Record<string, Npc>;
}

export function RoomInfo({ location, items, npcs }: Props) {
  const exitList = Object.keys(location.exits).map((dir) => {
    const isLocked = dir in location.lockedExits;
    return isLocked ? `${dir} (locked)` : dir;
  });

  const npcNames = location.npcs
    .map((id) => npcs[id]?.name)
    .filter(Boolean);

  const itemNames = location.items
    .map((id) => items[id]?.name)
    .filter(Boolean);

  return (
    <div>
      <h3 className="mb-2 font-bold text-[var(--accent)]">{location.name}</h3>
      <div className="space-y-1 text-xs">
        {exitList.length > 0 && (
          <div className="text-[var(--text)]">
            <span className="text-[var(--text-dim)]">Exits: </span>
            {exitList.join(", ")}
          </div>
        )}
        {npcNames.length > 0 && (
          <div className="text-[var(--dialogue)]">
            <span className="text-[var(--text-dim)]">NPCs: </span>
            {npcNames.join(", ")}
          </div>
        )}
        {itemNames.length > 0 && (
          <div className="text-[var(--text)]">
            <span className="text-[var(--text-dim)]">Items: </span>
            {itemNames.join(", ")}
          </div>
        )}
      </div>
    </div>
  );
}
