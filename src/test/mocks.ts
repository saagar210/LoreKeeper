import { vi } from "vitest";
import type {
  CommandResponse,
  Direction,
  GameSettings,
  Item,
  Location,
  Npc,
  OutputLine,
  Player,
  Quest,
  SaveSlotInfo,
  WorldState,
} from "../store/types";

// Shared mock references for Tauri IPC
export const mockInvoke = vi.fn();
export const mockListen = vi.fn();
export const mockUnlisten = vi.fn();

// Factory functions

export function createPlayer(overrides?: Partial<Player>): Player {
  return {
    location: "entrance_hall",
    inventory: [],
    maxInventory: 10,
    health: 100,
    maxHealth: 100,
    attack: 10,
    defense: 5,
    equippedWeapon: null,
    equippedArmor: null,
    questFlags: {},
    visitedLocations: ["entrance_hall"],
    turnsElapsed: 0,
    ...overrides,
  };
}

export function createLocation(overrides?: Partial<Location>): Location {
  return {
    id: "entrance_hall",
    name: "Entrance Hall",
    description: "A grand entrance hall.",
    items: [],
    npcs: [],
    exits: {} as Record<Direction, string>,
    lockedExits: {} as Record<Direction, string>,
    visited: true,
    discoveredSecrets: [],
    ambientMood: "mysterious",
    ...overrides,
  };
}

export function createItem(overrides?: Partial<Item>): Item {
  return {
    id: "rusty_sword",
    name: "Rusty Sword",
    description: "A worn blade.",
    itemType: "weapon",
    modifier: { attack: 3, defense: 0, health: 0 },
    usable: true,
    consumable: false,
    keyId: null,
    ...overrides,
  };
}

export function createNpc(overrides?: Partial<Npc>): Npc {
  return {
    id: "old_sage",
    name: "Old Sage",
    description: "A wizened figure.",
    personalitySeed: "wise",
    dialogueState: "greeting",
    hostile: false,
    health: 50,
    maxHealth: 50,
    attack: 5,
    defense: 3,
    items: [],
    questGiver: null,
    ...overrides,
  };
}

export function createQuest(overrides?: Partial<Quest>): Quest {
  return {
    id: "find_amulet",
    name: "Find the Amulet",
    description: "Locate the lost amulet.",
    giver: "old_sage",
    objective: { fetchItem: "amulet" },
    reward: ["gold_coin"],
    completed: false,
    active: true,
    ...overrides,
  };
}

export function createWorldState(overrides?: Partial<WorldState>): WorldState {
  const player = createPlayer(overrides?.player);
  const location = createLocation();
  return {
    player,
    locations: { [location.id]: location },
    items: {},
    npcs: {},
    quests: {},
    events: [],
    gameMode: "exploring",
    combatState: null,
    messageLog: [],
    combatLog: [],
    lastNarrativeContext: null,
    initialized: true,
    ...overrides,
    // Ensure nested overrides merge correctly
    ...(overrides?.player ? { player: { ...player, ...overrides.player } } : {}),
  };
}

export function createCommandResponse(
  overrides?: Partial<CommandResponse>,
): CommandResponse {
  return {
    messages: [{ text: "Welcome to Thornhold.", lineType: "narration" }],
    worldState: createWorldState(),
    ...overrides,
  };
}

export function createSaveSlot(overrides?: Partial<SaveSlotInfo>): SaveSlotInfo {
  return {
    slotName: "save1",
    playerLocation: "Entrance Hall",
    playerHealth: 100,
    turnsElapsed: 5,
    questsCompleted: 0,
    savedAt: "2025-01-01T00:00:00Z",
    ...overrides,
  };
}

export function createSettings(overrides?: Partial<GameSettings>): GameSettings {
  return {
    ollamaEnabled: false,
    ollamaModel: "llama3.2",
    ollamaUrl: "http://localhost:11434",
    temperature: 0.7,
    narratorTone: "atmospheric",
    typewriterSpeed: 30,
    theme: "greenTerminal",
    narrationVerbosity: "normal",
    ...overrides,
  };
}

export function createOutputLines(count: number): OutputLine[] {
  return Array.from({ length: count }, (_, i) => ({
    text: `Line ${i + 1}`,
    lineType: "narration" as const,
  }));
}
