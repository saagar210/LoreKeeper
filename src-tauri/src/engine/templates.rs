use crate::models::*;
use std::collections::HashMap;

pub fn describe_location(
    location: &Location,
    items: &HashMap<String, Item>,
    npcs: &HashMap<String, Npc>,
    first_visit: bool,
) -> Vec<String> {
    let mut lines = Vec::new();

    if first_visit {
        lines.push(format!("--- {} ---", location.name));
        lines.push(location.description.clone());
    } else {
        lines.push(format!("--- {} (revisited) ---", location.name));
        if let Some(revisit) = &location.revisit_description {
            lines.push(revisit.clone());
        } else {
            lines.push(location.description.clone());
        }
    }

    // Items on the ground
    let item_names: Vec<String> = location
        .items
        .iter()
        .filter_map(|id| items.get(id).map(|i| i.name.clone()))
        .collect();
    if !item_names.is_empty() {
        lines.push(format!("You see: {}", item_names.join(", ")));
    }

    // NPCs present
    let npc_names: Vec<String> = location
        .npcs
        .iter()
        .filter_map(|id| {
            npcs.get(id).map(|n| {
                if n.dialogue_state != DialogueState::Dead {
                    n.name.clone()
                } else {
                    format!("{} (dead)", n.name)
                }
            })
        })
        .collect();
    if !npc_names.is_empty() {
        lines.push(format!("Present: {}", npc_names.join(", ")));
    }

    // Exits
    let mut exit_strs: Vec<String> = location
        .exits
        .keys()
        .map(|d| {
            if location.locked_exits.contains_key(d) {
                format!("{} (locked)", d.display_name())
            } else {
                d.display_name().to_string()
            }
        })
        .collect();
    exit_strs.sort();
    if !exit_strs.is_empty() {
        lines.push(format!("Exits: {}", exit_strs.join(", ")));
    }

    lines
}

pub fn describe_take(item_name: &str) -> String {
    format!("You pick up the {}.", item_name)
}

pub fn describe_drop(item_name: &str) -> String {
    format!("You drop the {}.", item_name)
}

pub fn describe_use(item_name: &str, effect: &str) -> String {
    format!("You use the {}. {}", item_name, effect)
}

pub fn describe_equip(item_name: &str) -> String {
    format!("You equip the {}.", item_name)
}

pub fn describe_unequip(item_name: &str) -> String {
    format!("You unequip the {}.", item_name)
}

pub fn describe_combat_attack(
    attacker: &str,
    defender: &str,
    damage: i32,
    critical: bool,
    remaining_hp: i32,
) -> String {
    if critical {
        format!(
            "CRITICAL HIT! {} strikes {} for {} damage! ({} HP remaining)",
            attacker, defender, damage, remaining_hp
        )
    } else {
        format!(
            "{} attacks {} for {} damage. ({} HP remaining)",
            attacker, defender, damage, remaining_hp
        )
    }
}

pub fn describe_combat_victory(enemy_name: &str) -> String {
    format!("{} has been defeated!", enemy_name)
}

pub fn describe_combat_flee(success: bool) -> String {
    if success {
        "You manage to escape!".to_string()
    } else {
        "You fail to escape!".to_string()
    }
}

pub fn describe_player_death() -> String {
    "You collapse to the ground. Darkness claims you...".to_string()
}

pub fn describe_npc_dialogue(npc: &Npc) -> String {
    match npc.dialogue_state {
        DialogueState::Greeting | DialogueState::Familiar => {
            if npc.relationship > 30 {
                format!(
                    "{} greets you warmly. \"Welcome back, friend!\"",
                    npc.name
                )
            } else if npc.relationship < -30 {
                format!("{} regards you with suspicion.", npc.name)
            } else if npc.dialogue_state == DialogueState::Greeting {
                format!(
                    "{} regards you with interest. \"Greetings, traveler.\"",
                    npc.name
                )
            } else {
                format!("{} nods in recognition. \"We meet again.\"", npc.name)
            }
        }
        DialogueState::QuestOffered => {
            format!(
                "{} leans forward. \"I have a task for you, if you're willing...\"",
                npc.name
            )
        }
        DialogueState::QuestActive => {
            format!(
                "{} asks, \"Have you completed what I asked of you?\"",
                npc.name
            )
        }
        DialogueState::QuestComplete => {
            format!(
                "{} smiles broadly. \"You've done it! Here is your reward.\"",
                npc.name
            )
        }
        DialogueState::Hostile => {
            format!("{} snarls and lunges at you!", npc.name)
        }
        DialogueState::Dead => "The body lies still.".to_string(),
    }
}

pub fn describe_quest_started(quest: &Quest) -> String {
    format!("New Quest: {} — {}", quest.name, quest.description)
}

pub fn describe_quest_completed(quest: &Quest) -> String {
    format!("Quest Complete: {}!", quest.name)
}

pub fn describe_event_message(message: &str) -> String {
    message.to_string()
}

pub fn describe_event_damage(amount: i32) -> String {
    format!("You take {} damage!", amount)
}

pub fn describe_inventory(player: &Player, items: &HashMap<String, Item>) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "--- Inventory ({}/{}) ---",
        player.inventory.len(),
        player.max_inventory
    ));

    if player.inventory.is_empty() {
        lines.push("Your inventory is empty.".to_string());
    } else {
        for item_id in &player.inventory {
            if let Some(item) = items.get(item_id) {
                let mut desc = item.name.clone();
                if player.equipped_weapon.as_ref() == Some(item_id) {
                    desc.push_str(" (wielded)");
                } else if player.equipped_armor.as_ref() == Some(item_id) {
                    desc.push_str(" (worn)");
                }
                lines.push(format!("  - {}", desc));
            }
        }
    }
    lines
}

pub fn describe_stats(player: &Player, items: &HashMap<String, Item>) -> Vec<String> {
    let mut lines = Vec::new();
    let weapon_bonus = player
        .equipped_weapon
        .as_ref()
        .and_then(|id| items.get(id))
        .and_then(|i| i.modifier.as_ref())
        .map(|m| m.attack)
        .unwrap_or(0);
    let armor_bonus = player
        .equipped_armor
        .as_ref()
        .and_then(|id| items.get(id))
        .and_then(|i| i.modifier.as_ref())
        .map(|m| m.defense)
        .unwrap_or(0);

    lines.push(format!("HP: {}/{}", player.health, player.max_health));
    lines.push(format!(
        "Attack: {} (+{})",
        player.attack + weapon_bonus,
        weapon_bonus
    ));
    lines.push(format!(
        "Defense: {} (+{})",
        player.defense + armor_bonus,
        armor_bonus
    ));
    lines.push(format!("Turns: {}", player.turns_elapsed));
    lines
}

pub fn describe_map(
    locations: &HashMap<String, Location>,
    player: &Player,
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push("--- Map ---".to_string());

    for loc_id in &player.visited_locations {
        if let Some(loc) = locations.get(loc_id) {
            let marker = if player.location == *loc_id {
                " <-- You are here"
            } else {
                ""
            };
            let mut exits: Vec<String> = loc.exits.keys().map(|d| d.display_name().to_string()).collect();
            exits.sort();
            lines.push(format!("  {} (exits: {}){}", loc.name, exits.join(", "), marker));
        }
    }
    lines
}

pub fn describe_cant_go(direction: &Direction) -> String {
    format!("You can't go {} from here.", direction.display_name())
}

pub fn describe_not_found(target: &str) -> String {
    format!("You don't see '{}' here.", target)
}

pub fn describe_inventory_full() -> String {
    "Your inventory is full!".to_string()
}

pub fn describe_locked_door(direction: &Direction) -> String {
    format!(
        "The way {} is locked. You need a key.",
        direction.display_name()
    )
}

pub fn describe_door_unlocked(direction: &Direction, key_name: &str) -> String {
    format!(
        "You use the {} to unlock the way {}.",
        key_name,
        direction.display_name()
    )
}

pub fn describe_examine_item(item: &Item) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("--- {} ---", item.name));
    lines.push(item.description.clone());
    if let Some(modifier) = &item.modifier {
        let mut stats = Vec::new();
        if modifier.attack != 0 {
            stats.push(format!("Attack {:+}", modifier.attack));
        }
        if modifier.defense != 0 {
            stats.push(format!("Defense {:+}", modifier.defense));
        }
        if modifier.health != 0 {
            stats.push(format!("Health {:+}", modifier.health));
        }
        if !stats.is_empty() {
            lines.push(format!("Stats: {}", stats.join(", ")));
        }
    }
    if let Some(lore) = &item.lore {
        lines.push(format!("Lore: {}", lore));
    }
    lines
}

pub fn describe_examine_npc(npc: &Npc) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("--- {} ---", npc.name));
    if let Some(examine_text) = &npc.examine_text {
        lines.push(examine_text.clone());
    } else {
        lines.push(npc.description.clone());
    }
    if npc.dialogue_state != DialogueState::Dead {
        lines.push(format!("Health: {}/{}", npc.health, npc.max_health));
    } else {
        lines.push("Dead.".to_string());
    }
    lines
}

pub fn describe_examine_room(location: &Location) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("--- {} (detailed) ---", location.name));
    if let Some(details) = &location.examine_details {
        lines.push(details.clone());
    } else {
        lines.push(location.description.clone());
    }
    lines
}

pub fn describe_ambiguous_target(matches: &[String]) -> String {
    format!("Which one? {}", matches.join(", "))
}

pub fn describe_help(game_mode: &GameMode) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push("--- Help ---".to_string());

    match game_mode {
        GameMode::InCombat(_) => {
            lines.push("Combat commands:".to_string());
            lines.push("  attack        - Attack the enemy".to_string());
            lines.push("  use <item>    - Use an item".to_string());
            lines.push("  flee          - Try to escape".to_string());
            lines.push("  inventory     - Check your items".to_string());
        }
        GameMode::InDialogue(_) => {
            lines.push("Dialogue mode:".to_string());
            lines.push("  Type your response to speak".to_string());
            lines.push("  leave/goodbye - End conversation".to_string());
            lines.push("  inventory     - Check your items".to_string());
        }
        _ => {
            lines.push("Movement:  go <direction>, north/south/east/west/up/down".to_string());
            lines.push("Look:      look, examine <target>".to_string());
            lines.push("Items:     take/drop/use/equip/unequip <item>".to_string());
            lines.push("Interact:  talk to <npc>, attack <target>".to_string());
            lines.push("Info:      inventory, map, quests, help".to_string());
            lines.push("Game:      save [name], load [name]".to_string());
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_location_basic() {
        let loc = Location {
            id: "test".into(),
            name: "Test Room".into(),
            description: "A small room.".into(),
            items: vec!["sword".into()],
            npcs: vec!["guard".into()],
            exits: HashMap::from([(Direction::North, "hall".into())]),
            locked_exits: HashMap::new(),
            visited: false,
            discovered_secrets: vec![],
            ambient_mood: Mood::Peaceful,
                examine_details: None,
                revisit_description: None,
        };
        let mut items = HashMap::new();
        items.insert(
            "sword".into(),
            Item {
                id: "sword".into(),
                name: "Short Sword".into(),
                description: "A blade.".into(),
                item_type: ItemType::Weapon,
                modifier: None,
                usable: false,
                consumable: false,
                key_id: None,
                lore: None,
            },
        );
        let mut npcs = HashMap::new();
        npcs.insert(
            "guard".into(),
            Npc {
                id: "guard".into(),
                name: "Guard".into(),
                description: "A guard.".into(),
                personality_seed: String::new(),
                dialogue_state: DialogueState::Greeting,
                hostile: false,
                health: 20,
                max_health: 20,
                attack: 5,
                defense: 3,
                items: vec![],
                quest_giver: None,
                examine_text: None,
                relationship: 0,
                memory: vec![],
            },
        );

        let lines = describe_location(&loc, &items, &npcs, true);
        assert!(lines[0].contains("Test Room"));
        assert!(lines[1].contains("A small room"));
        assert!(lines.iter().any(|l| l.contains("Short Sword")));
        assert!(lines.iter().any(|l| l.contains("Guard")));
        assert!(lines.iter().any(|l| l.contains("North")));
    }

    #[test]
    fn describe_help_modes() {
        let exploring = describe_help(&GameMode::Exploring);
        assert!(exploring.iter().any(|l| l.contains("Movement")));

        let combat = describe_help(&GameMode::InCombat("enemy".into()));
        assert!(combat.iter().any(|l| l.contains("Combat")));

        let dialogue = describe_help(&GameMode::InDialogue("npc".into()));
        assert!(dialogue.iter().any(|l| l.contains("Dialogue")));
    }
}
