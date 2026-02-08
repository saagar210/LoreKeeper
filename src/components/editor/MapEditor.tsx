import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useRef, useState } from "react";
import { trapFocus } from "../../lib/focusTrap";
import type {
  Direction,
  EditorConnection,
  EditorRoom,
  Item,
  Mood,
  Npc,
  ValidationResult,
} from "../../store/types";
import { EditorToolbar, type EditorTool } from "./EditorToolbar";
import { RoomEditor } from "./RoomEditor";

interface Props {
  onClose: () => void;
}

const ROOM_W = 120;
const ROOM_H = 60;

const ALL_DIRECTIONS: Direction[] = ["north", "south", "east", "west", "up", "down"];

function oppositeDir(d: Direction): Direction {
  const map: Record<Direction, Direction> = {
    north: "south",
    south: "north",
    east: "west",
    west: "east",
    up: "down",
    down: "up",
  };
  return map[d];
}

function makeId(name: string): string {
  return name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "_")
    .replace(/^_|_$/g, "");
}

function defaultRoom(x: number, y: number, counter: number): EditorRoom {
  const name = `Room ${counter}`;
  return {
    id: makeId(name),
    name,
    description: "A new room.",
    x,
    y,
    mood: "peaceful" as Mood,
    items: [],
    npcs: [],
    examineDetails: null,
  };
}

function defaultItem(id: string): Item {
  return {
    id,
    name: id.replace(/_/g, " ").replace(/\b\w/g, (c) => c.toUpperCase()),
    description: "An item.",
    itemType: "miscellaneous",
    modifier: null,
    usable: false,
    consumable: false,
    keyId: null,
    lore: null,
  };
}

function defaultNpc(id: string): Npc {
  return {
    id,
    name: id.replace(/_/g, " ").replace(/\b\w/g, (c) => c.toUpperCase()),
    description: "A character.",
    personalitySeed: "neutral",
    dialogueState: "greeting",
    hostile: false,
    health: 50,
    maxHealth: 50,
    attack: 5,
    defense: 3,
    items: [],
    questGiver: null,
    examineText: null,
    relationship: 0,
    memory: [],
  };
}

function buildWorldStateJson(
  rooms: EditorRoom[],
  connections: EditorConnection[],
  items: Record<string, Item>,
  npcs: Record<string, Npc>,
): string {
  const locations: Record<string, unknown> = {};
  for (const room of rooms) {
    const exits: Record<string, string> = {};
    const lockedExits: Record<string, string> = {};
    for (const conn of connections) {
      if (conn.fromId === room.id) {
        exits[conn.fromDir] = conn.toId;
        if (conn.locked && conn.keyId) {
          lockedExits[conn.fromDir] = conn.keyId;
        }
      }
      if (conn.toId === room.id) {
        exits[conn.toDir] = conn.fromId;
        if (conn.locked && conn.keyId) {
          lockedExits[conn.toDir] = conn.keyId;
        }
      }
    }
    locations[room.id] = {
      id: room.id,
      name: room.name,
      description: room.description,
      items: room.items,
      npcs: room.npcs,
      exits,
      lockedExits,
      visited: false,
      discoveredSecrets: [],
      ambientMood: room.mood,
      examineDetails: room.examineDetails,
      revisitDescription: null,
    };
  }

  const startLocation = rooms.length > 0 ? rooms[0].id : "courtyard";
  const visitedLocations = rooms.length > 0 ? [rooms[0].id] : [];

  const worldState = {
    player: {
      location: startLocation,
      inventory: [],
      maxInventory: 10,
      health: 100,
      maxHealth: 100,
      attack: 5,
      defense: 3,
      equippedWeapon: null,
      equippedArmor: null,
      questFlags: {},
      visitedLocations,
      turnsElapsed: 0,
      statusEffects: [],
      discoveredSecrets: [],
    },
    locations,
    items,
    npcs,
    quests: {},
    events: [],
    gameMode: "exploring",
    combatState: null,
    messageLog: [],
    combatLog: [],
    lastNarrativeContext: null,
    initialized: false,
    difficulty: "normal",
    journal: [],
    recipes: [],
    dialogueHistory: [],
    commandLog: [],
  };

  return JSON.stringify(worldState);
}

export function MapEditor({ onClose }: Props) {
  const [rooms, setRooms] = useState<EditorRoom[]>([]);
  const [connections, setConnections] = useState<EditorConnection[]>([]);
  const [items, setItems] = useState<Record<string, Item>>({});
  const [npcs, setNpcs] = useState<Record<string, Npc>>({});
  const [selectedRoomId, setSelectedRoomId] = useState<string | null>(null);
  const [selectedTool, setSelectedTool] = useState<EditorTool>("select");
  const [validation, setValidation] = useState<ValidationResult | null>(null);
  const [connectFrom, setConnectFrom] = useState<string | null>(null);
  const [dragging, setDragging] = useState<{ roomId: string; offsetX: number; offsetY: number } | null>(null);
  const [exportName, setExportName] = useState("");
  const [showExportDialog, setShowExportDialog] = useState(false);
  const [statusMsg, setStatusMsg] = useState<string | null>(null);
  const roomCounterRef = useRef(1);

  const dialogRef = useRef<HTMLDivElement>(null);
  const svgRef = useRef<SVGSVGElement>(null);
  const statusTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    if (dialogRef.current) {
      return trapFocus(dialogRef.current);
    }
  }, []);

  const showStatus = useCallback((msg: string) => {
    setStatusMsg(msg);
    if (statusTimer.current) clearTimeout(statusTimer.current);
    statusTimer.current = setTimeout(() => setStatusMsg(null), 3000);
  }, []);

  useEffect(() => {
    return () => {
      if (statusTimer.current) clearTimeout(statusTimer.current);
    };
  }, []);

  const selectedRoom = rooms.find((r) => r.id === selectedRoomId) ?? null;

  // SVG click handler
  const handleSvgClick = useCallback(
    (e: React.MouseEvent<SVGSVGElement>) => {
      if (e.target !== svgRef.current) return; // only blank area
      if (selectedTool === "addRoom") {
        const svg = svgRef.current;
        if (!svg) return;
        const rect = svg.getBoundingClientRect();
        const x = e.clientX - rect.left - ROOM_W / 2;
        const y = e.clientY - rect.top - ROOM_H / 2;
        const newRoom = defaultRoom(Math.max(0, x), Math.max(0, y), roomCounterRef.current++);
        setRooms((prev) => [...prev, newRoom]);
        setSelectedRoomId(newRoom.id);
        setSelectedTool("select");
      } else {
        setSelectedRoomId(null);
        setConnectFrom(null);
      }
    },
    [selectedTool],
  );

  // Room click handler
  const handleRoomClick = useCallback(
    (roomId: string) => {
      if (selectedTool === "select") {
        setSelectedRoomId(roomId);
      } else if (selectedTool === "delete") {
        setRooms((prev) => prev.filter((r) => r.id !== roomId));
        setConnections((prev) =>
          prev.filter((c) => c.fromId !== roomId && c.toId !== roomId),
        );
        if (selectedRoomId === roomId) setSelectedRoomId(null);
      } else if (selectedTool === "connect") {
        if (connectFrom === null) {
          setConnectFrom(roomId);
        } else if (connectFrom !== roomId) {
          // Find available directions
          const fromRoom = rooms.find((r) => r.id === connectFrom);
          const toRoom = rooms.find((r) => r.id === roomId);
          if (fromRoom && toRoom) {
            const usedFromDirs = connections
              .filter((c) => c.fromId === connectFrom || c.toId === connectFrom)
              .flatMap((c) => (c.fromId === connectFrom ? [c.fromDir] : [c.toDir]));
            const usedToDirs = connections
              .filter((c) => c.fromId === roomId || c.toId === roomId)
              .flatMap((c) => (c.fromId === roomId ? [c.fromDir] : [c.toDir]));

            const availFrom = ALL_DIRECTIONS.filter((d) => !usedFromDirs.includes(d));
            const availTo = ALL_DIRECTIONS.filter((d) => !usedToDirs.includes(d));

            // Pick best direction based on relative position
            const dx = toRoom.x - fromRoom.x;
            const dy = toRoom.y - fromRoom.y;
            let preferredFrom: Direction;
            if (Math.abs(dx) > Math.abs(dy)) {
              preferredFrom = dx > 0 ? "east" : "west";
            } else {
              preferredFrom = dy > 0 ? "south" : "north";
            }
            const fromDir = availFrom.includes(preferredFrom)
              ? preferredFrom
              : availFrom[0];
            const toDir = availTo.includes(oppositeDir(fromDir))
              ? oppositeDir(fromDir)
              : availTo[0];

            if (fromDir && toDir) {
              // Check for existing connection between these rooms
              const duplicate = connections.some(
                (c) =>
                  (c.fromId === connectFrom && c.toId === roomId) ||
                  (c.fromId === roomId && c.toId === connectFrom),
              );
              if (duplicate) {
                showStatus("These rooms are already connected.");
              } else {
                setConnections((prev) => [
                  ...prev,
                  {
                    fromId: connectFrom,
                    toId: roomId,
                    fromDir,
                    toDir,
                    locked: false,
                    keyId: null,
                  },
                ]);
              }
            } else {
              showStatus("No available directions for connection.");
            }
          }
          setConnectFrom(null);
          setSelectedTool("select");
        }
      }
    },
    [selectedTool, connectFrom, rooms, connections, selectedRoomId, showStatus],
  );

  // Drag handlers
  const handleMouseDown = useCallback(
    (e: React.MouseEvent, roomId: string) => {
      if (selectedTool !== "select") return;
      e.stopPropagation();
      const room = rooms.find((r) => r.id === roomId);
      if (!room || !svgRef.current) return;
      const rect = svgRef.current.getBoundingClientRect();
      setDragging({
        roomId,
        offsetX: e.clientX - rect.left - room.x,
        offsetY: e.clientY - rect.top - room.y,
      });
    },
    [selectedTool, rooms],
  );

  const handleMouseMove = useCallback(
    (e: React.MouseEvent) => {
      if (!dragging || !svgRef.current) return;
      const rect = svgRef.current.getBoundingClientRect();
      const newX = Math.max(0, e.clientX - rect.left - dragging.offsetX);
      const newY = Math.max(0, e.clientY - rect.top - dragging.offsetY);
      setRooms((prev) =>
        prev.map((r) =>
          r.id === dragging.roomId ? { ...r, x: newX, y: newY } : r,
        ),
      );
    },
    [dragging],
  );

  const handleMouseUp = useCallback(() => {
    setDragging(null);
  }, []);

  // Room property update
  const handleRoomUpdate = useCallback(
    (updated: EditorRoom) => {
      setRooms((prev) => {
        const oldRoom = prev.find((r) => r.id === selectedRoomId);
        const newRooms = prev.map((r) => (r.id === selectedRoomId ? updated : r));
        // Update connection references if ID changed
        if (oldRoom && oldRoom.id !== updated.id) {
          setConnections((prevConns) =>
            prevConns.map((c) => ({
              ...c,
              fromId: c.fromId === oldRoom.id ? updated.id : c.fromId,
              toId: c.toId === oldRoom.id ? updated.id : c.toId,
            })),
          );
          setSelectedRoomId(updated.id);
        }
        return newRooms;
      });
    },
    [selectedRoomId],
  );

  // Item management
  const handleAddItem = useCallback(() => {
    if (!selectedRoomId) return;
    const id = prompt("Enter item ID (e.g. rusty_sword):");
    if (!id) return;
    const trimmed = id.trim().toLowerCase().replace(/\s+/g, "_");
    if (!trimmed) return;
    if (!items[trimmed]) {
      setItems((prev) => ({ ...prev, [trimmed]: defaultItem(trimmed) }));
    }
    setRooms((prev) =>
      prev.map((r) =>
        r.id === selectedRoomId && !r.items.includes(trimmed)
          ? { ...r, items: [...r.items, trimmed] }
          : r,
      ),
    );
  }, [selectedRoomId, items]);

  const handleRemoveItem = useCallback(
    (itemId: string) => {
      if (!selectedRoomId) return;
      setRooms((prev) =>
        prev.map((r) =>
          r.id === selectedRoomId
            ? { ...r, items: r.items.filter((i) => i !== itemId) }
            : r,
        ),
      );
    },
    [selectedRoomId],
  );

  // NPC management
  const handleAddNpc = useCallback(() => {
    if (!selectedRoomId) return;
    const id = prompt("Enter NPC ID (e.g. old_wizard):");
    if (!id) return;
    const trimmed = id.trim().toLowerCase().replace(/\s+/g, "_");
    if (!trimmed) return;
    if (!npcs[trimmed]) {
      setNpcs((prev) => ({ ...prev, [trimmed]: defaultNpc(trimmed) }));
    }
    setRooms((prev) =>
      prev.map((r) =>
        r.id === selectedRoomId && !r.npcs.includes(trimmed)
          ? { ...r, npcs: [...r.npcs, trimmed] }
          : r,
      ),
    );
  }, [selectedRoomId, npcs]);

  const handleRemoveNpc = useCallback(
    (npcId: string) => {
      if (!selectedRoomId) return;
      setRooms((prev) =>
        prev.map((r) =>
          r.id === selectedRoomId
            ? { ...r, npcs: r.npcs.filter((n) => n !== npcId) }
            : r,
        ),
      );
    },
    [selectedRoomId],
  );

  // Validate
  const handleValidate = useCallback(async () => {
    const json = buildWorldStateJson(rooms, connections, items, npcs);
    try {
      const result = await invoke<ValidationResult>("validate_module_json", { json });
      setValidation(result);
      if (result.valid) {
        showStatus("Module is valid.");
      } else {
        showStatus(`Validation found ${result.errors.length} error(s).`);
      }
    } catch (err) {
      showStatus(`Validation error: ${err}`);
    }
  }, [rooms, connections, items, npcs, showStatus]);

  // Export
  const handleExport = useCallback(async () => {
    if (!exportName.trim()) {
      showStatus("Please enter a module name.");
      return;
    }
    const json = buildWorldStateJson(rooms, connections, items, npcs);
    try {
      const path = await invoke<string>("export_module", {
        name: exportName.trim(),
        json,
      });
      showStatus(`Exported to: ${path}`);
      setShowExportDialog(false);
      setExportName("");
    } catch (err) {
      showStatus(`Export failed: ${err}`);
    }
  }, [exportName, rooms, connections, items, npcs, showStatus]);

  // Clear
  const handleClear = useCallback(() => {
    setRooms([]);
    setConnections([]);
    setItems({});
    setNpcs({});
    setSelectedRoomId(null);
    setConnectFrom(null);
    setValidation(null);
    roomCounterRef.current = 1;
  }, []);

  // Get room center for connection lines
  const roomCenter = (roomId: string) => {
    const room = rooms.find((r) => r.id === roomId);
    if (!room) return { cx: 0, cy: 0 };
    return { cx: room.x + ROOM_W / 2, cy: room.y + ROOM_H / 2 };
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 font-mono"
      role="dialog"
      aria-modal="true"
      aria-labelledby="editor-heading"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div
        ref={dialogRef}
        className="flex h-[90vh] w-[90vw] max-w-[1200px] flex-col bg-[var(--panel-bg)] border border-[var(--border)] overflow-hidden"
      >
        {/* Header */}
        <div className="flex items-center justify-between border-b border-[var(--border)] px-4 py-3">
          <h2
            id="editor-heading"
            className="text-lg font-bold text-[var(--accent)]"
          >
            Map Editor
          </h2>
          <button
            onClick={onClose}
            className="text-[var(--text-dim)] hover:text-[var(--text)]"
          >
            [X]
          </button>
        </div>

        {/* Toolbar */}
        <EditorToolbar
          selectedTool={selectedTool}
          onToolChange={(tool) => {
            setSelectedTool(tool);
            setConnectFrom(null);
          }}
          onValidate={handleValidate}
          onExport={() => setShowExportDialog(true)}
          onClear={handleClear}
          validation={validation}
          roomCount={rooms.length}
        />

        {/* Main content */}
        <div className="flex flex-1 overflow-hidden">
          {/* SVG Canvas */}
          <div className="flex-1 overflow-auto bg-[var(--bg)]">
            <svg
              ref={svgRef}
              className="min-h-full min-w-full"
              width="100%"
              height="100%"
              style={{
                cursor:
                  selectedTool === "addRoom"
                    ? "crosshair"
                    : selectedTool === "connect"
                      ? "pointer"
                      : selectedTool === "delete"
                        ? "not-allowed"
                        : "default",
              }}
              onClick={handleSvgClick}
              onMouseMove={handleMouseMove}
              onMouseUp={handleMouseUp}
              onMouseLeave={handleMouseUp}
            >
              {/* Connection lines */}
              {connections.map((conn, i) => {
                const from = roomCenter(conn.fromId);
                const to = roomCenter(conn.toId);
                return (
                  <g key={`conn-${i}`}>
                    <line
                      x1={from.cx}
                      y1={from.cy}
                      x2={to.cx}
                      y2={to.cy}
                      stroke={conn.locked ? "var(--accent)" : "var(--text-dim)"}
                      strokeWidth={conn.locked ? 2 : 1}
                      strokeDasharray={conn.locked ? "4 4" : undefined}
                      opacity={0.6}
                    />
                    <text
                      x={(from.cx + to.cx) / 2}
                      y={(from.cy + to.cy) / 2 - 6}
                      textAnchor="middle"
                      fontSize={9}
                      fill="var(--text-dim)"
                      opacity={0.5}
                    >
                      {conn.fromDir[0].toUpperCase()}-{conn.toDir[0].toUpperCase()}
                    </text>
                  </g>
                );
              })}

              {/* Connect-from indicator line */}
              {connectFrom && (
                <circle
                  cx={roomCenter(connectFrom).cx}
                  cy={roomCenter(connectFrom).cy}
                  r={8}
                  fill="none"
                  stroke="var(--accent)"
                  strokeWidth={2}
                  opacity={0.8}
                />
              )}

              {/* Rooms */}
              {rooms.map((room) => (
                <g
                  key={room.id}
                  onClick={(e) => {
                    e.stopPropagation();
                    handleRoomClick(room.id);
                  }}
                  onMouseDown={(e) => handleMouseDown(e, room.id)}
                  style={{ cursor: selectedTool === "select" ? "grab" : undefined }}
                >
                  <rect
                    x={room.x}
                    y={room.y}
                    width={ROOM_W}
                    height={ROOM_H}
                    rx={4}
                    fill={
                      room.id === selectedRoomId
                        ? "var(--accent)"
                        : room.id === connectFrom
                          ? "var(--accent)"
                          : "var(--panel-bg)"
                    }
                    fillOpacity={
                      room.id === selectedRoomId || room.id === connectFrom
                        ? 0.2
                        : 0.8
                    }
                    stroke={
                      room.id === selectedRoomId
                        ? "var(--accent)"
                        : "var(--border)"
                    }
                    strokeWidth={room.id === selectedRoomId ? 2 : 1}
                  />
                  <text
                    x={room.x + ROOM_W / 2}
                    y={room.y + ROOM_H / 2 - 6}
                    textAnchor="middle"
                    fontSize={11}
                    fontFamily="monospace"
                    fill={
                      room.id === selectedRoomId
                        ? "var(--accent)"
                        : "var(--text)"
                    }
                    fontWeight="bold"
                  >
                    {room.name.length > 14
                      ? room.name.slice(0, 12) + ".."
                      : room.name}
                  </text>
                  <text
                    x={room.x + ROOM_W / 2}
                    y={room.y + ROOM_H / 2 + 10}
                    textAnchor="middle"
                    fontSize={9}
                    fontFamily="monospace"
                    fill="var(--text-dim)"
                    opacity={0.7}
                  >
                    {room.mood}
                  </text>
                </g>
              ))}

              {/* Empty state */}
              {rooms.length === 0 && (
                <text
                  x="50%"
                  y="50%"
                  textAnchor="middle"
                  dominantBaseline="middle"
                  fontSize={14}
                  fontFamily="monospace"
                  fill="var(--text-dim)"
                  opacity={0.5}
                >
                  Select &quot;Add Room&quot; and click to place rooms
                </text>
              )}
            </svg>
          </div>

          {/* Room editor side panel */}
          {selectedRoom && (
            <RoomEditor
              room={selectedRoom}
              items={items}
              npcs={npcs}
              onUpdate={handleRoomUpdate}
              onAddItem={handleAddItem}
              onRemoveItem={handleRemoveItem}
              onAddNpc={handleAddNpc}
              onRemoveNpc={handleRemoveNpc}
            />
          )}
        </div>

        {/* Status bar */}
        {statusMsg && (
          <div className="border-t border-[var(--border)] px-4 py-2 text-xs text-[var(--system)]">
            {statusMsg}
          </div>
        )}

        {/* Export dialog */}
        {showExportDialog && (
          <div
            className="fixed inset-0 z-[60] flex items-center justify-center bg-black/60"
            onClick={(e) => {
              if (e.target === e.currentTarget) setShowExportDialog(false);
            }}
          >
            <div className="flex flex-col gap-3 border border-[var(--border)] bg-[var(--panel-bg)] p-6 w-80">
              <h3 className="text-sm font-bold text-[var(--accent)]">
                Export Module
              </h3>
              <label className="flex flex-col gap-1 text-xs text-[var(--text-dim)]">
                Module Name
                <input
                  type="text"
                  value={exportName}
                  onChange={(e) => setExportName(e.target.value)}
                  placeholder="My Adventure"
                  className="border border-[var(--border)] bg-[var(--bg)] px-2 py-1 text-xs text-[var(--text)]"
                  autoFocus
                  onKeyDown={(e) => {
                    if (e.key === "Enter") handleExport();
                  }}
                />
              </label>
              <div className="flex gap-2 justify-end">
                <button
                  onClick={() => setShowExportDialog(false)}
                  className="border border-[var(--border)] px-3 py-1 text-xs text-[var(--text-dim)] hover:text-[var(--text)]"
                >
                  Cancel
                </button>
                <button
                  onClick={handleExport}
                  className="border border-[var(--accent)] px-3 py-1 text-xs text-[var(--accent)] hover:bg-[var(--accent)] hover:text-[var(--bg)]"
                >
                  Export
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
