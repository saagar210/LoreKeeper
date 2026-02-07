use std::collections::HashMap;

use crate::models::*;

pub struct DungeonConfig {
    pub entry_location: String,
    pub entry_direction: Direction,
    pub depth: usize,
    pub difficulty_base: i32,
}

struct RoomTemplate {
    name: &'static str,
    description: &'static str,
    mood: Mood,
}

const ROOM_TEMPLATES: &[RoomTemplate] = &[
    RoomTemplate {
        name: "Narrow Tunnel",
        description: "A cramped tunnel hewn from rough stone. Water seeps through cracks in the walls.",
        mood: Mood::Dark,
    },
    RoomTemplate {
        name: "Musty Chamber",
        description: "A large chamber with a low ceiling. Mushrooms grow in clusters along the walls.",
        mood: Mood::Mysterious,
    },
    RoomTemplate {
        name: "Collapsed Hall",
        description: "Once a grand hall, now half-buried in rubble. Dust motes dance in shafts of dim light.",
        mood: Mood::Tense,
    },
    RoomTemplate {
        name: "Flooded Passage",
        description: "Ankle-deep water fills this passage. Something ripples beneath the surface.",
        mood: Mood::Dark,
    },
    RoomTemplate {
        name: "Bone Gallery",
        description: "Walls lined with ancient bones arranged in deliberate patterns. A dark shrine stands at one end.",
        mood: Mood::Dangerous,
    },
    RoomTemplate {
        name: "Crystal Cavern",
        description: "Crystalline formations jut from every surface, casting prismatic reflections.",
        mood: Mood::Mysterious,
    },
    RoomTemplate {
        name: "Ancient Forge",
        description: "A dwarven forge, cold and silent. Tools still lie where their owners left them.",
        mood: Mood::Tense,
    },
];

pub fn generate_dungeon(config: &DungeonConfig, state: &mut WorldState) {
    // Skip if dungeon already exists (e.g. loaded from save)
    if state.locations.contains_key("dungeon_d0_r0") {
        return;
    }

    let depth = config.depth.max(2); // At least 2 rooms

    let mut prev_room_id = config.entry_location.clone();
    let mut prev_direction = config.entry_direction;

    for d in 0..depth {
        let room_id = format!("dungeon_d{d}_r{d}");
        let is_final = d == depth - 1;
        let template = &ROOM_TEMPLATES[d % ROOM_TEMPLATES.len()];

        let name = if is_final {
            "The Dungeon Heart".to_string()
        } else {
            format!("{} (Depth {})", template.name, d + 1)
        };

        let description = if is_final {
            "The deepest point of the dungeon. An oppressive aura permeates the air. \
             A massive creature guards a chest overflowing with treasures."
                .to_string()
        } else {
            template.description.to_string()
        };

        let mood = if is_final {
            Mood::Dangerous
        } else {
            template.mood
        };

        // Alternate exit direction: Down, South, Down, South...
        let next_direction = if d % 2 == 0 {
            Direction::Down
        } else {
            Direction::South
        };
        let back_direction = prev_direction.opposite();

        let mut exits = HashMap::new();
        exits.insert(back_direction, prev_room_id.clone());

        if !is_final {
            exits.insert(
                next_direction,
                format!("dungeon_d{}_r{}", d + 1, d + 1),
            );
        }

        // Items: health potion in room at index 1, treasure in final room
        let items = if is_final {
            vec!["dungeon_treasure".into()]
        } else if d == 1 {
            vec!["dungeon_health_potion".into()]
        } else {
            vec![]
        };

        // NPCs: guard at midpoint, boss at final room
        let npcs = if is_final {
            vec!["dungeon_boss".into()]
        } else if d == depth / 2 {
            vec![format!("dungeon_guard_{d}")]
        } else {
            vec![]
        };

        let location = Location {
            id: room_id.clone(),
            name,
            description,
            items,
            npcs: npcs.clone(),
            exits,
            locked_exits: HashMap::new(),
            visited: false,
            discovered_secrets: vec![],
            ambient_mood: mood,
            examine_details: None,
            revisit_description: None,
        };

        state.locations.insert(room_id.clone(), location);

        // Create mid-dungeon guard NPC
        if !is_final && d == depth / 2 {
            let guard_id = format!("dungeon_guard_{d}");
            let guard_attack = config.difficulty_base + (d as i32 * 2);
            let guard_health = 15 + (d as i32 * 5);
            state.npcs.insert(
                guard_id.clone(),
                Npc {
                    id: guard_id,
                    name: "Dungeon Lurker".into(),
                    description: "A twisted creature adapted to the darkness. Its pale eyes gleam with hunger.".into(),
                    personality_seed: "Hostile. Attacks on sight. Protects its territory.".into(),
                    dialogue_state: DialogueState::Hostile,
                    hostile: true,
                    health: guard_health,
                    max_health: guard_health,
                    attack: guard_attack,
                    defense: config.difficulty_base / 2,
                    items: vec![],
                    quest_giver: None,
                    examine_text: None,
                    relationship: 0,
                    memory: vec![],
                },
            );
        }

        // Link previous room to this one
        if let Some(prev_loc) = state.locations.get_mut(&prev_room_id) {
            prev_loc.exits.insert(prev_direction, room_id.clone());
        }

        prev_room_id = room_id;
        prev_direction = next_direction;
    }

    // Create boss NPC
    let boss_health = 30 + (config.difficulty_base * 2);
    let boss_attack = config.difficulty_base + 8;
    state.npcs.insert(
        "dungeon_boss".into(),
        Npc {
            id: "dungeon_boss".into(),
            name: "The Dungeon Keeper".into(),
            description: "A massive, armored beast with eyes like molten gold. \
                          It guards its hoard with primal fury."
                .into(),
            personality_seed: "Territorial and primal. Roars before attacking. Ancient and powerful."
                .into(),
            dialogue_state: DialogueState::Hostile,
            hostile: true,
            health: boss_health,
            max_health: boss_health,
            attack: boss_attack,
            defense: config.difficulty_base,
            items: vec!["dungeon_key_shard".into()],
            quest_giver: None,
            examine_text: Some(
                "Scars criss-cross its thick hide. A crown of twisted metal sits upon its head \
                 — perhaps it was once something more."
                    .into(),
            ),
            relationship: 0,
            memory: vec![],
        },
    );

    // Create dungeon items
    state.items.insert(
        "dungeon_treasure".into(),
        Item {
            id: "dungeon_treasure".into(),
            name: "Dungeon Treasure".into(),
            description: "A chest of ancient gold coins, gemstones, and a mysterious crystal shard."
                .into(),
            item_type: ItemType::Quest,
            modifier: Some(StatModifier {
                attack: 2,
                defense: 2,
                health: 0,
            }),
            usable: false,
            consumable: false,
            key_id: None,
            lore: Some(
                "The accumulated wealth of centuries, guarded by a creature that long forgot why it hoards."
                    .into(),
            ),
        },
    );

    state.items.insert(
        "dungeon_health_potion".into(),
        Item {
            id: "dungeon_health_potion".into(),
            name: "Glowing Elixir".into(),
            description: "A vial of luminescent liquid found deep underground. It radiates warmth."
                .into(),
            item_type: ItemType::Consumable,
            modifier: Some(StatModifier {
                attack: 0,
                defense: 0,
                health: 25,
            }),
            usable: true,
            consumable: true,
            key_id: None,
            lore: None,
        },
    );

    state.items.insert(
        "dungeon_key_shard".into(),
        Item {
            id: "dungeon_key_shard".into(),
            name: "Key Shard".into(),
            description: "A fragment of an ancient key. It hums with residual magic.".into(),
            item_type: ItemType::Miscellaneous,
            modifier: None,
            usable: false,
            consumable: false,
            key_id: None,
            lore: Some(
                "Part of the original key to Thornhold's deepest vault. \
                 Whoever carried it was consumed by what they guarded."
                    .into(),
            ),
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::world_builder;

    fn build_state_with_dungeon(difficulty_base: i32, depth: usize) -> WorldState {
        let mut state = world_builder::build_thornhold();
        generate_dungeon(
            &DungeonConfig {
                entry_location: "armory".to_string(),
                entry_direction: Direction::Down,
                depth,
                difficulty_base,
            },
            &mut state,
        );
        state
    }

    #[test]
    fn generates_correct_number_of_rooms() {
        let state = build_state_with_dungeon(5, 5);
        let dungeon_rooms: Vec<_> = state
            .locations
            .keys()
            .filter(|k| k.starts_with("dungeon_"))
            .collect();
        assert_eq!(dungeon_rooms.len(), 5);
    }

    #[test]
    fn armory_has_down_exit_to_dungeon() {
        let state = build_state_with_dungeon(5, 5);
        let armory = state.locations.get("armory").unwrap();
        assert_eq!(
            armory.exits.get(&Direction::Down),
            Some(&"dungeon_d0_r0".to_string())
        );
    }

    #[test]
    fn first_room_links_back_to_armory() {
        let state = build_state_with_dungeon(5, 5);
        let first = state.locations.get("dungeon_d0_r0").unwrap();
        assert_eq!(
            first.exits.get(&Direction::Up),
            Some(&"armory".to_string())
        );
    }

    #[test]
    fn rooms_are_connected_sequentially() {
        let state = build_state_with_dungeon(5, 5);
        // Walk from armory through each dungeon room.
        // Forward directions follow the generator pattern:
        //   armory->d0: Down, d0->d1: Down, d1->d2: South, d2->d3: Down, d3->d4: South
        let mut current = "armory".to_string();
        let forward_dirs = [
            Direction::Down,  // armory -> d0
            Direction::Down,  // d0 -> d1
            Direction::South, // d1 -> d2
            Direction::Down,  // d2 -> d3
            Direction::South, // d3 -> d4
        ];

        for (d, direction) in forward_dirs.iter().enumerate() {
            let loc = state.locations.get(&current).unwrap();
            let next_id = loc
                .exits
                .get(direction)
                .unwrap_or_else(|| panic!("No {direction:?} exit from {current}"));
            let expected = format!("dungeon_d{d}_r{d}");
            assert_eq!(*next_id, expected, "Room {d} not connected from {current}");
            current = next_id.clone();
        }
    }

    #[test]
    fn final_room_is_dungeon_heart() {
        let state = build_state_with_dungeon(5, 5);
        let final_room = state.locations.get("dungeon_d4_r4").unwrap();
        assert_eq!(final_room.name, "The Dungeon Heart");
        assert_eq!(final_room.ambient_mood, Mood::Dangerous);
    }

    #[test]
    fn final_room_has_no_forward_exit() {
        let state = build_state_with_dungeon(5, 5);
        let final_room = state.locations.get("dungeon_d4_r4").unwrap();
        // Should only have the back exit, no forward
        assert_eq!(final_room.exits.len(), 1);
    }

    #[test]
    fn boss_npc_exists_at_final_room() {
        let state = build_state_with_dungeon(5, 5);
        let final_room = state.locations.get("dungeon_d4_r4").unwrap();
        assert!(final_room.npcs.contains(&"dungeon_boss".to_string()));

        let boss = state.npcs.get("dungeon_boss").unwrap();
        assert_eq!(boss.name, "The Dungeon Keeper");
        assert!(boss.hostile);
    }

    #[test]
    fn mid_dungeon_guard_exists() {
        let state = build_state_with_dungeon(5, 5);
        // depth / 2 = 2, so guard at dungeon_d2_r2
        let mid_room = state.locations.get("dungeon_d2_r2").unwrap();
        assert!(mid_room.npcs.contains(&"dungeon_guard_2".to_string()));

        let guard = state.npcs.get("dungeon_guard_2").unwrap();
        assert_eq!(guard.name, "Dungeon Lurker");
        assert!(guard.hostile);
    }

    #[test]
    fn difficulty_scaling_easy() {
        let state = build_state_with_dungeon(3, 5);
        let boss = state.npcs.get("dungeon_boss").unwrap();
        assert_eq!(boss.health, 30 + 6); // 30 + 3*2
        assert_eq!(boss.attack, 3 + 8);
        assert_eq!(boss.defense, 3);
    }

    #[test]
    fn difficulty_scaling_hard() {
        let state = build_state_with_dungeon(8, 5);
        let boss = state.npcs.get("dungeon_boss").unwrap();
        assert_eq!(boss.health, 30 + 16); // 30 + 8*2
        assert_eq!(boss.attack, 8 + 8);
        assert_eq!(boss.defense, 8);
    }

    #[test]
    fn dungeon_items_created() {
        let state = build_state_with_dungeon(5, 5);
        assert!(state.items.contains_key("dungeon_treasure"));
        assert!(state.items.contains_key("dungeon_health_potion"));
        assert!(state.items.contains_key("dungeon_key_shard"));

        let potion = state.items.get("dungeon_health_potion").unwrap();
        assert!(potion.consumable);
        assert!(potion.usable);
        assert_eq!(potion.item_type, ItemType::Consumable);
    }

    #[test]
    fn health_potion_in_second_room() {
        let state = build_state_with_dungeon(5, 5);
        let room = state.locations.get("dungeon_d1_r1").unwrap();
        assert!(room.items.contains(&"dungeon_health_potion".to_string()));
    }

    #[test]
    fn treasure_in_final_room() {
        let state = build_state_with_dungeon(5, 5);
        let final_room = state.locations.get("dungeon_d4_r4").unwrap();
        assert!(final_room.items.contains(&"dungeon_treasure".to_string()));
    }

    #[test]
    fn skips_if_already_generated() {
        let mut state = world_builder::build_thornhold();
        let config = DungeonConfig {
            entry_location: "armory".to_string(),
            entry_direction: Direction::Down,
            depth: 5,
            difficulty_base: 5,
        };
        generate_dungeon(&config, &mut state);
        let count_before = state.locations.len();

        // Call again — should be a no-op
        generate_dungeon(&config, &mut state);
        assert_eq!(state.locations.len(), count_before);
    }

    #[test]
    fn minimum_depth_clamped_to_two() {
        let state = build_state_with_dungeon(5, 1);
        let dungeon_rooms: Vec<_> = state
            .locations
            .keys()
            .filter(|k| k.starts_with("dungeon_"))
            .collect();
        assert_eq!(dungeon_rooms.len(), 2);
    }

    #[test]
    fn existing_locations_preserved() {
        let state = build_state_with_dungeon(5, 5);
        // Original 14 + 5 dungeon rooms = 19
        assert_eq!(state.locations.len(), 19);
        assert!(state.locations.contains_key("courtyard"));
        assert!(state.locations.contains_key("great_hall"));
        assert!(state.locations.contains_key("armory"));
    }
}
