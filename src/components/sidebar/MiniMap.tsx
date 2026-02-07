import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import type { Location, MapData, Player } from "../../store/types";

interface Props {
  locations: Record<string, Location>;
  player: Player;
}

export function MiniMap({ locations, player }: Props) {
  const [mapData, setMapData] = useState<MapData | null>(null);

  useEffect(() => {
    let ignore = false;
    invoke<MapData>("get_map_data")
      .then((data) => {
        if (!ignore) setMapData(data);
      })
      .catch(() => {
        if (!ignore) setMapData(null);
      });
    return () => {
      ignore = true;
    };
  }, [player.location, Object.keys(locations).length]);

  if (!mapData) {
    return (
      <div>
        <h3 className="mb-2 font-bold text-[var(--accent)]">Map</h3>
        <p className="text-xs text-[var(--text-dim)]">Loading...</p>
      </div>
    );
  }

  // Build a lookup for node positions
  const nodeMap = new Map(mapData.nodes.map((n) => [n.id, n]));

  // SVG dimensions to fit the data (dynamic height for dungeon rooms)
  const padding = 20;
  const nodeRadius = 8;
  const svgWidth = 280;
  const maxY = Math.max(...mapData.nodes.map((n) => n.y), 400);
  const svgHeight = maxY + 40;

  return (
    <div>
      <h3 className="mb-2 font-bold text-[var(--accent)]">Map</h3>
      <svg
        width={svgWidth}
        height={svgHeight}
        viewBox={`${-padding} ${-padding} ${svgWidth} ${svgHeight}`}
        className="w-full"
        role="img"
        aria-label="Game map"
      >
        {/* Edges */}
        {mapData.edges.map((edge) => {
          const from = nodeMap.get(edge.from);
          const to = nodeMap.get(edge.to);
          if (!from || !to) return null;
          // Only show edges where at least one end is visited
          if (!from.visited && !to.visited) return null;
          return (
            <line
              key={`${edge.from}-${edge.to}`}
              x1={from.x}
              y1={from.y}
              x2={to.x}
              y2={to.y}
              stroke={edge.locked ? "var(--error)" : "var(--border)"}
              strokeWidth={1.5}
              strokeDasharray={edge.locked ? "4 2" : undefined}
              opacity={0.6}
            />
          );
        })}
        {/* Nodes */}
        {mapData.nodes.map((node) => {
          // Only show visited nodes or nodes adjacent to visited
          if (!node.visited && !node.current) {
            const isAdjacent = mapData.edges.some(
              (e) =>
                (e.from === node.id &&
                  nodeMap.get(e.to)?.visited) ||
                (e.to === node.id &&
                  nodeMap.get(e.from)?.visited),
            );
            if (!isAdjacent) return null;
          }

          return (
            <g key={node.id}>
              <circle
                cx={node.x}
                cy={node.y}
                r={node.current ? nodeRadius + 2 : nodeRadius}
                fill={
                  node.current
                    ? "var(--accent)"
                    : node.visited
                      ? "var(--text-dim)"
                      : "none"
                }
                stroke={
                  node.current
                    ? "var(--accent)"
                    : node.visited
                      ? "var(--text-dim)"
                      : "var(--border)"
                }
                strokeWidth={1.5}
                opacity={node.visited || node.current ? 1 : 0.4}
              />
              {(node.current || node.visited) && (
                <text
                  x={node.x}
                  y={node.y + nodeRadius + 12}
                  textAnchor="middle"
                  fill={node.current ? "var(--accent)" : "var(--text-dim)"}
                  fontSize={8}
                  fontFamily="monospace"
                  fontWeight={node.current ? "bold" : "normal"}
                >
                  {node.name.length > 15
                    ? node.name.slice(0, 14) + "..."
                    : node.name}
                </text>
              )}
            </g>
          );
        })}
      </svg>
    </div>
  );
}
