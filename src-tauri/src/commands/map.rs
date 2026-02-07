use serde::Serialize;
use tauri::State;

use crate::models::WorldState;
use crate::persistence::state::GameState;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MapNode {
    pub id: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub visited: bool,
    pub current: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MapEdge {
    pub from: String,
    pub to: String,
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MapData {
    pub nodes: Vec<MapNode>,
    pub edges: Vec<MapEdge>,
}

/// Hand-tuned x/y positions for the 13 Thornhold locations.
/// Coordinate space: 0-280 x, 0-400 y (fits sidebar width).
fn get_position(id: &str) -> (f32, f32) {
    match id {
        "courtyard"       => (40.0, 40.0),
        "great_hall"      => (140.0, 40.0),
        "tower_apex"      => (140.0, 0.0),      // above great hall (up)
        "library"         => (240.0, 40.0),
        "barracks"        => (40.0, 120.0),
        "kitchen"         => (140.0, 120.0),
        "chapel"          => (240.0, 120.0),
        "armory"          => (40.0, 200.0),
        "cellar_entrance" => (140.0, 200.0),
        "wine_cellar"     => (140.0, 260.0),
        "crypt_passage"   => (200.0, 260.0),
        "deep_chamber"    => (200.0, 320.0),
        "final_sanctum"   => (200.0, 380.0),
        _                 => (140.0, 200.0),
    }
}

#[tauri::command]
pub fn get_map_data(game_state: State<GameState>) -> Result<MapData, String> {
    let state = game_state.0.lock().map_err(|e| e.to_string())?;
    Ok(build_map_data(&state))
}

fn build_map_data(state: &WorldState) -> MapData {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut seen_edges = std::collections::HashSet::new();

    for (id, loc) in &state.locations {
        let (x, y) = get_position(id);
        nodes.push(MapNode {
            id: id.clone(),
            name: loc.name.clone(),
            x,
            y,
            visited: loc.visited,
            current: state.player.location == *id,
        });

        for dest_id in loc.exits.values() {
            let edge_key = if id < dest_id {
                (id.clone(), dest_id.clone())
            } else {
                (dest_id.clone(), id.clone())
            };

            if seen_edges.insert(edge_key) {
                let locked = loc
                    .locked_exits
                    .iter()
                    .any(|(dir, _)| loc.exits.get(dir) == Some(dest_id));

                edges.push(MapEdge {
                    from: id.clone(),
                    to: dest_id.clone(),
                    locked,
                });
            }
        }
    }

    nodes.sort_by(|a, b| a.id.cmp(&b.id));
    MapData { nodes, edges }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::world_builder;

    #[test]
    fn map_data_has_all_locations() {
        let state = world_builder::build_thornhold();
        let data = build_map_data(&state);
        assert_eq!(data.nodes.len(), 13);
    }

    #[test]
    fn map_data_has_edges() {
        let state = world_builder::build_thornhold();
        let data = build_map_data(&state);
        assert!(!data.edges.is_empty());
    }

    #[test]
    fn current_location_marked() {
        let state = world_builder::build_thornhold();
        let data = build_map_data(&state);
        let current = data.nodes.iter().find(|n| n.current).unwrap();
        assert_eq!(current.id, "courtyard");
    }
}
