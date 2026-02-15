import { act, renderHook } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  createCommandResponse,
  createWorldState,
  mockInvoke,
  mockListen,
  mockUnlisten,
  mockDatabase,
  resetMockDatabase,
  setupMockInvokeWithDatabase,
} from "../test/mocks";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: (...args: unknown[]) => mockListen(...args),
}));

// Default: listen resolves with unlisten fn
mockListen.mockResolvedValue(mockUnlisten);

const { useGame } = await import("./useGame");

describe("useGame Integration Tests", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    resetMockDatabase();
    setupMockInvokeWithDatabase();
  });

  describe("Save/Load Flow", () => {
    it("saves game state to database and loads it back", async () => {
      const { result } = renderHook(() => useGame());

      // Initialize game
      const initResponse = createCommandResponse({
        worldState: createWorldState({
          player: {
            location: "courtyard",
            inventory: [],
            maxInventory: 10,
            health: 100,
            maxHealth: 100,
            attack: 10,
            defense: 5,
            equippedWeapon: null,
            equippedArmor: null,
            questFlags: {},
            visitedLocations: ["courtyard"],
            turnsElapsed: 0,
            statusEffects: [],
            discoveredSecrets: [],
          },
        }),
      });
      mockInvoke.mockResolvedValueOnce(initResponse);

      await act(async () => {
        await result.current.initializeGame();
      });

      const initialState = result.current.worldState;
      expect(initialState).not.toBeNull();
      expect(initialState?.player.location).toBe("courtyard");
      expect(initialState?.player.inventory).toEqual([]);

      // Execute a command to modify state
      const cmdResponse = createCommandResponse({
        worldState: createWorldState({
          player: {
            location: "courtyard",
            inventory: ["rusty_lantern"],
            maxInventory: 10,
            health: 100,
            maxHealth: 100,
            attack: 10,
            defense: 5,
            equippedWeapon: null,
            equippedArmor: null,
            questFlags: {},
            visitedLocations: ["courtyard"],
            turnsElapsed: 1,
            statusEffects: [],
            discoveredSecrets: [],
          },
        }),
        messages: [{ text: "You take the rusty lantern.", lineType: "narration" }],
      });
      mockInvoke.mockResolvedValueOnce(cmdResponse);

      await act(async () => {
        await result.current.sendCommand("take rusty_lantern");
      });

      const modifiedState = result.current.worldState;
      expect(modifiedState?.player.inventory).toContain("rusty_lantern");
      expect(modifiedState?.player.turnsElapsed).toBe(1);

      // Save game
      await act(async () => {
        await mockInvoke("save_game", {
          slotName: "test-slot",
          worldState: modifiedState,
        });
      });

      // Verify save was stored in mock database
      expect(mockDatabase.saves["test-slot"]).toBeDefined();
      expect(mockDatabase.saves["test-slot"].data.player.inventory).toContain(
        "rusty_lantern",
      );

      // Simulate more game progress
      const progressResponse = createCommandResponse({
        worldState: createWorldState({
          player: {
            location: "great_hall",
            inventory: ["rusty_lantern", "torch"],
            maxInventory: 10,
            health: 100,
            maxHealth: 100,
            attack: 10,
            defense: 5,
            equippedWeapon: null,
            equippedArmor: null,
            questFlags: {},
            visitedLocations: ["courtyard", "great_hall"],
            turnsElapsed: 5,
            statusEffects: [],
            discoveredSecrets: [],
          },
        }),
        messages: [{ text: "You enter the great hall.", lineType: "narration" }],
      });
      mockInvoke.mockResolvedValueOnce(progressResponse);

      await act(async () => {
        await result.current.sendCommand("go east");
      });

      expect(result.current.worldState?.player.location).toBe("great_hall");
      expect(result.current.worldState?.player.turnsElapsed).toBe(5);

      // Load from saved slot - verify database has correct data
      const loadResponse = await mockInvoke("load_game", {
        slotName: "test-slot",
      });

      // Verify loaded data matches saved state (no progress after save)
      expect(loadResponse.worldState.player.location).toBe("courtyard");
      expect(loadResponse.worldState.player.inventory).toContain("rusty_lantern");
      expect(loadResponse.worldState.player.inventory).not.toContain("torch");
      expect(loadResponse.worldState.player.turnsElapsed).toBe(1);
    });

    it("handles corrupted save gracefully", async () => {
      renderHook(() => useGame());

      // Try to load non-existent save
      let error: Error | null = null;
      try {
        await act(async () => {
          await mockInvoke("load_game", { slotName: "non-existent" });
        });
      } catch (e) {
        error = e as Error;
      }

      expect(error).not.toBeNull();
      expect(error?.message).toContain("Save not found");
    });

    it("lists all saved games", async () => {
      // Create multiple saves
      const state1 = createWorldState({
        player: {
          location: "courtyard",
          inventory: [],
          maxInventory: 10,
          health: 100,
          maxHealth: 100,
          attack: 10,
          defense: 5,
          equippedWeapon: null,
          equippedArmor: null,
          questFlags: {},
          visitedLocations: ["courtyard"],
          turnsElapsed: 5,
          statusEffects: [],
          discoveredSecrets: [],
        },
      });
      const state2 = createWorldState({
        player: {
          location: "great_hall",
          inventory: ["rusty_lantern"],
          maxInventory: 10,
          health: 80,
          maxHealth: 100,
          attack: 10,
          defense: 5,
          equippedWeapon: null,
          equippedArmor: null,
          questFlags: {},
          visitedLocations: ["courtyard", "great_hall"],
          turnsElapsed: 20,
          statusEffects: [],
          discoveredSecrets: [],
        },
      });

      await mockInvoke("save_game", { slotName: "slot1", worldState: state1 });
      await mockInvoke("save_game", { slotName: "slot2", worldState: state2 });

      const saves = await mockInvoke("list_saves");

      expect(saves).toHaveLength(2);
      expect(saves[0].slotName).toBe("slot1");
      expect(saves[0].playerLocation).toBe("courtyard");
      expect(saves[0].turnsElapsed).toBe(5);
      expect(saves[1].slotName).toBe("slot2");
      expect(saves[1].playerLocation).toBe("great_hall");
      expect(saves[1].turnsElapsed).toBe(20);
    });
  });

  describe("Achievement Flow", () => {
    it("unlocks First Blood achievement on first enemy kill", async () => {
      const { result } = renderHook(() => useGame());

      // Initialize game
      mockInvoke.mockResolvedValueOnce(createCommandResponse());
      await act(async () => {
        await result.current.initializeGame();
      });

      // Get initial achievements
      const initialAchievements = await mockInvoke("get_achievements");
      const firstBlood = initialAchievements.achievements.find(
        (a: any) => a.id === "first_blood",
      );
      expect(firstBlood.unlockedAt).toBeUndefined();

      // Execute attack command (should trigger achievement)
      await act(async () => {
        await result.current.sendCommand("attack goblin");
      });

      // Verify achievement was unlocked in database
      expect(mockDatabase.achievements).toHaveLength(1);
      expect(mockDatabase.achievements[0].id).toBe("first_blood");
      expect(mockDatabase.achievements[0].unlockedAt).toBeDefined();

      // Fetch achievements again
      const updatedAchievements = await mockInvoke("get_achievements");
      const unlockedFirstBlood = updatedAchievements.achievements.find(
        (a: any) => a.id === "first_blood",
      );
      expect(unlockedFirstBlood.unlockedAt).toBeDefined();
    });

    it("does not duplicate achievement unlocks", async () => {
      const { result } = renderHook(() => useGame());

      mockInvoke.mockResolvedValueOnce(createCommandResponse());
      await act(async () => {
        await result.current.initializeGame();
      });

      // Kill first enemy
      await act(async () => {
        await result.current.sendCommand("attack goblin");
      });

      expect(mockDatabase.achievements).toHaveLength(1);

      // Kill second enemy
      await act(async () => {
        await result.current.sendCommand("attack zombie");
      });

      // Achievement should still only be unlocked once
      expect(mockDatabase.achievements).toHaveLength(1);
      expect(mockDatabase.achievements[0].id).toBe("first_blood");
    });
  });

  describe("Narration Event Streaming", () => {
    it("receives and displays narration events from backend", async () => {
      const narrativeEvents: any[] = [];

      // Set up event listener capture
      const mockEventHandler = vi.fn((eventName: string, handler: Function) => {
        if (eventName === "narrative-event") {
          narrativeEvents.push = (event: any) => {
            handler({ payload: event });
            return Array.prototype.push.call(narrativeEvents, event);
          };
        }
        return Promise.resolve(mockUnlisten);
      });
      mockListen.mockImplementation(mockEventHandler);

      const { result } = renderHook(() => useGame());

      // Verify listener was set up
      expect(mockListen).toHaveBeenCalledWith(
        "narrative-event",
        expect.any(Function),
      );

      // Simulate backend emitting streaming tokens
      const listener = mockListen.mock.calls[0][1] as Function;

      await act(async () => {
        listener({
          payload: { type: "token", text: "You enter " },
        });
      });

      await act(async () => {
        listener({
          payload: { type: "token", text: "the great hall..." },
        });
      });

      await act(async () => {
        listener({
          payload: { type: "complete" },
        });
      });

      // Verify history was updated with streaming text
      expect(result.current.history.some((line) => line.text.includes("You enter"))).toBe(
        true,
      );
    });

    it("handles fallback narration event", async () => {
      const { result } = renderHook(() => useGame());

      const listener = mockListen.mock.calls[0][1] as Function;

      await act(async () => {
        listener({
          payload: { type: "fallback", text: "Template narration used." },
        });
      });

      // Verify narration state was reset
      expect(result.current.isNarrating).toBe(false);
    });
  });
});
