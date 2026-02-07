import type { WorldState } from "../../store/types";
import { CombatLog } from "./CombatLog";
import { InventoryPanel } from "./InventoryPanel";
import { MiniMap } from "./MiniMap";
import { QuestLog } from "./QuestLog";
import { RoomInfo } from "./RoomInfo";
import { StatsPanel } from "./StatsPanel";

interface Props {
  worldState: WorldState;
  onClose?: () => void;
}

export function SidePanel({ worldState, onClose }: Props) {
  const currentLocation = worldState.locations[worldState.player.location];

  return (
    <div
      className="flex h-full w-72 flex-col gap-4 overflow-y-auto border-l border-[var(--border)] bg-[var(--panel-bg)] p-4 font-mono text-sm"
      role="complementary"
      aria-label="Game information"
    >
      {onClose && (
        <div className="flex justify-end">
          <button
            onClick={onClose}
            className="text-xs text-[var(--text-dim)] hover:text-[var(--text)]"
          >
            [close]
          </button>
        </div>
      )}
      {currentLocation && (
        <RoomInfo
          location={currentLocation}
          items={worldState.items}
          npcs={worldState.npcs}
        />
      )}
      <div className="border-t border-[var(--border)] pt-3">
        <StatsPanel player={worldState.player} items={worldState.items} />
      </div>
      <div className="border-t border-[var(--border)] pt-3">
        <InventoryPanel player={worldState.player} items={worldState.items} />
      </div>
      <div className="border-t border-[var(--border)] pt-3">
        <QuestLog quests={worldState.quests} />
      </div>
      {worldState.combatLog.length > 0 && (
        <div className="border-t border-[var(--border)] pt-3">
          <CombatLog combatLog={worldState.combatLog} />
        </div>
      )}
      <div className="border-t border-[var(--border)] pt-3">
        <MiniMap locations={worldState.locations} player={worldState.player} />
      </div>
    </div>
  );
}
