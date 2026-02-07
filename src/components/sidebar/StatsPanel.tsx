import type { Item, Player } from "../../store/types";

interface Props {
  player: Player;
  items: Record<string, Item>;
}

export function StatsPanel({ player, items }: Props) {
  const hpPct = Math.round((player.health / player.maxHealth) * 100);
  const hpColor =
    hpPct > 60 ? "var(--hp-high)" : hpPct > 25 ? "var(--hp-mid)" : "var(--hp-low)";

  const weaponBonus =
    player.equippedWeapon && items[player.equippedWeapon]?.modifier?.attack
      ? items[player.equippedWeapon].modifier!.attack
      : 0;
  const armorBonus =
    player.equippedArmor && items[player.equippedArmor]?.modifier?.defense
      ? items[player.equippedArmor].modifier!.defense
      : 0;

  return (
    <div>
      <h3 className="mb-2 font-bold text-[var(--accent)]">Stats</h3>
      <div className="space-y-2 text-xs">
        <div>
          <div className="flex justify-between text-[var(--text)]">
            <span>HP</span>
            <span>
              {player.health}/{player.maxHealth}
            </span>
          </div>
          <div
            className="mt-1 h-2 w-full rounded bg-[var(--border)]"
            role="progressbar"
            aria-valuenow={player.health}
            aria-valuemin={0}
            aria-valuemax={player.maxHealth}
            aria-label="Health"
          >
            <div
              className="h-full rounded transition-all"
              style={{ width: `${hpPct}%`, backgroundColor: hpColor }}
            />
          </div>
        </div>
        <div className="flex justify-between text-[var(--text)]">
          <span>Attack</span>
          <span>
            {player.attack + weaponBonus}
            {weaponBonus > 0 && (
              <span className="text-[var(--accent)]"> (+{weaponBonus})</span>
            )}
          </span>
        </div>
        <div className="flex justify-between text-[var(--text)]">
          <span>Defense</span>
          <span>
            {player.defense + armorBonus}
            {armorBonus > 0 && (
              <span className="text-[var(--accent)]"> (+{armorBonus})</span>
            )}
          </span>
        </div>
        <div className="flex justify-between text-[var(--text-dim)]">
          <span>Turns</span>
          <span>{player.turnsElapsed}</span>
        </div>
      </div>
      {player.statusEffects.length > 0 && (
        <div className="mt-3 border-t border-[var(--border)] pt-2">
          <div className="text-xs text-[var(--text-dim)] mb-1">Status Effects:</div>
          {player.statusEffects.map((effect, i) => (
            <div key={i} className="text-xs text-[var(--text)]">
              {effect.name} ({effect.turnsRemaining} turns)
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
