import type { Page } from "@playwright/test";

export class GamePage {
  constructor(public page: Page) {}

  async goto() {
    await this.page.addInitScript(() => {
      const globalAny = window as typeof window & Record<string, unknown>;
      if (globalAny.__LOREKEEPER_TAURI_MOCK__) return;

      const clone = <T>(value: T): T => JSON.parse(JSON.stringify(value));

      const makeWorldState = () => ({
        player: {
          location: "courtyard",
          inventory: [],
          maxInventory: 10,
          health: 100,
          maxHealth: 100,
          attack: 5,
          defense: 3,
          equippedWeapon: null,
          equippedArmor: null,
          questFlags: {},
          visitedLocations: ["courtyard"],
          turnsElapsed: 0,
          statusEffects: [],
          discoveredSecrets: [],
        },
        locations: {
          courtyard: {
            id: "courtyard",
            name: "Courtyard",
            description: "The ruined Courtyard of Thornhold.",
            items: ["rusty_lantern"],
            npcs: [],
            exits: { east: "great_hall" },
            lockedExits: {},
            visited: true,
            discoveredSecrets: [],
            ambientMood: "mysterious",
            examineDetails: null,
            revisitDescription: null,
          },
          great_hall: {
            id: "great_hall",
            name: "Great Hall",
            description: "A vast hall lined with broken banners.",
            items: [],
            npcs: [],
            exits: { west: "courtyard" },
            lockedExits: {},
            visited: false,
            discoveredSecrets: [],
            ambientMood: "tense",
            examineDetails: null,
            revisitDescription: null,
          },
        },
        items: {
          rusty_lantern: {
            id: "rusty_lantern",
            name: "Rusty Lantern",
            description: "An old lantern with cracked glass.",
            itemType: "miscellaneous",
            modifier: null,
            usable: false,
            consumable: false,
            keyId: null,
            lore: null,
          },
        },
        npcs: {},
        quests: {},
        events: [],
        gameMode: "exploring",
        combatState: null,
        messageLog: [],
        combatLog: [],
        lastNarrativeContext: null,
        initialized: true,
        difficulty: "normal",
        journal: [],
        recipes: [],
        dialogueHistory: [],
        commandLog: [],
      });

      const state: {
        worldState: ReturnType<typeof makeWorldState>;
        settings: {
          ollamaEnabled: boolean;
          ollamaModel: string;
          ollamaUrl: string;
          temperature: number;
          narratorTone: string;
          typewriterSpeed: number;
          theme: "greenTerminal" | "amberTerminal" | "parchment" | "darkModern";
          narrationVerbosity: string;
          soundEnabled: boolean;
          soundVolume: number;
          difficulty: "easy" | "normal" | "hard";
        };
        saves: Record<string, { worldState: ReturnType<typeof makeWorldState>; savedAt: string }>;
        stats: Record<string, number>;
      } = {
        worldState: makeWorldState(),
        settings: {
          ollamaEnabled: false,
          ollamaModel: "llama3.2",
          ollamaUrl: "http://localhost:11434",
          temperature: 0.7,
          narratorTone: "atmospheric",
          typewriterSpeed: 30,
          theme: "greenTerminal",
          narrationVerbosity: "normal",
          soundEnabled: false,
          soundVolume: 0.5,
          difficulty: "normal",
        },
        saves: {},
        stats: {
          commandsEntered: 0,
          roomsExplored: 1,
          enemiesDefeated: 0,
          deaths: 0,
          questsCompleted: 0,
        },
      };

      const response = (text: string) => ({
        messages: [{ text, lineType: "narration" }],
        worldState: clone(state.worldState),
        soundCues: [],
      });

      globalAny.__TAURI_EVENT_PLUGIN_INTERNALS__ = {
        unregisterListener: () => {},
      };

      globalAny.__TAURI_INTERNALS__ = {
        invoke: async (cmd: string, args: Record<string, unknown> = {}) => {
          switch (cmd) {
            case "plugin:event|listen":
              return 1;
            case "plugin:event|unlisten":
            case "plugin:event|emit":
            case "plugin:event|emit_to":
              return null;
            case "initialize_game":
            case "new_game":
              state.worldState = makeWorldState();
              return response("You descend into The Depths of Thornhold. Courtyard.");
            case "process_command": {
              const input = String(args.input ?? "").trim().toLowerCase();
              state.worldState.player.turnsElapsed += 1;
              state.stats.commandsEntered += 1;
              if (input === "look") {
                return response(
                  `Courtyard. ${state.worldState.locations[state.worldState.player.location].description}`,
                );
              }
              if (input === "inventory") {
                const inv = state.worldState.player.inventory;
                return response(inv.length ? `Inventory: ${inv.join(", ")}` : "Inventory is empty.");
              }
              if (input === "help") {
                return response("Try: look, go east, go west, take rusty_lantern.");
              }
              if (input === "take rusty_lantern") {
                if (!state.worldState.player.inventory.includes("rusty_lantern")) {
                  state.worldState.player.inventory.push("rusty_lantern");
                }
                state.worldState.locations.courtyard.items = [];
                return response("You take the rusty lantern.");
              }
              if (input === "go east") {
                state.worldState.player.location = "great_hall";
                if (!state.worldState.player.visitedLocations.includes("great_hall")) {
                  state.worldState.player.visitedLocations.push("great_hall");
                }
                state.stats.roomsExplored = Math.max(state.stats.roomsExplored, 2);
                return response("You enter the Great Hall.");
              }
              if (input === "go west") {
                state.worldState.player.location = "courtyard";
                return response("You return to the Courtyard.");
              }
              return response("The command echoes in the dark, but nothing happens.");
            }
            case "save_game": {
              const slotName = String(args.slotName ?? "quicksave");
              const savedAt = new Date().toISOString();
              state.saves[slotName] = { worldState: clone(state.worldState), savedAt };
              return { slotName, savedAt };
            }
            case "load_game": {
              const slotName = String(args.slotName ?? "");
              const saved = state.saves[slotName];
              if (!saved) throw new Error("Save not found");
              state.worldState = clone(saved.worldState);
              return response(`Loaded ${slotName}. Courtyard.`);
            }
            case "list_saves":
              return Object.entries(state.saves).map(([slotName, entry]) => ({
                slotName,
                playerLocation:
                  entry.worldState.locations[entry.worldState.player.location]?.name ?? null,
                playerHealth: entry.worldState.player.health,
                turnsElapsed: entry.worldState.player.turnsElapsed,
                questsCompleted: 0,
                savedAt: entry.savedAt,
              }));
            case "delete_save": {
              const slotName = String(args.slotName ?? "");
              delete state.saves[slotName];
              return null;
            }
            case "get_settings":
              return clone(state.settings);
            case "update_settings":
              state.settings = {
                ...state.settings,
                ...(args.settings as Record<string, unknown>),
              };
              return null;
            case "get_ollama_status":
              return { connected: false, version: null };
            case "get_available_models":
              return [];
            case "get_completions":
              return [];
            case "get_map_data":
              return { nodes: [], edges: [] };
            case "get_stats":
              return clone(state.stats);
            case "reset_stats":
              state.stats = {
                commandsEntered: 0,
                roomsExplored: 0,
                enemiesDefeated: 0,
                deaths: 0,
                questsCompleted: 0,
              };
              return null;
            case "list_modules":
              return [];
            case "load_module":
              return response("Module loaded.");
            case "list_custom_themes":
              return [];
            case "save_custom_theme":
            case "delete_custom_theme":
              return null;
            case "get_achievements":
              return [];
            case "list_replays":
              return [];
            case "get_replay":
              throw new Error("Replay not found");
            case "retry_narration":
            case "rate_narration":
              return null;
            default:
              throw new Error(`Unhandled mock Tauri command: ${cmd}`);
          }
        },
        transformCallback: () => 1,
        unregisterCallback: () => {},
        convertFileSrc: (filePath: string) => filePath,
      };

      globalAny.__LOREKEEPER_TAURI_MOCK__ = true;
    });
    await this.page.goto("/");
  }

  get titleHeading() {
    return this.page.locator("h1");
  }

  get newGameButton() {
    return this.page.getByRole("button", { name: "New Game" });
  }

  get loadGameButton() {
    return this.page.getByRole("button", { name: "Load Game" });
  }

  get settingsButton() {
    return this.page.getByRole("button", { name: "Settings" });
  }

  get commandInput() {
    return this.page.getByLabel("Game command input");
  }

  get gameOutput() {
    return this.page.getByLabel("Game output");
  }

  async typeCommand(cmd: string) {
    await this.commandInput.fill(cmd);
    await this.commandInput.press("Enter");
  }
}
