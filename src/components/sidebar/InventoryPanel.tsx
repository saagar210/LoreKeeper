import type { Item, Player } from "../../store/types";

interface Props {
  player: Player;
  items: Record<string, Item>;
}

export function InventoryPanel({ player, items }: Props) {
  return (
    <div>
      <h3 className="mb-2 font-bold text-[var(--accent)]">
        Inventory ({player.inventory.length}/{player.maxInventory})
      </h3>
      {player.inventory.length === 0 ? (
        <p className="text-[var(--text-dim)] text-xs">Empty</p>
      ) : (
        <ul className="space-y-1 text-xs">
          {player.inventory.map((id) => {
            const item = items[id];
            if (!item) return null;
            const equipped =
              player.equippedWeapon === id
                ? " [W]"
                : player.equippedArmor === id
                  ? " [A]"
                  : "";
            return (
              <li key={id} className="text-[var(--text)]">
                {item.name}
                {equipped && (
                  <span className="text-[var(--accent)]">{equipped}</span>
                )}
              </li>
            );
          })}
        </ul>
      )}
    </div>
  );
}
