import { invoke } from "@tauri-apps/api/core";
import { lazy, Suspense, useCallback, useEffect, useRef, useState } from "react";

// Lazy-load heavy screens for better initial load time
const AchievementsScreen = lazy(() =>
  import("./components/screens/AchievementsScreen").then((m) => ({
    default: m.AchievementsScreen,
  }))
);
const ReplayScreen = lazy(() =>
  import("./components/screens/ReplayScreen").then((m) => ({
    default: m.ReplayScreen,
  }))
);
const MapEditor = lazy(() => import("./components/editor/MapEditorLazy"));
const StatsScreen = lazy(() =>
  import("./components/screens/StatsScreen").then((m) => ({
    default: m.StatsScreen,
  }))
);
const ThemeCreator = lazy(() =>
  import("./components/screens/ThemeCreator").then((m) => ({
    default: m.ThemeCreator,
  }))
);

// Eagerly load critical screens
import { DeathScreen } from "./components/screens/DeathScreen";
import { EndingScreen } from "./components/screens/EndingScreen";
import { SaveLoadScreen } from "./components/screens/SaveLoadScreen";
import { ModuleSelectScreen } from "./components/screens/ModuleSelectScreen";
import { SettingsPanel } from "./components/screens/SettingsPanel";
import { TitleScreen } from "./components/screens/TitleScreen";
import { SidePanel } from "./components/sidebar/SidePanel";
import { Terminal } from "./components/terminal/Terminal";
import { Transition } from "./components/Transition";
import { TAURI_COMMANDS } from "./lib/tauriCommands";
import { applyTheme } from "./lib/themes";
import type {
  CommandResponse,
  EndingType,
  Screen,
  ThemeName,
  WorldState,
} from "./store/types";

type Overlay = null | "settings" | "save" | "load" | "stats" | "modules" | "themeCreator" | "achievements" | "replays" | "editor";

function getGameOver(mode: WorldState["gameMode"]): EndingType | null {
  if (typeof mode === "object" && "gameOver" in mode) {
    return mode.gameOver;
  }
  return null;
}

function useIsMobile() {
  const [mobile, setMobile] = useState(
    () => window.matchMedia("(max-width: 768px)").matches,
  );
  useEffect(() => {
    const mql = window.matchMedia("(max-width: 768px)");
    const handler = (e: MediaQueryListEvent) => setMobile(e.matches);
    mql.addEventListener("change", handler);
    return () => mql.removeEventListener("change", handler);
  }, []);
  return mobile;
}

export default function App() {
  const [screen, setScreen] = useState<Screen>("title");
  const [overlay, setOverlay] = useState<Overlay>(null);
  const [worldState, setWorldState] = useState<WorldState | null>(null);
  const [statusMessage, setStatusMessage] = useState<string | null>(null);
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const isMobile = useIsMobile();
  const statusTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  function showStatus(msg: string) {
    setStatusMessage(msg);
    if (statusTimer.current) clearTimeout(statusTimer.current);
    statusTimer.current = setTimeout(() => setStatusMessage(null), 3000);
  }

  // Apply default theme on mount; clean up status timer
  useEffect(() => {
    applyTheme("greenTerminal");
    return () => {
      if (statusTimer.current) clearTimeout(statusTimer.current);
    };
  }, []);

  const handleQuickSave = useCallback(async () => {
    try {
      await invoke(TAURI_COMMANDS.saveGame, { slotName: "quicksave" });
      showStatus("Game saved.");
    } catch (err) {
      showStatus(`Save failed: ${err}`);
    }
  }, []);

  // Keyboard shortcuts
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      const key = e.key.toLowerCase();
      if (e.key === "Escape") {
        if (overlay) {
          setOverlay(null);
        } else if (sidebarOpen) {
          setSidebarOpen(false);
        } else if (screen === "game") {
          setOverlay("settings");
        }
      }
      if ((e.metaKey || e.ctrlKey) && key === "s" && screen === "game") {
        e.preventDefault();
        handleQuickSave();
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [screen, overlay, sidebarOpen, handleQuickSave]);

  const handleNewGame = useCallback(async () => {
    setOverlay(null);
    try {
      const response = await invoke<CommandResponse>(TAURI_COMMANDS.newGame);
      setWorldState(response.worldState);
      setScreen("game");
    } catch (err) {
      showStatus(`Failed to start new game: ${err}`);
    }
  }, []);

  const handleSave = useCallback(async (slot: string) => {
    try {
      await invoke(TAURI_COMMANDS.saveGame, { slotName: slot });
      showStatus(`Saved to '${slot}'.`);
    } catch (err) {
      showStatus(`Save failed: ${err}`);
    }
  }, []);

  const handleLoad = useCallback(async (slot: string) => {
    try {
      const response = await invoke<CommandResponse>(TAURI_COMMANDS.loadGame, {
        slotName: slot,
      });
      setWorldState(response.worldState);
      setScreen("game");
      setOverlay(null);
      showStatus(`Loaded '${slot}'.`);
    } catch (err) {
      showStatus(`Load failed: ${err}`);
    }
  }, []);

  const handleThemeChange = useCallback((theme: ThemeName) => {
    applyTheme(theme);
  }, []);

  const handleWorldStateChange = useCallback((state: WorldState) => {
    setWorldState(state);
  }, []);

  const endingType = worldState ? getGameOver(worldState.gameMode) : null;

  const statusBar = statusMessage && (
    <div className="fixed bottom-4 left-1/2 z-50 -translate-x-1/2 border border-[var(--border)] bg-[var(--panel-bg)] px-4 py-2 font-mono text-xs text-[var(--system)]">
      {statusMessage}
    </div>
  );

  if (screen === "title") {
    return (
      <>
        <Transition show={true} type="fade">
          <TitleScreen
            onNewGame={handleNewGame}
            onLoadGame={() => setOverlay("load")}
            onSettings={() => setOverlay("settings")}
            onStats={() => setOverlay("stats")}
            onModules={() => setOverlay("modules")}
            onAchievements={() => setOverlay("achievements")}
            onReplays={() => setOverlay("replays")}
            onEditor={() => setOverlay("editor")}
          />
        </Transition>
        {overlay === "settings" && (
          <SettingsPanel
            onClose={() => setOverlay(null)}
            onThemeChange={handleThemeChange}
            onOpenThemeCreator={() => setOverlay("themeCreator")}
            onOpenLoad={() => setOverlay("load")}
          />
        )}
        {overlay === "themeCreator" && (
          <Suspense fallback={null}>
            <ThemeCreator onClose={() => setOverlay("settings")} />
          </Suspense>
        )}
        {overlay === "load" && (
          <SaveLoadScreen
            mode="load"
            onLoad={handleLoad}
            onClose={() => setOverlay(null)}
          />
        )}
        {overlay === "stats" && (
          <Suspense fallback={null}>
            <StatsScreen onClose={() => setOverlay(null)} />
          </Suspense>
        )}
        {overlay === "modules" && (
          <ModuleSelectScreen
            onModuleLoaded={(response) => {
              setWorldState(response.worldState);
              setScreen("game");
              setOverlay(null);
            }}
            onClose={() => setOverlay(null)}
          />
        )}
        {overlay === "achievements" && (
          <Suspense fallback={null}>
            <AchievementsScreen onClose={() => setOverlay(null)} />
          </Suspense>
        )}
        {overlay === "replays" && (
          <Suspense fallback={null}>
            <ReplayScreen onClose={() => setOverlay(null)} />
          </Suspense>
        )}
        {overlay === "editor" && (
          <Suspense fallback={null}>
            <MapEditor onClose={() => setOverlay(null)} />
          </Suspense>
        )}
        {statusBar}
      </>
    );
  }

  return (
    <div className="flex h-full">
      <div className="flex flex-1 flex-col">
        <Terminal onWorldStateChange={handleWorldStateChange} />
      </div>

      {/* Sidebar toggle button (mobile only) */}
      {isMobile && worldState && !sidebarOpen && (
        <button
          className="sidebar-toggle"
          onClick={() => setSidebarOpen(true)}
        >
          [info]
        </button>
      )}

      {/* Sidebar: drawer on mobile, static on desktop */}
      {worldState && isMobile && sidebarOpen && (
        <div className="sidebar-backdrop" onClick={() => setSidebarOpen(false)} />
      )}
      {worldState && (!isMobile || sidebarOpen) && (
        <div className={isMobile ? "sidebar-drawer open" : ""}>
          <SidePanel
            worldState={worldState}
            onClose={isMobile ? () => setSidebarOpen(false) : undefined}
          />
        </div>
      )}

      <Transition show={endingType === "death" && !!worldState} type="slideUp">
        {endingType === "death" && worldState && (
          <DeathScreen
            player={worldState.player}
            onLoadSave={() => setOverlay("load")}
            onNewGame={handleNewGame}
          />
        )}
      </Transition>

      <Transition show={!!endingType && endingType !== "death" && !!worldState} type="slideUp">
        {endingType && endingType !== "death" && worldState && (
          <EndingScreen
            endingType={endingType}
            player={worldState.player}
            onPlayAgain={handleNewGame}
          />
        )}
      </Transition>

      {overlay === "settings" && (
        <SettingsPanel
          onClose={() => setOverlay(null)}
          onThemeChange={handleThemeChange}
          onOpenThemeCreator={() => setOverlay("themeCreator")}
          onOpenSave={() => setOverlay("save")}
          onOpenLoad={() => setOverlay("load")}
        />
      )}

      {overlay === "themeCreator" && (
        <ThemeCreator onClose={() => setOverlay("settings")} />
      )}

      {overlay === "save" && (
        <SaveLoadScreen
          mode="save"
          onSave={handleSave}
          onClose={() => setOverlay(null)}
        />
      )}

      {overlay === "load" && (
        <SaveLoadScreen
          mode="load"
          onLoad={handleLoad}
          onClose={() => setOverlay(null)}
        />
      )}

      {statusBar}
    </div>
  );
}
