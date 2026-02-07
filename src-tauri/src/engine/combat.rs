use rand::Rng;

use crate::models::*;

pub struct CombatResult {
    pub messages: Vec<OutputLine>,
    pub action_type: ActionType,
    pub enemy_defeated: bool,
    pub player_died: bool,
    pub fled: bool,
}

fn calculate_damage(attacker_attack: i32, defender_defense: i32) -> (i32, bool) {
    let mut rng = rand::thread_rng();
    let variance = rng.gen_range(-2..=2);
    let critical = rng.gen_range(0..10) == 0; // 10% crit chance
    let base_damage = (attacker_attack - defender_defense + variance).max(1);
    let damage = if critical { base_damage * 2 } else { base_damage };
    (damage, critical)
}

fn get_player_attack(player: &Player, items: &std::collections::HashMap<String, Item>) -> i32 {
    let weapon_bonus = player
        .equipped_weapon
        .as_ref()
        .and_then(|id| items.get(id))
        .and_then(|i| i.modifier.as_ref())
        .map(|m| m.attack)
        .unwrap_or(0);
    player.attack + weapon_bonus
}

fn get_player_defense(player: &Player, items: &std::collections::HashMap<String, Item>) -> i32 {
    let armor_bonus = player
        .equipped_armor
        .as_ref()
        .and_then(|id| items.get(id))
        .and_then(|i| i.modifier.as_ref())
        .map(|m| m.defense)
        .unwrap_or(0);
    player.defense + armor_bonus
}

pub fn execute_player_attack(state: &mut WorldState) -> CombatResult {
    let mut messages = Vec::new();
    let enemy_id = match &state.game_mode {
        GameMode::InCombat(id) => id.clone(),
        _ => {
            return CombatResult {
                messages: vec![OutputLine {
                    text: "You're not in combat!".into(),
                    line_type: LineType::Error,
                }],
                action_type: ActionType::Error {
                    message: "Not in combat".into(),
                },
                enemy_defeated: false,
                player_died: false,
                fled: false,
            }
        }
    };

    let player_atk = get_player_attack(&state.player, &state.items);
    let enemy = match state.npcs.get(&enemy_id) {
        Some(e) => e.clone(),
        None => {
            state.game_mode = GameMode::Exploring;
            state.combat_state = None;
            return CombatResult {
                messages: vec![OutputLine {
                    text: "Your opponent has vanished.".into(),
                    line_type: LineType::System,
                }],
                action_type: ActionType::DisplayOnly,
                enemy_defeated: false,
                player_died: false,
                fled: false,
            };
        }
    };
    let (damage, critical) = calculate_damage(player_atk, enemy.defense);

    // Apply damage to enemy
    let new_hp = (enemy.health - damage).max(0);
    if let Some(npc) = state.npcs.get_mut(&enemy_id) {
        npc.health = new_hp;
    }

    let action_type = ActionType::CombatAttack {
        damage,
        target_name: enemy.name.clone(),
        target_hp: new_hp,
        target_max_hp: enemy.max_health,
    };

    // Log player attack
    state.combat_log.push(CombatLogEntry {
        turn: state.player.turns_elapsed,
        attacker: "Player".to_string(),
        defender: enemy.name.clone(),
        damage,
        defender_hp_after: new_hp,
        is_player_attack: true,
    });
    if state.combat_log.len() > 100 {
        state.combat_log.remove(0);
    }

    messages.push(OutputLine {
        text: crate::engine::templates::describe_combat_attack(
            "You", &enemy.name, damage, critical, new_hp,
        ),
        line_type: LineType::Combat,
    });

    // Check if enemy is dead
    if new_hp <= 0 {
        if let Some(npc) = state.npcs.get_mut(&enemy_id) {
            npc.dialogue_state = DialogueState::Dead;
            npc.hostile = false;
        }

        // Drop items
        let dropped_items: Vec<String> = state
            .npcs
            .get(&enemy_id)
            .map(|n| n.items.clone())
            .unwrap_or_default();

        if let Some(loc) = state.locations.get_mut(&state.player.location) {
            for item_id in &dropped_items {
                if !loc.items.contains(item_id) {
                    loc.items.push(item_id.clone());
                }
            }
            loc.npcs.retain(|id| id != &enemy_id);
        }

        if !dropped_items.is_empty() {
            let names: Vec<String> = dropped_items
                .iter()
                .filter_map(|id| state.items.get(id).map(|i| i.name.clone()))
                .collect();
            if !names.is_empty() {
                messages.push(OutputLine {
                    text: format!("Dropped: {}", names.join(", ")),
                    line_type: LineType::System,
                });
            }
        }

        messages.push(OutputLine {
            text: crate::engine::templates::describe_combat_victory(&enemy.name),
            line_type: LineType::Combat,
        });

        state.game_mode = GameMode::Exploring;
        state.combat_state = None;

        return CombatResult {
            messages,
            action_type: ActionType::CombatVictory {
                enemy_name: enemy.name,
            },
            enemy_defeated: true,
            player_died: false,
            fled: false,
        };
    }

    // Enemy's turn
    let player_def = get_player_defense(&state.player, &state.items);
    let (enemy_damage, enemy_crit) = calculate_damage(enemy.attack, player_def);
    state.player.health = (state.player.health - enemy_damage).max(0);

    // Log enemy attack
    state.combat_log.push(CombatLogEntry {
        turn: state.player.turns_elapsed,
        attacker: enemy.name.clone(),
        defender: "Player".to_string(),
        damage: enemy_damage,
        defender_hp_after: state.player.health,
        is_player_attack: false,
    });
    if state.combat_log.len() > 100 {
        state.combat_log.remove(0);
    }

    messages.push(OutputLine {
        text: crate::engine::templates::describe_combat_attack(
            &enemy.name,
            "you",
            enemy_damage,
            enemy_crit,
            state.player.health,
        ),
        line_type: LineType::Combat,
    });

    if state.player.health <= 0 {
        state.game_mode = GameMode::GameOver(EndingType::Death);
        state.combat_state = None;
        messages.push(OutputLine {
            text: crate::engine::templates::describe_player_death(),
            line_type: LineType::Combat,
        });
        return CombatResult {
            messages,
            action_type: ActionType::PlayerDeath,
            enemy_defeated: false,
            player_died: true,
            fled: false,
        };
    }

    if let Some(cs) = &mut state.combat_state {
        cs.turn_count += 1;
    }

    CombatResult {
        messages,
        action_type,
        enemy_defeated: false,
        player_died: false,
        fled: false,
    }
}

pub fn execute_flee(state: &mut WorldState) -> CombatResult {
    let mut messages = Vec::new();
    let mut rng = rand::thread_rng();
    let success = rng.gen_bool(0.5);

    if !success {
        messages.push(OutputLine {
            text: crate::engine::templates::describe_combat_flee(false),
            line_type: LineType::Combat,
        });

        // Enemy gets a free attack
        let enemy_id = match &state.game_mode {
            GameMode::InCombat(id) => id.clone(),
            _ => return CombatResult {
                messages,
                action_type: ActionType::CombatFlee { success: false },
                enemy_defeated: false,
                player_died: false,
                fled: false,
            },
        };

        if let Some(enemy) = state.npcs.get(&enemy_id) {
            let player_def = get_player_defense(&state.player, &state.items);
            let (damage, critical) = calculate_damage(enemy.attack, player_def);
            state.player.health = (state.player.health - damage).max(0);
            messages.push(OutputLine {
                text: crate::engine::templates::describe_combat_attack(
                    &enemy.name, "you", damage, critical, state.player.health,
                ),
                line_type: LineType::Combat,
            });
        }

        if state.player.health <= 0 {
            state.game_mode = GameMode::GameOver(EndingType::Death);
            state.combat_state = None;
            messages.push(OutputLine {
                text: crate::engine::templates::describe_player_death(),
                line_type: LineType::Combat,
            });
            return CombatResult {
                messages,
                action_type: ActionType::PlayerDeath,
                enemy_defeated: false,
                player_died: true,
                fled: false,
            };
        }

        return CombatResult {
            messages,
            action_type: ActionType::CombatFlee { success: false },
            enemy_defeated: false,
            player_died: false,
            fled: false,
        };
    }

    // Success: move to random available exit
    messages.push(OutputLine {
        text: crate::engine::templates::describe_combat_flee(true),
        line_type: LineType::Combat,
    });

    state.game_mode = GameMode::Exploring;
    state.combat_state = None;

    let current_loc = state.player.location.clone();
    if let Some(loc) = state.locations.get(&current_loc) {
        let exits: Vec<String> = loc.exits.values().cloned().collect();
        if !exits.is_empty() {
            let idx = rng.gen_range(0..exits.len());
            let new_loc = &exits[idx];
            state.player.location = new_loc.clone();
            state.player.visited_locations.insert(new_loc.clone());
            if let Some(new_location) = state.locations.get_mut(new_loc) {
                new_location.visited = true;
            }
            if let Some(new_location) = state.locations.get(new_loc) {
                messages.push(OutputLine {
                    text: format!("You flee to {}.", new_location.name),
                    line_type: LineType::System,
                });
            }
        } else {
            messages.push(OutputLine {
                text: "You break free from combat but there's nowhere to run!".into(),
                line_type: LineType::System,
            });
        }
    }

    CombatResult {
        messages,
        action_type: ActionType::CombatFlee { success: true },
        enemy_defeated: false,
        player_died: false,
        fled: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_combat_state() -> WorldState {
        let mut state = WorldState::default();
        state.locations.insert(
            "arena".into(),
            Location {
                id: "arena".into(),
                name: "Arena".into(),
                description: "A fighting pit.".into(),
                items: vec![],
                npcs: vec!["goblin".into()],
                exits: HashMap::from([(Direction::North, "exit".into())]),
                locked_exits: HashMap::new(),
                visited: true,
                discovered_secrets: vec![],
                ambient_mood: Mood::Dangerous,
            },
        );
        state.locations.insert(
            "exit".into(),
            Location {
                id: "exit".into(),
                name: "Exit".into(),
                description: "An exit.".into(),
                items: vec![],
                npcs: vec![],
                exits: HashMap::from([(Direction::South, "arena".into())]),
                locked_exits: HashMap::new(),
                visited: false,
                discovered_secrets: vec![],
                ambient_mood: Mood::Peaceful,
            },
        );
        state.npcs.insert(
            "goblin".into(),
            Npc {
                id: "goblin".into(),
                name: "Goblin".into(),
                description: "A vile goblin.".into(),
                personality_seed: String::new(),
                dialogue_state: DialogueState::Hostile,
                hostile: true,
                health: 15,
                max_health: 15,
                attack: 4,
                defense: 1,
                items: vec!["gold_coin".into()],
                quest_giver: None,
            },
        );
        state.items.insert(
            "gold_coin".into(),
            Item {
                id: "gold_coin".into(),
                name: "Gold Coin".into(),
                description: "A shiny coin.".into(),
                item_type: ItemType::Miscellaneous,
                modifier: None,
                usable: false,
                consumable: false,
                key_id: None,
            },
        );
        state.player.location = "arena".into();
        state.game_mode = GameMode::InCombat("goblin".into());
        state.combat_state = Some(CombatState {
            enemy_id: "goblin".into(),
            player_turn: true,
            turn_count: 0,
        });
        state
    }

    #[test]
    fn player_attack_deals_damage() {
        let mut state = make_combat_state();
        let initial_hp = state.npcs.get("goblin").unwrap().health;
        let result = execute_player_attack(&mut state);
        let final_hp = state.npcs.get("goblin").unwrap().health;
        assert!(final_hp < initial_hp || result.enemy_defeated);
        assert!(!result.messages.is_empty());
    }

    #[test]
    fn enemy_death_drops_items_and_exits_combat() {
        let mut state = make_combat_state();
        state.npcs.get_mut("goblin").unwrap().health = 1;
        state.npcs.get_mut("goblin").unwrap().defense = 0;
        let result = execute_player_attack(&mut state);
        assert!(result.enemy_defeated);
        assert_eq!(state.game_mode, GameMode::Exploring);
        assert!(state.combat_state.is_none());
    }
}
