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
  lore: string | null;
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
  examineDetails: string | null;
  revisitDescription: string | null;
}

export type StatusEffectType = "poison" | "blessed" | "weakened" | "burning";

export interface StatusEffect {
  effectType: StatusEffectType;
  name: string;
  turnsRemaining: number;
  damagePerTurn: number;
  attackModifier: number;
  defenseModifier: number;
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
  statusEffects: StatusEffect[];
  discoveredSecrets: string[];
}

export interface NpcMemory {
  turn: number;
  event: string;
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
  examineText: string | null;
  relationship: number;
  memory: NpcMemory[];
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
  completedTurn: number | null;
}

export type JournalCategory = "lore" | "bestiary" | "location" | "item";

export interface JournalEntry {
  id: string;
  category: JournalCategory;
  title: string;
  content: string;
  discoveredTurn: number;
}

export interface CraftingRecipe {
  id: string;
  inputs: string[];
  output: string;
  hint: string;
  discovered: boolean;
}

export interface DialogueHistoryEntry {
  role: string;
  text: string;
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
  difficulty: Difficulty;
  journal: JournalEntry[];
  recipes: CraftingRecipe[];
  dialogueHistory: DialogueHistoryEntry[];
  commandLog: CommandLogEntry[];
}

export interface OutputLine {
  text: string;
  lineType: LineType;
}

export interface CommandResponse {
  messages: OutputLine[];
  worldState: WorldState;
  soundCues: SoundCue[];
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
  soundEnabled: boolean;
  soundVolume: number;
  difficulty: Difficulty;
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

export type Difficulty = "easy" | "normal" | "hard";

export type SoundCue =
  | "ambientPeaceful"
  | "ambientDark"
  | "ambientTense"
  | "ambientSacred"
  | "combatHit"
  | "combatMiss"
  | "combatVictory"
  | "itemPickup"
  | "itemDrop"
  | "itemUse"
  | "doorUnlock"
  | "questComplete"
  | "questStart"
  | "playerDeath"
  | "npcGreeting"
  | "fleeSuccess"
  | "fleeFail";

export interface AchievementInfo {
  id: string;
  name: string;
  description: string;
  icon: string;
  unlocked: boolean;
  unlockedAt: string | null;
}

export interface CommandLogEntry {
  turn: number;
  input: string;
  location: string;
  timestampMs: number;
}

export interface ReplayInfo {
  id: number;
  endedAt: string;
  endingType: string | null;
  turnsTaken: number | null;
  questsCompleted: number | null;
  commandCount: number;
}

export interface ReplayDetail {
  info: ReplayInfo;
  commands: CommandLogEntry[];
}

export interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

export interface EditorRoom {
  id: string;
  name: string;
  description: string;
  x: number;
  y: number;
  mood: Mood;
  items: string[];
  npcs: string[];
  examineDetails: string | null;
}

export interface EditorConnection {
  fromId: string;
  toId: string;
  fromDir: Direction;
  toDir: Direction;
  locked: boolean;
  keyId: string | null;
}

export type Screen = "title" | "game" | "settings" | "saveload";
