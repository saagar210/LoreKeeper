import { useMemo } from "react";
import type { CraftingRecipe, Item, Player } from "../../store/types";

interface Props {
  player: Player;
  items: Record<string, Item>;
  recipes?: CraftingRecipe[];
}

export function InventoryPanel({ player, items, recipes = [] }: Props) {
  const hasCraftable = useMemo(() => {
    const inventorySet = new Set(player.inventory);
    return recipes.some(
      (recipe) =>
        !recipe.discovered &&
        recipe.inputs.every((input) => inventorySet.has(input)),
    );
  }, [player.inventory, recipes]);

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
      {hasCraftable && (
        <p className="mt-2 text-[var(--accent)] text-xs animate-pulse">
          Craftable items available!
        </p>
      )}
    </div>
  );
}
