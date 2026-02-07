// TypeScript mirrors of Rust types (camelCase)

export type Direction = "north" | "south" | "east" | "west" | "up" | "down";
export type Mood = "peaceful" | "tense" | "mysterious" | "dark" | "sacred" | "dangerous";
export type ItemType = "weapon" | "armor" | "consumable" | "key" | "scroll" | "quest" | "miscellaneous";
export type DialogueState = "greeting" | "familiar" | "questOffered" | "questActive" | "questComplete" | "hostile" | "dead";
export type LineType = "narration" | "system" | "error" | "playerInput" | "combat" | "dialogue";
export type ThemeName = "greenTerminal" | "amberTerminal" | "parchment" | "darkModern";
export type EndingType = "victoryPeace" | "victoryCombat" | "death";

export type GameMode =
  | "exploring"
  | { inCombat: string }
  | { inDialogue: string }
  | { gameOver: EndingType };

export interface StatModifier {
  attack: number;
  defense: number;
  health: number;
}

export interface Item {
  id: string;
  name: string;
  description: string;
  itemType: ItemType;
  modifier: StatModifier | null;
  usable: boolean;
  consumable: boolean;
  keyId: string | null;
}

export interface Location {
  id: string;
  name: string;
  description: string;
  items: string[];
  npcs: string[];
  exits: Record<Direction, string>;
  lockedExits: Record<Direction, string>;
  visited: boolean;
  discoveredSecrets: string[];
  ambientMood: Mood;
}

export interface Player {
  location: string;
  inventory: string[];
  maxInventory: number;
  health: number;
  maxHealth: number;
  attack: number;
  defense: number;
  equippedWeapon: string | null;
  equippedArmor: string | null;
  questFlags: Record<string, boolean>;
  visitedLocations: string[];
  turnsElapsed: number;
}

export interface Npc {
  id: string;
  name: string;
  description: string;
  personalitySeed: string;
  dialogueState: DialogueState;
  hostile: boolean;
  health: number;
  maxHealth: number;
  attack: number;
  defense: number;
  items: string[];
  questGiver: string | null;
}

export interface QuestObjective {
  fetchItem?: string;
  killNpc?: string;
  reachLocation?: string;
}

export interface Quest {
  id: string;
  name: string;
  description: string;
  giver: string;
  objective: QuestObjective;
  reward: string[];
  completed: boolean;
  active: boolean;
}

export interface GameEvent {
  trigger: unknown;
  action: unknown;
  oneShot: boolean;
  fired: boolean;
  locationId: string;
}

export interface CombatState {
  enemyId: string;
  playerTurn: boolean;
  turnCount: number;
}

export interface WorldState {
  player: Player;
  locations: Record<string, Location>;
  items: Record<string, Item>;
  npcs: Record<string, Npc>;
  quests: Record<string, Quest>;
  events: GameEvent[];
  gameMode: GameMode;
  combatState: CombatState | null;
  messageLog: string[];
  combatLog: CombatLogEntry[];
  lastNarrativeContext: NarrativeContext | null;
  initialized: boolean;
}

export interface OutputLine {
  text: string;
  lineType: LineType;
}

export interface CommandResponse {
  messages: OutputLine[];
  worldState: WorldState;
}

export interface SaveSlotInfo {
  slotName: string;
  playerLocation: string | null;
  playerHealth: number | null;
  turnsElapsed: number | null;
  questsCompleted: number | null;
  savedAt: string;
}

export interface GameSettings {
  ollamaEnabled: boolean;
  ollamaModel: string;
  ollamaUrl: string;
  temperature: number;
  narratorTone: string;
  typewriterSpeed: number;
  theme: ThemeName;
  narrationVerbosity: string;
}

export interface OllamaStatus {
  connected: boolean;
  version: string | null;
}

export interface ModelInfo {
  name: string;
  size: number | null;
}

export type NarrativeEvent =
  | { type: "token"; text: string }
  | { type: "complete" }
  | { type: "fallback" };

export interface MapNode {
  id: string;
  name: string;
  x: number;
  y: number;
  visited: boolean;
  current: boolean;
}

export interface MapEdge {
  from: string;
  to: string;
  locked: boolean;
}

export interface MapData {
  nodes: MapNode[];
  edges: MapEdge[];
}

export interface CombatLogEntry {
  turn: number;
  attacker: string;
  defender: string;
  damage: number;
  defenderHpAfter: number;
  isPlayerAttack: boolean;
}

export interface NarrativeContext {
  locationName: string;
  locationDescription: string;
  mood: string;
  playerHealth: number;
  playerMaxHealth: number;
  inventoryNames: string[];
  roomItemNames: string[];
  roomNpcNames: string[];
  actionType: unknown;
  turnsElapsed: number;
}

export type GameStats = Record<string, number>;

export type ThemeConfig = Record<string, string>;

export interface CustomThemeInfo {
  name: string;
  config: string;
}

export interface ModuleInfo {
  name: string;
  description: string;
  path: string;
  locationCount: number;
  itemCount: number;
}

export type Screen = "title" | "game" | "settings" | "saveload";
