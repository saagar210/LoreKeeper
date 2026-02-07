use crate::engine::{combat, crafting, dialogue, events, parser::GameCommand, quest, templates};
use crate::models::*;

fn build_narrative_context(
    action_type: &ActionType,
    state: &WorldState,
) -> Option<NarrativeContext> {
    let loc = state.locations.get(&state.player.location)?;
    let inventory_names: Vec<String> = state
        .player
        .inventory
        .iter()
        .filter_map(|id| state.items.get(id).map(|i| i.name.clone()))
        .collect();
    let room_item_names: Vec<String> = loc
        .items
        .iter()
        .filter_map(|id| state.items.get(id).map(|i| i.name.clone()))
        .collect();
    let room_npc_names: Vec<String> = loc
        .npcs
        .iter()
        .filter_map(|id| state.npcs.get(id).map(|n| n.name.clone()))
        .collect();

    Some(NarrativeContext {
        location_name: loc.name.clone(),
        location_description: loc.description.clone(),
        mood: format!("{:?}", loc.ambient_mood),
        player_health: state.player.health,
        player_max_health: state.player.max_health,
        inventory_names,
        room_item_names,
        room_npc_names,
        action_type: action_type.clone(),
        turns_elapsed: state.player.turns_elapsed,
    })
}

fn fuzzy_match_item<'a>(
    target: &str,
    available_ids: &'a [String],
    items: &'a std::collections::HashMap<String, Item>,
) -> Vec<(&'a str, &'a str)> {
    let target_lower = target.to_lowercase();
    available_ids
        .iter()
        .filter_map(|id| {
            items.get(id).and_then(|item| {
                let name_lower = item.name.to_lowercase();
                let id_lower = id.to_lowercase();
                if name_lower == target_lower
                    || id_lower == target_lower
                    || name_lower.contains(&target_lower)
                    || id_lower.contains(&target_lower)
                {
                    Some((id.as_str(), item.name.as_str()))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn fuzzy_match_npc<'a>(
    target: &str,
    available_ids: &'a [String],
    npcs: &'a std::collections::HashMap<String, Npc>,
) -> Option<(&'a str, &'a str)> {
    let target_lower = target.to_lowercase();
    available_ids.iter().find_map(|id| {
        npcs.get(id).and_then(|npc| {
            let name_lower = npc.name.to_lowercase();
            let id_lower = id.to_lowercase();
            if name_lower == target_lower
                || id_lower == target_lower
                || name_lower.contains(&target_lower)
                || id_lower.contains(&target_lower)
            {
                Some((id.as_str(), npc.name.as_str()))
            } else {
                None
            }
        })
    })
}

pub fn execute(command: GameCommand, state: &mut WorldState) -> ActionResult {
    match command {
        GameCommand::Look(target) => execute_look(target, state),
        GameCommand::Go(direction) => execute_go(direction, state),
        GameCommand::Take(target) => execute_take(&target, state),
        GameCommand::Drop(target) => execute_drop(&target, state),
        GameCommand::Use(target) => execute_use(&target, state),
        GameCommand::Equip(target) => execute_equip(&target, state),
        GameCommand::Unequip(target) => execute_unequip(&target, state),
        GameCommand::TalkTo(target) => execute_talk(&target, state),
        GameCommand::Attack(target) => execute_attack(&target, state),
        GameCommand::Flee => execute_flee(state),
        GameCommand::Inventory => execute_inventory(state),
        GameCommand::Map => execute_map(state),
        GameCommand::QuestLog => execute_quest_log(state),
        GameCommand::Journal => execute_journal(state),
        GameCommand::Craft(first, second) => {
            crafting::execute_craft(&first, second.as_deref(), state)
        }
        GameCommand::Secret(word) => execute_secret(&word, state),
        GameCommand::Help => execute_help(state),
        GameCommand::Save(_) | GameCommand::Load(_) => {
            // Handled at command layer
            ActionResult {
                messages: vec![],
                action_type: ActionType::DisplayOnly,
                narrative_context: None,
                sound_cues: vec![],
            }
        }
        GameCommand::Unknown(msg) => {
            // In dialogue mode, process as dialogue input
            if let GameMode::InDialogue(npc_id) = &state.game_mode {
                let npc_id = npc_id.clone();
                return execute_dialogue_input(&msg, &npc_id, state);
            }
            let text = if msg.is_empty() {
                "What would you like to do?".to_string()
            } else {
                format!("I don't understand '{}'.", msg)
            };
            ActionResult {
                messages: vec![OutputLine {
                    text,
                    line_type: LineType::Error,
                }],
                action_type: ActionType::Error { message: msg },
                narrative_context: None,
                sound_cues: vec![],
            }
        }
    }
}

fn execute_look(target: Option<String>, state: &mut WorldState) -> ActionResult {
    let loc = match state.locations.get(&state.player.location) {
        Some(l) => l.clone(),
        None => {
            return ActionResult {
                messages: vec![OutputLine {
                    text: "You are nowhere.".into(),
                    line_type: LineType::Error,
                }],
                action_type: ActionType::Error {
                    message: "Invalid location".into(),
                },
                narrative_context: None,
                sound_cues: vec![],
            }
        }
    };

    match target {
        None => {
            let lines = templates::describe_location(&loc, &state.items, &state.npcs, !loc.visited);
            let messages: Vec<OutputLine> = lines
                .into_iter()
                .map(|text| OutputLine {
                    text,
                    line_type: LineType::Narration,
                })
                .collect();
            let action_type = ActionType::RoomEntered {
                first_visit: !loc.visited,
            };
            let ctx = build_narrative_context(&action_type, state);
            ActionResult {
                messages,
                action_type,
                narrative_context: ctx,
                sound_cues: vec![],
            }
        }
        Some(target) => {
            // Check for room examine keywords
            if matches!(target.as_str(), "room" | "around" | "here" | "area" | "surroundings") {
                let lines = templates::describe_examine_room(&loc);
                return ActionResult {
                    messages: lines.into_iter().map(|text| OutputLine { text, line_type: LineType::Narration }).collect(),
                    action_type: ActionType::DisplayOnly,
                    narrative_context: None,
                sound_cues: vec![],
                };
            }

            // Search items in room, inventory, then NPCs
            let room_items = &loc.items;
            let matches = fuzzy_match_item(&target, room_items, &state.items);
            if let Some(&(id, _)) = matches.first() {
                if let Some(item) = state.items.get(id).cloned() {
                    let lines = templates::describe_examine_item(&item);
                    if item.lore.is_some() {
                        add_journal_entry(state, &format!("item_{}", id), JournalCategory::Item, &item.name, item.lore.as_deref().unwrap_or(&item.description));
                    }
                    return ActionResult {
                        messages: lines.into_iter().map(|text| OutputLine { text, line_type: LineType::Narration }).collect(),
                        action_type: ActionType::DisplayOnly,
                        narrative_context: None,
                        sound_cues: vec![],
                    };
                }
            }

            let inv_matches = fuzzy_match_item(&target, &state.player.inventory, &state.items);
            if let Some(&(id, _)) = inv_matches.first() {
                if let Some(item) = state.items.get(id).cloned() {
                    let lines = templates::describe_examine_item(&item);
                    if item.lore.is_some() {
                        add_journal_entry(state, &format!("item_{}", id), JournalCategory::Item, &item.name, item.lore.as_deref().unwrap_or(&item.description));
                    }
                    return ActionResult {
                        messages: lines.into_iter().map(|text| OutputLine { text, line_type: LineType::Narration }).collect(),
                        action_type: ActionType::DisplayOnly,
                        narrative_context: None,
                        sound_cues: vec![],
                    };
                }
            }

            if let Some((id, _)) = fuzzy_match_npc(&target, &loc.npcs, &state.npcs) {
                if let Some(npc) = state.npcs.get(id) {
                    let lines = templates::describe_examine_npc(npc);
                    return ActionResult {
                        messages: lines.into_iter().map(|text| OutputLine { text, line_type: LineType::Narration }).collect(),
                        action_type: ActionType::DisplayOnly,
                        narrative_context: None,
                sound_cues: vec![],
                    };
                }
            }

            ActionResult {
                messages: vec![OutputLine {
                    text: templates::describe_not_found(&target),
                    line_type: LineType::Error,
                }],
                action_type: ActionType::Error {
                    message: format!("Not found: {}", target),
                },
                narrative_context: None,
                sound_cues: vec![],
            }
        }
    }
}

/// Shared post-move logic: events, death check, auto-look, hostile NPC check, quest progress.
fn finalize_move(dest_id: &str, first_visit: bool, messages: &mut Vec<OutputLine>, state: &mut WorldState) -> Option<ActionResult> {
    // Fire OnEnter events
    let event_msgs = events::process_events(&EventTrigger::OnEnter, dest_id, state);
    messages.extend(event_msgs);

    // Check for player death from event damage
    if state.player.health <= 0 {
        state.game_mode = GameMode::GameOver(EndingType::Death);
        messages.push(OutputLine {
            text: templates::describe_player_death(),
            line_type: LineType::Combat,
        });
        return Some(ActionResult {
            messages: messages.clone(),
            action_type: ActionType::PlayerDeath,
            narrative_context: build_narrative_context(&ActionType::PlayerDeath, state),
                sound_cues: vec![],
        });
    }

    // Auto-look
    if let Some(dest_loc) = state.locations.get(dest_id) {
        let look_lines = templates::describe_location(dest_loc, &state.items, &state.npcs, first_visit);
        messages.extend(look_lines.into_iter().map(|text| OutputLine {
            text,
            line_type: LineType::Narration,
        }));
    }

    // Check for hostile NPCs → auto enter combat
    if let Some(hostile_npc) = find_hostile_npc_in_location(dest_id, state) {
        state.game_mode = GameMode::InCombat(hostile_npc.clone());
        state.combat_state = Some(CombatState {
            enemy_id: hostile_npc.clone(),
            player_turn: true,
            turn_count: 0,
        });
        let npc_name = state
            .npcs
            .get(&hostile_npc)
            .map(|n| n.name.clone())
            .unwrap_or_default();
        messages.push(OutputLine {
            text: format!("{} attacks you!", npc_name),
            line_type: LineType::Combat,
        });
    }

    // Check quest progress
    let quest_msgs = quest::check_quest_progress(state);
    messages.extend(quest_msgs);

    None
}

fn execute_go(direction: Direction, state: &mut WorldState) -> ActionResult {
    let current_loc = state.player.location.clone();
    let loc = match state.locations.get(&current_loc) {
        Some(l) => l.clone(),
        None => {
            return ActionResult {
                messages: vec![OutputLine {
                    text: "You are nowhere.".into(),
                    line_type: LineType::Error,
                }],
                action_type: ActionType::Error {
                    message: "Invalid location".into(),
                },
                narrative_context: None,
                sound_cues: vec![],
            }
        }
    };

    // Check if exit exists
    let dest_id = match loc.exits.get(&direction) {
        Some(dest) => dest.clone(),
        None => {
            return ActionResult {
                messages: vec![OutputLine {
                    text: templates::describe_cant_go(&direction),
                    line_type: LineType::Error,
                }],
                action_type: ActionType::Error {
                    message: format!("Can't go {}", direction),
                },
                narrative_context: None,
                sound_cues: vec![],
            }
        }
    };

    let mut messages = Vec::new();

    // Check if locked
    if let Some(key_id) = loc.locked_exits.get(&direction) {
        if state.player.inventory.contains(key_id) {
            // Unlock the door
            let key_name = state
                .items
                .get(key_id)
                .map(|i| i.name.clone())
                .unwrap_or_else(|| key_id.clone());

            // Remove key from inventory and unlock both sides
            state.player.inventory.retain(|id| id != key_id);
            if let Some(l) = state.locations.get_mut(&current_loc) {
                l.locked_exits.remove(&direction);
            }
            if let Some(dest_loc) = state.locations.get_mut(&dest_id) {
                dest_loc.locked_exits.remove(&direction.opposite());
            }

            messages.push(OutputLine {
                text: templates::describe_door_unlocked(&direction, &key_name),
                line_type: LineType::System,
            });
        } else {
            return ActionResult {
                messages: vec![OutputLine {
                    text: templates::describe_locked_door(&direction),
                    line_type: LineType::Error,
                }],
                action_type: ActionType::Error {
                    message: format!("Locked: {}", direction),
                },
                narrative_context: None,
                sound_cues: vec![],
            };
        }
    }

    // Move player
    state.player.location = dest_id.clone();
    state.player.turns_elapsed += 1;
    let first_visit = !state.player.visited_locations.contains(&dest_id);
    state.player.visited_locations.insert(dest_id.clone());

    if let Some(dest) = state.locations.get_mut(&dest_id) {
        dest.visited = true;
    }

    // Auto-add journal entry for first visit
    if first_visit {
        if let Some(dest) = state.locations.get(&dest_id) {
            let dest_name = dest.name.clone();
            let dest_desc = dest.description.clone();
            add_journal_entry(state, &format!("loc_{}", dest_id), JournalCategory::Location, &dest_name, &dest_desc);
        }
    }

    // Process turn-based events and status effect ticks
    let turn_msgs = events::process_turn_events(state);
    messages.extend(turn_msgs);

    // Check for player death from status effect damage
    if state.player.health <= 0 {
        state.game_mode = GameMode::GameOver(EndingType::Death);
        messages.push(OutputLine {
            text: templates::describe_player_death(),
            line_type: LineType::Combat,
        });
        return ActionResult {
            messages,
            action_type: ActionType::PlayerDeath,
            narrative_context: build_narrative_context(&ActionType::PlayerDeath, state),
            sound_cues: vec![],
        };
    }

    // Shared post-move logic
    if let Some(early_return) = finalize_move(&dest_id, first_visit, &mut messages, state) {
        return early_return;
    }

    let action_type = ActionType::RoomEntered { first_visit };
    let ctx = build_narrative_context(&action_type, state);
    ActionResult {
        messages,
        action_type,
        narrative_context: ctx,
                sound_cues: vec![],
    }
}

fn find_hostile_npc_in_location(loc_id: &str, state: &WorldState) -> Option<String> {
    state.locations.get(loc_id).and_then(|loc| {
        loc.npcs.iter().find_map(|npc_id| {
            state.npcs.get(npc_id).and_then(|npc| {
                if npc.hostile && npc.dialogue_state != DialogueState::Dead {
                    Some(npc_id.clone())
                } else {
                    None
                }
            })
        })
    })
}

fn execute_take(target: &str, state: &mut WorldState) -> ActionResult {
    let loc_id = state.player.location.clone();
    let room_items = state
        .locations
        .get(&loc_id)
        .map(|l| l.items.clone())
        .unwrap_or_default();

    let matches = fuzzy_match_item(target, &room_items, &state.items);
    if matches.is_empty() {
        return ActionResult {
            messages: vec![OutputLine {
                text: templates::describe_not_found(target),
                line_type: LineType::Error,
            }],
            action_type: ActionType::Error {
                message: format!("Not found: {}", target),
            },
            narrative_context: None,
                sound_cues: vec![],
        };
    }

    if matches.len() > 1 {
        let names: Vec<String> = matches.iter().map(|(_, name)| name.to_string()).collect();
        return ActionResult {
            messages: vec![OutputLine {
                text: templates::describe_ambiguous_target(&names),
                line_type: LineType::System,
            }],
            action_type: ActionType::DisplayOnly,
            narrative_context: None,
                sound_cues: vec![],
        };
    }

    let (item_id, item_name) = (matches[0].0.to_string(), matches[0].1.to_string());

    if state.player.inventory.len() >= state.player.max_inventory {
        return ActionResult {
            messages: vec![OutputLine {
                text: templates::describe_inventory_full(),
                line_type: LineType::Error,
            }],
            action_type: ActionType::Error {
                message: "Inventory full".into(),
            },
            narrative_context: None,
                sound_cues: vec![],
        };
    }

    // Move item from room to inventory
    if let Some(loc) = state.locations.get_mut(&loc_id) {
        loc.items.retain(|id| id != &item_id);
    }
    state.player.inventory.push(item_id.clone());
    state.player.turns_elapsed += 1;

    let mut messages = vec![OutputLine {
        text: templates::describe_take(&item_name),
        line_type: LineType::Narration,
    }];

    // Fire OnTake events
    let event_msgs = events::process_events(
        &EventTrigger::OnTake(item_id.clone()),
        &loc_id,
        state,
    );
    messages.extend(event_msgs);

    // Check quest progress
    let quest_msgs = quest::check_quest_progress(state);
    messages.extend(quest_msgs);

    let action_type = ActionType::ItemTaken { item_name: item_name.clone() };
    let ctx = build_narrative_context(&action_type, state);
    ActionResult {
        messages,
        action_type,
        narrative_context: ctx,
                sound_cues: vec![],
    }
}

fn execute_drop(target: &str, state: &mut WorldState) -> ActionResult {
    let matches = fuzzy_match_item(target, &state.player.inventory, &state.items);
    if matches.is_empty() {
        return ActionResult {
            messages: vec![OutputLine {
                text: format!("You don't have '{}'.", target),
                line_type: LineType::Error,
            }],
            action_type: ActionType::Error {
                message: format!("Not in inventory: {}", target),
            },
            narrative_context: None,
                sound_cues: vec![],
        };
    }

    let (item_id, item_name) = (matches[0].0.to_string(), matches[0].1.to_string());

    // Unequip if equipped
    if state.player.equipped_weapon.as_deref() == Some(&item_id) {
        state.player.equipped_weapon = None;
    }
    if state.player.equipped_armor.as_deref() == Some(&item_id) {
        state.player.equipped_armor = None;
    }

    state.player.inventory.retain(|id| id != &item_id);
    let loc_id = state.player.location.clone();
    if let Some(loc) = state.locations.get_mut(&loc_id) {
        loc.items.push(item_id);
    }
    state.player.turns_elapsed += 1;

    let action_type = ActionType::ItemDropped { item_name: item_name.clone() };
    let ctx = build_narrative_context(&action_type, state);
    ActionResult {
        messages: vec![OutputLine {
            text: templates::describe_drop(&item_name),
            line_type: LineType::Narration,
        }],
        action_type,
        narrative_context: ctx,
                sound_cues: vec![],
    }
}

fn execute_use(target: &str, state: &mut WorldState) -> ActionResult {
    let matches = fuzzy_match_item(target, &state.player.inventory, &state.items);
    if matches.is_empty() {
        return ActionResult {
            messages: vec![OutputLine {
                text: format!("You don't have '{}'.", target),
                line_type: LineType::Error,
            }],
            action_type: ActionType::Error {
                message: format!("Not in inventory: {}", target),
            },
            narrative_context: None,
                sound_cues: vec![],
        };
    }

    let (item_id, _) = (matches[0].0.to_string(), matches[0].1.to_string());
    let item = match state.items.get(&item_id) {
        Some(i) => i.clone(),
        None => {
            return ActionResult {
                messages: vec![OutputLine {
                    text: "Item data not found.".into(),
                    line_type: LineType::Error,
                }],
                action_type: ActionType::Error {
                    message: "Item not found".into(),
                },
                narrative_context: None,
                sound_cues: vec![],
            }
        }
    };

    if !item.usable {
        return ActionResult {
            messages: vec![OutputLine {
                text: format!("You can't use the {}.", item.name),
                line_type: LineType::Error,
            }],
            action_type: ActionType::Error {
                message: format!("Not usable: {}", item.name),
            },
            narrative_context: None,
                sound_cues: vec![],
        };
    }

    let mut messages = Vec::new();
    let mut effect = String::new();

    match item.item_type {
        ItemType::Consumable => {
            if let Some(modifier) = &item.modifier {
                if modifier.health > 0 {
                    state.player.health =
                        (state.player.health + modifier.health).min(state.player.max_health);
                    effect = format!("You feel restored. (+{} HP)", modifier.health);
                }
                if modifier.attack > 0 {
                    state.player.attack += modifier.attack;
                    effect.push_str(&format!(" (+{} Attack)", modifier.attack));
                }
                if modifier.defense > 0 {
                    state.player.defense += modifier.defense;
                    effect.push_str(&format!(" (+{} Defense)", modifier.defense));
                }
            } else {
                effect = "You consume it.".to_string();
            }
            if item.consumable {
                state.player.inventory.retain(|id| id != &item_id);
            }
        }
        ItemType::Scroll => {
            effect = "The scroll crumbles to dust as its magic takes effect.".to_string();
            if item.consumable {
                state.player.inventory.retain(|id| id != &item_id);
            }
        }
        ItemType::Key => {
            return ActionResult {
                messages: vec![OutputLine {
                    text: "Use this by going through a locked door.".into(),
                    line_type: LineType::System,
                }],
                action_type: ActionType::DisplayOnly,
                narrative_context: None,
                sound_cues: vec![],
            };
        }
        _ => {
            effect = "Nothing happens.".to_string();
        }
    }

    messages.push(OutputLine {
        text: templates::describe_use(&item.name, &effect),
        line_type: LineType::Narration,
    });

    // Fire OnUse events
    let loc_id = state.player.location.clone();
    let event_msgs = events::process_events(
        &EventTrigger::OnUse(item_id.clone()),
        &loc_id,
        state,
    );
    messages.extend(event_msgs);

    // Check quest progress
    let quest_msgs = quest::check_quest_progress(state);
    messages.extend(quest_msgs);

    state.player.turns_elapsed += 1;

    let action_type = ActionType::ItemUsed {
        item_name: item.name.clone(),
        effect: effect.clone(),
    };
    let ctx = build_narrative_context(&action_type, state);
    ActionResult {
        messages,
        action_type,
        narrative_context: ctx,
                sound_cues: vec![],
    }
}

fn execute_equip(target: &str, state: &mut WorldState) -> ActionResult {
    let matches = fuzzy_match_item(target, &state.player.inventory, &state.items);
    if matches.is_empty() {
        return ActionResult {
            messages: vec![OutputLine {
                text: format!("You don't have '{}'.", target),
                line_type: LineType::Error,
            }],
            action_type: ActionType::Error {
                message: format!("Not in inventory: {}", target),
            },
            narrative_context: None,
                sound_cues: vec![],
        };
    }

    let (item_id, _) = (matches[0].0.to_string(), matches[0].1.to_string());
    let item = state.items.get(&item_id).cloned();
    let item = match item {
        Some(i) => i,
        None => {
            return ActionResult {
                messages: vec![OutputLine {
                    text: "Item not found.".into(),
                    line_type: LineType::Error,
                }],
                action_type: ActionType::Error {
                    message: "Item not found".into(),
                },
                narrative_context: None,
                sound_cues: vec![],
            }
        }
    };

    let mut messages = Vec::new();

    match item.item_type {
        ItemType::Weapon => {
            if let Some(old_weapon) = &state.player.equipped_weapon {
                if let Some(old) = state.items.get(old_weapon) {
                    messages.push(OutputLine {
                        text: templates::describe_unequip(&old.name),
                        line_type: LineType::System,
                    });
                }
            }
            state.player.equipped_weapon = Some(item_id);
        }
        ItemType::Armor => {
            if let Some(old_armor) = &state.player.equipped_armor {
                if let Some(old) = state.items.get(old_armor) {
                    messages.push(OutputLine {
                        text: templates::describe_unequip(&old.name),
                        line_type: LineType::System,
                    });
                }
            }
            state.player.equipped_armor = Some(item_id);
        }
        _ => {
            return ActionResult {
                messages: vec![OutputLine {
                    text: format!("You can't equip the {}.", item.name),
                    line_type: LineType::Error,
                }],
                action_type: ActionType::Error {
                    message: format!("Not equippable: {}", item.name),
                },
                narrative_context: None,
                sound_cues: vec![],
            };
        }
    }

    messages.push(OutputLine {
        text: templates::describe_equip(&item.name),
        line_type: LineType::Narration,
    });

    let action_type = ActionType::ItemEquipped { item_name: item.name.clone() };
    let ctx = build_narrative_context(&action_type, state);
    ActionResult {
        messages,
        action_type,
        narrative_context: ctx,
                sound_cues: vec![],
    }
}

fn execute_unequip(target: &str, state: &mut WorldState) -> ActionResult {
    let target_lower = target.to_lowercase();

    // Check weapon
    if let Some(weapon_id) = &state.player.equipped_weapon.clone() {
        if let Some(item) = state.items.get(weapon_id) {
            if item.name.to_lowercase().contains(&target_lower)
                || weapon_id.to_lowercase().contains(&target_lower)
            {
                let name = item.name.clone();
                state.player.equipped_weapon = None;
                return ActionResult {
                    messages: vec![OutputLine {
                        text: templates::describe_unequip(&name),
                        line_type: LineType::Narration,
                    }],
                    action_type: ActionType::ItemUnequipped { item_name: name },
                    narrative_context: None,
                sound_cues: vec![],
                };
            }
        }
    }

    // Check armor
    if let Some(armor_id) = &state.player.equipped_armor.clone() {
        if let Some(item) = state.items.get(armor_id) {
            if item.name.to_lowercase().contains(&target_lower)
                || armor_id.to_lowercase().contains(&target_lower)
            {
                let name = item.name.clone();
                state.player.equipped_armor = None;
                return ActionResult {
                    messages: vec![OutputLine {
                        text: templates::describe_unequip(&name),
                        line_type: LineType::Narration,
                    }],
                    action_type: ActionType::ItemUnequipped { item_name: name },
                    narrative_context: None,
                sound_cues: vec![],
                };
            }
        }
    }

    ActionResult {
        messages: vec![OutputLine {
            text: format!("You don't have '{}' equipped.", target),
            line_type: LineType::Error,
        }],
        action_type: ActionType::Error {
            message: format!("Not equipped: {}", target),
        },
        narrative_context: None,
                sound_cues: vec![],
    }
}

fn execute_talk(target: &str, state: &mut WorldState) -> ActionResult {
    let loc_id = state.player.location.clone();
    let npc_ids = state
        .locations
        .get(&loc_id)
        .map(|l| l.npcs.clone())
        .unwrap_or_default();

    let npc_match = fuzzy_match_npc(target, &npc_ids, &state.npcs);
    match npc_match {
        Some((npc_id, _)) => {
            let npc_id = npc_id.to_string();
            let result = dialogue::enter_dialogue(&npc_id, state);
            state.player.turns_elapsed += 1;
            ActionResult {
                messages: result.messages,
                action_type: result.action_type,
                narrative_context: build_narrative_context(&ActionType::DisplayOnly, state),
                sound_cues: vec![],
            }
        }
        None => ActionResult {
            messages: vec![OutputLine {
                text: templates::describe_not_found(target),
                line_type: LineType::Error,
            }],
            action_type: ActionType::Error {
                message: format!("NPC not found: {}", target),
            },
            narrative_context: None,
                sound_cues: vec![],
        },
    }
}

fn execute_dialogue_input(input: &str, npc_id: &str, state: &mut WorldState) -> ActionResult {
    let result = dialogue::process_dialogue_input(input, npc_id, state);
    ActionResult {
        messages: result.messages,
        action_type: result.action_type,
        narrative_context: build_narrative_context(&ActionType::DisplayOnly, state),
                sound_cues: vec![],
    }
}

fn execute_attack(target: &str, state: &mut WorldState) -> ActionResult {
    // If already in combat, execute attack
    if let GameMode::InCombat(_) = &state.game_mode {
        let result = combat::execute_player_attack(state);
        state.player.turns_elapsed += 1;

        if result.enemy_defeated {
            let quest_msgs = quest::check_quest_progress(state);
            let mut messages = result.messages;
            messages.extend(quest_msgs);
            let ctx = build_narrative_context(&result.action_type, state);
            return ActionResult {
                messages,
                action_type: result.action_type,
                narrative_context: ctx,
                sound_cues: vec![],
            };
        }

        let ctx = build_narrative_context(&result.action_type, state);
        return ActionResult {
            messages: result.messages,
            action_type: result.action_type,
            narrative_context: ctx,
                sound_cues: vec![],
        };
    }

    // Find NPC to attack
    let loc_id = state.player.location.clone();
    let npc_ids = state
        .locations
        .get(&loc_id)
        .map(|l| l.npcs.clone())
        .unwrap_or_default();

    let npc_match = fuzzy_match_npc(target, &npc_ids, &state.npcs);
    match npc_match {
        Some((npc_id, _)) => {
            let npc_id = npc_id.to_string();
            let npc = match state.npcs.get(&npc_id) {
                Some(n) => n.clone(),
                None => {
                    return ActionResult {
                        messages: vec![OutputLine {
                            text: "Target not found.".into(),
                            line_type: LineType::Error,
                        }],
                        action_type: ActionType::Error {
                            message: "NPC data missing".into(),
                        },
                        narrative_context: None,
                sound_cues: vec![],
                    }
                }
            };

            if npc.dialogue_state == DialogueState::Dead {
                return ActionResult {
                    messages: vec![OutputLine {
                        text: format!("{} is already dead.", npc.name),
                        line_type: LineType::Error,
                    }],
                    action_type: ActionType::Error {
                        message: "Target is dead".into(),
                    },
                    narrative_context: None,
                sound_cues: vec![],
                };
            }

            // Enter combat
            state.game_mode = GameMode::InCombat(npc_id.clone());
            state.combat_state = Some(CombatState {
                enemy_id: npc_id.clone(),
                player_turn: true,
                turn_count: 0,
            });

            // Make NPC hostile
            if let Some(n) = state.npcs.get_mut(&npc_id) {
                if !n.hostile {
                    n.relationship = -50;
                    n.memory.push(crate::models::npc::NpcMemory {
                        turn: state.player.turns_elapsed,
                        event: "attacked_while_friendly".into(),
                    });
                }
                n.hostile = true;
            }

            // Add bestiary journal entry
            add_journal_entry(state, &format!("npc_{}", npc_id), JournalCategory::Bestiary, &npc.name, &npc.description);

            let mut messages = vec![OutputLine {
                text: format!("You engage {} in combat!", npc.name),
                line_type: LineType::Combat,
            }];

            // Execute first attack
            let result = combat::execute_player_attack(state);
            messages.extend(result.messages);
            state.player.turns_elapsed += 1;

            if result.player_died {
                return ActionResult {
                    messages,
                    action_type: result.action_type,
                    narrative_context: build_narrative_context(&ActionType::PlayerDeath, state),
                sound_cues: vec![],
                };
            }

            if result.enemy_defeated {
                let quest_msgs = quest::check_quest_progress(state);
                messages.extend(quest_msgs);
            }

            let ctx = build_narrative_context(&result.action_type, state);
            ActionResult {
                messages,
                action_type: result.action_type,
                narrative_context: ctx,
                sound_cues: vec![],
            }
        }
        None => ActionResult {
            messages: vec![OutputLine {
                text: templates::describe_not_found(target),
                line_type: LineType::Error,
            }],
            action_type: ActionType::Error {
                message: format!("Target not found: {}", target),
            },
            narrative_context: None,
                sound_cues: vec![],
        },
    }
}

fn execute_flee(state: &mut WorldState) -> ActionResult {
    if !matches!(state.game_mode, GameMode::InCombat(_)) {
        return ActionResult {
            messages: vec![OutputLine {
                text: "You're not in combat!".into(),
                line_type: LineType::Error,
            }],
            action_type: ActionType::Error {
                message: "Not in combat".into(),
            },
            narrative_context: None,
                sound_cues: vec![],
        };
    }

    let prev_loc = state.player.location.clone();
    let result = combat::execute_flee(state);
    state.player.turns_elapsed += 1;

    if result.player_died {
        return ActionResult {
            messages: result.messages,
            action_type: result.action_type,
            narrative_context: build_narrative_context(&ActionType::PlayerDeath, state),
                sound_cues: vec![],
        };
    }

    let mut messages = result.messages;

    // If player actually moved to a new room, fire OnEnter events and set visited
    if result.fled && state.player.location != prev_loc {
        let dest_id = state.player.location.clone();
        if let Some(loc) = state.locations.get_mut(&dest_id) {
            loc.visited = true;
        }
        let first_visit = !state.player.visited_locations.contains(&dest_id);
        if let Some(death_result) = finalize_move(&dest_id, first_visit, &mut messages, state) {
            return death_result;
        }
    }

    let ctx = build_narrative_context(&result.action_type, state);
    ActionResult {
        messages,
        action_type: result.action_type,
        narrative_context: ctx,
                sound_cues: vec![],
    }
}

fn execute_inventory(state: &mut WorldState) -> ActionResult {
    let lines = templates::describe_inventory(&state.player, &state.items);
    ActionResult {
        messages: lines
            .into_iter()
            .map(|text| OutputLine {
                text,
                line_type: LineType::System,
            })
            .collect(),
        action_type: ActionType::DisplayOnly,
        narrative_context: None,
                sound_cues: vec![],
    }
}

fn execute_map(state: &mut WorldState) -> ActionResult {
    let lines = templates::describe_map(&state.locations, &state.player);
    ActionResult {
        messages: lines
            .into_iter()
            .map(|text| OutputLine {
                text,
                line_type: LineType::System,
            })
            .collect(),
        action_type: ActionType::DisplayOnly,
        narrative_context: None,
                sound_cues: vec![],
    }
}

fn execute_quest_log(state: &mut WorldState) -> ActionResult {
    let mut lines = Vec::new();
    lines.push("--- Quest Log ---".to_string());

    let active: Vec<&Quest> = state.quests.values().filter(|q| q.active && !q.completed).collect();
    let completed: Vec<&Quest> = state.quests.values().filter(|q| q.completed).collect();

    if active.is_empty() && completed.is_empty() {
        lines.push("No quests yet.".to_string());
    } else {
        if !active.is_empty() {
            lines.push("Active:".to_string());
            for quest in active {
                lines.push(format!("  - {} — {}", quest.name, quest.description));
            }
        }
        if !completed.is_empty() {
            lines.push("Completed:".to_string());
            for quest in completed {
                lines.push(format!("  - {} (done)", quest.name));
            }
        }
    }

    ActionResult {
        messages: lines
            .into_iter()
            .map(|text| OutputLine {
                text,
                line_type: LineType::System,
            })
            .collect(),
        action_type: ActionType::DisplayOnly,
        narrative_context: None,
                sound_cues: vec![],
    }
}

fn add_journal_entry(state: &mut WorldState, id: &str, category: JournalCategory, title: &str, content: &str) {
    if !state.journal.iter().any(|e| e.id == id) {
        state.journal.push(JournalEntry {
            id: id.to_string(),
            category,
            title: title.to_string(),
            content: content.to_string(),
            discovered_turn: state.player.turns_elapsed,
        });
    }
}

fn execute_journal(state: &mut WorldState) -> ActionResult {
    let mut lines = vec!["--- Codex ---".to_string()];
    if state.journal.is_empty() {
        lines.push("No entries yet. Explore and examine to discover lore.".to_string());
    } else {
        let categories = [
            (JournalCategory::Location, "Locations"),
            (JournalCategory::Bestiary, "Bestiary"),
            (JournalCategory::Item, "Items"),
            (JournalCategory::Lore, "Lore"),
        ];
        for (cat, label) in &categories {
            let entries: Vec<&JournalEntry> = state.journal.iter().filter(|e| &e.category == cat).collect();
            if !entries.is_empty() {
                lines.push(format!("\n{}:", label));
                for entry in entries {
                    lines.push(format!("  - {}: {}", entry.title, entry.content));
                }
            }
        }
    }
    ActionResult {
        messages: lines.into_iter().map(|text| OutputLine { text, line_type: LineType::System }).collect(),
        action_type: ActionType::DisplayOnly,
        narrative_context: None,
        sound_cues: vec![],
    }
}

fn execute_secret(word: &str, state: &mut WorldState) -> ActionResult {
    let already_discovered = state.player.discovered_secrets.contains(&word.to_string());
    if !already_discovered {
        state.player.discovered_secrets.push(word.to_string());
    }

    match word {
        "xyzzy" => {
            // Teleport to a random visited room
            let visited: Vec<String> = state.player.visited_locations.iter()
                .filter(|loc| *loc != &state.player.location)
                .cloned()
                .collect();
            if visited.is_empty() {
                return ActionResult {
                    messages: vec![OutputLine {
                        text: "A hollow voice says \"Nothing happens.\" You haven't explored enough.".to_string(),
                        line_type: LineType::System,
                    }],
                    action_type: ActionType::DisplayOnly,
                    narrative_context: None,
                    sound_cues: vec![],
                };
            }
            // Use turns_elapsed as a pseudo-random index
            let idx = state.player.turns_elapsed as usize % visited.len();
            let dest_id = visited[idx].clone();
            state.player.location = dest_id.clone();
            let dest_name = state.locations.get(&dest_id).map(|l| l.name.clone()).unwrap_or(dest_id.clone());
            ActionResult {
                messages: vec![
                    OutputLine {
                        text: "The world shifts and blurs around you...".to_string(),
                        line_type: LineType::System,
                    },
                    OutputLine {
                        text: format!("You find yourself in {}.", dest_name),
                        line_type: LineType::Narration,
                    },
                ],
                action_type: ActionType::RoomEntered { first_visit: false },
                narrative_context: None,
                sound_cues: vec![SoundCue::DoorUnlock],
            }
        }
        "plugh" => {
            // Reveal hidden vault if in great_hall, else flavor text
            if state.player.location == "great_hall" {
                // Add exit to hidden_vault from great_hall
                if let Some(loc) = state.locations.get_mut("great_hall") {
                    if let std::collections::hash_map::Entry::Vacant(e) = loc.exits.entry(Direction::Down) {
                        e.insert("hidden_vault".into());
                        return ActionResult {
                            messages: vec![
                                OutputLine {
                                    text: "You speak the ancient word. The floor trembles, and a hidden staircase descends into darkness below!".to_string(),
                                    line_type: LineType::System,
                                },
                                OutputLine {
                                    text: "A new passage has opened downward.".to_string(),
                                    line_type: LineType::System,
                                },
                            ],
                            action_type: ActionType::EventTriggered { event_description: "Hidden vault revealed".into() },
                            narrative_context: None,
                            sound_cues: vec![SoundCue::DoorUnlock],
                        };
                    }
                }
                ActionResult {
                    messages: vec![OutputLine {
                        text: "The passage to the vault is already open.".to_string(),
                        line_type: LineType::System,
                    }],
                    action_type: ActionType::DisplayOnly,
                    narrative_context: None,
                    sound_cues: vec![],
                }
            } else {
                ActionResult {
                    messages: vec![OutputLine {
                        text: "A hollow voice says \"Plugh.\" Nothing seems to happen here. Perhaps in a grander hall...".to_string(),
                        line_type: LineType::System,
                    }],
                    action_type: ActionType::DisplayOnly,
                    narrative_context: None,
                    sound_cues: vec![],
                }
            }
        }
        "abracadabra" => {
            // Restore a small amount of health
            let heal = 5;
            state.player.health = (state.player.health + heal).min(state.player.max_health);
            ActionResult {
                messages: vec![OutputLine {
                    text: format!("A tingle of magic courses through you. (+{} HP)", heal),
                    line_type: LineType::System,
                }],
                action_type: ActionType::DisplayOnly,
                narrative_context: None,
                sound_cues: vec![SoundCue::ItemUse],
            }
        }
        "sesame" | "opensesame" => {
            ActionResult {
                messages: vec![OutputLine {
                    text: "The word echoes off the walls. You feel you're being watched by something ancient and amused.".to_string(),
                    line_type: LineType::System,
                }],
                action_type: ActionType::DisplayOnly,
                narrative_context: None,
                sound_cues: vec![],
            }
        }
        _ => {
            ActionResult {
                messages: vec![OutputLine {
                    text: "Nothing happens.".to_string(),
                    line_type: LineType::System,
                }],
                action_type: ActionType::DisplayOnly,
                narrative_context: None,
                sound_cues: vec![],
            }
        }
    }
}

fn execute_help(state: &mut WorldState) -> ActionResult {
    let lines = templates::describe_help(&state.game_mode);
    ActionResult {
        messages: lines
            .into_iter()
            .map(|text| OutputLine {
                text,
                line_type: LineType::System,
            })
            .collect(),
        action_type: ActionType::DisplayOnly,
        narrative_context: None,
                sound_cues: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::world_builder;
    use std::collections::HashMap;

    fn make_test_world() -> WorldState {
        let mut state = WorldState::default();
        state.locations.insert(
            "room_a".into(),
            Location {
                id: "room_a".into(),
                name: "Room A".into(),
                description: "The first room.".into(),
                items: vec!["sword".into(), "potion".into()],
                npcs: vec!["guard".into()],
                exits: HashMap::from([(Direction::North, "room_b".into())]),
                locked_exits: HashMap::new(),
                visited: true,
                discovered_secrets: vec![],
                ambient_mood: Mood::Peaceful,
                examine_details: Some("Scratches on the walls suggest a struggle.".into()),
                revisit_description: Some("Room A feels familiar.".into()),
            },
        );
        state.locations.insert(
            "room_b".into(),
            Location {
                id: "room_b".into(),
                name: "Room B".into(),
                description: "The second room.".into(),
                items: vec![],
                npcs: vec![],
                exits: HashMap::from([(Direction::South, "room_a".into())]),
                locked_exits: HashMap::new(),
                visited: false,
                discovered_secrets: vec![],
                ambient_mood: Mood::Mysterious,
                examine_details: None,
                revisit_description: None,
            },
        );
        state.items.insert(
            "sword".into(),
            Item {
                id: "sword".into(),
                name: "Short Sword".into(),
                description: "A sharp blade.".into(),
                item_type: ItemType::Weapon,
                modifier: Some(StatModifier {
                    attack: 3,
                    defense: 0,
                    health: 0,
                }),
                usable: false,
                consumable: false,
                key_id: None,
                lore: Some("An ancient blade.".into()),
            },
        );
        state.items.insert(
            "potion".into(),
            Item {
                id: "potion".into(),
                name: "Health Potion".into(),
                description: "A red potion.".into(),
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
        state.npcs.insert(
            "guard".into(),
            Npc {
                id: "guard".into(),
                name: "Guard".into(),
                description: "A watchful guard.".into(),
                personality_seed: String::new(),
                dialogue_state: DialogueState::Greeting,
                hostile: false,
                health: 20,
                max_health: 20,
                attack: 5,
                defense: 3,
                items: vec![],
                quest_giver: None,
                examine_text: Some("The guard wears a faded crest.".into()),
                relationship: 0,
                memory: vec![],
            },
        );
        state.player.location = "room_a".into();
        state.player.visited_locations.insert("room_a".into());
        state.initialized = true;
        state
    }

    #[test]
    fn test_look() {
        let mut state = make_test_world();
        let result = execute(GameCommand::Look(None), &mut state);
        assert!(!result.messages.is_empty());
        assert!(result.messages.iter().any(|m| m.text.contains("Room A")));
    }

    #[test]
    fn test_look_at_item() {
        let mut state = make_test_world();
        let result = execute(GameCommand::Look(Some("sword".into())), &mut state);
        assert!(result.messages.iter().any(|m| m.text.contains("sharp blade")));
    }

    #[test]
    fn test_go_north() {
        let mut state = make_test_world();
        let result = execute(GameCommand::Go(Direction::North), &mut state);
        assert_eq!(state.player.location, "room_b");
        assert!(result.messages.iter().any(|m| m.text.contains("Room B")));
    }

    #[test]
    fn test_cant_go() {
        let mut state = make_test_world();
        let result = execute(GameCommand::Go(Direction::West), &mut state);
        assert_eq!(state.player.location, "room_a");
        assert!(result
            .messages
            .iter()
            .any(|m| m.line_type == LineType::Error));
    }

    #[test]
    fn test_take_item() {
        let mut state = make_test_world();
        let result = execute(GameCommand::Take("sword".into()), &mut state);
        assert!(state.player.inventory.contains(&"sword".to_string()));
        assert!(result.messages.iter().any(|m| m.text.contains("pick up")));
    }

    #[test]
    fn test_drop_item() {
        let mut state = make_test_world();
        state.player.inventory.push("sword".into());
        state
            .locations
            .get_mut("room_a")
            .unwrap()
            .items
            .retain(|id| id != "sword");
        let result = execute(GameCommand::Drop("sword".into()), &mut state);
        assert!(!state.player.inventory.contains(&"sword".to_string()));
        assert!(result.messages.iter().any(|m| m.text.contains("drop")));
    }

    #[test]
    fn test_use_potion() {
        let mut state = make_test_world();
        state.player.inventory.push("potion".into());
        state.player.health = 50;
        let result = execute(GameCommand::Use("potion".into()), &mut state);
        assert_eq!(state.player.health, 75);
        assert!(!state.player.inventory.contains(&"potion".to_string()));
        assert!(result.messages.iter().any(|m| m.text.contains("restored")));
    }

    #[test]
    fn test_equip_weapon() {
        let mut state = make_test_world();
        state.player.inventory.push("sword".into());
        execute(GameCommand::Equip("sword".into()), &mut state);
        assert_eq!(state.player.equipped_weapon, Some("sword".into()));
    }

    #[test]
    fn test_inventory_full() {
        let mut state = make_test_world();
        for i in 0..10 {
            state.player.inventory.push(format!("item_{}", i));
        }
        let result = execute(GameCommand::Take("sword".into()), &mut state);
        assert!(result
            .messages
            .iter()
            .any(|m| m.text.contains("full")));
    }

    #[test]
    fn test_examine_item_shows_stats_and_lore() {
        let mut state = make_test_world();
        let result = execute(GameCommand::Look(Some("sword".into())), &mut state);
        assert!(result.messages.iter().any(|m| m.text.contains("Attack +3")));
        assert!(result.messages.iter().any(|m| m.text.contains("ancient blade")));
    }

    #[test]
    fn test_examine_npc_shows_examine_text() {
        let mut state = make_test_world();
        let result = execute(GameCommand::Look(Some("guard".into())), &mut state);
        assert!(result.messages.iter().any(|m| m.text.contains("faded crest")));
        assert!(result.messages.iter().any(|m| m.text.contains("20/20")));
    }

    #[test]
    fn test_examine_room() {
        let mut state = make_test_world();
        let result = execute(GameCommand::Look(Some("room".into())), &mut state);
        assert!(result.messages.iter().any(|m| m.text.contains("Scratches")));
    }

    #[test]
    fn test_examine_room_inventory_item() {
        let mut state = make_test_world();
        state.player.inventory.push("sword".into());
        state.locations.get_mut("room_a").unwrap().items.retain(|id| id != "sword");
        let result = execute(GameCommand::Look(Some("sword".into())), &mut state);
        assert!(result.messages.iter().any(|m| m.text.contains("Attack +3")));
    }

    #[test]
    fn test_revisit_description() {
        let mut state = make_test_world();
        // Room A is visited=true, has revisit_description
        let result = execute(GameCommand::Look(None), &mut state);
        assert!(result.messages.iter().any(|m| m.text.contains("familiar")));
    }

    #[test]
    fn test_first_visit_uses_full_description() {
        let mut state = make_test_world();
        let result = execute(GameCommand::Go(Direction::North), &mut state);
        assert!(result.messages.iter().any(|m| m.text.contains("second room")));
    }

    #[test]
    fn test_journal_entry_on_first_visit() {
        let mut state = make_test_world();
        assert!(state.journal.is_empty());
        execute(GameCommand::Go(Direction::North), &mut state);
        assert_eq!(state.player.location, "room_b");
        assert!(!state.journal.is_empty());
        let entry = state.journal.iter().find(|e| e.id == "loc_room_b");
        assert!(entry.is_some());
        let entry = entry.unwrap();
        assert_eq!(entry.category, JournalCategory::Location);
        assert_eq!(entry.title, "Room B");
        assert_eq!(entry.content, "The second room.");
    }

    #[test]
    fn test_journal_no_duplicate_on_revisit() {
        let mut state = make_test_world();
        execute(GameCommand::Go(Direction::North), &mut state);
        let count_before = state.journal.len();
        // Go back and revisit
        execute(GameCommand::Go(Direction::South), &mut state);
        execute(GameCommand::Go(Direction::North), &mut state);
        // room_b was already visited, so no new entry
        let loc_entries: Vec<_> = state.journal.iter().filter(|e| e.id == "loc_room_b").collect();
        assert_eq!(loc_entries.len(), 1);
        // room_a is also visited from start, but going back is not first_visit
        assert_eq!(state.journal.len(), count_before);
    }

    #[test]
    fn test_journal_command_empty() {
        let mut state = make_test_world();
        let result = execute(GameCommand::Journal, &mut state);
        assert!(result.messages.iter().any(|m| m.text.contains("Codex")));
        assert!(result.messages.iter().any(|m| m.text.contains("No entries yet")));
    }

    #[test]
    fn test_journal_command_shows_entries() {
        let mut state = make_test_world();
        state.journal.push(JournalEntry {
            id: "loc_room_a".into(),
            category: JournalCategory::Location,
            title: "Room A".into(),
            content: "The first room.".into(),
            discovered_turn: 0,
        });
        state.journal.push(JournalEntry {
            id: "npc_guard".into(),
            category: JournalCategory::Bestiary,
            title: "Guard".into(),
            content: "A watchful guard.".into(),
            discovered_turn: 1,
        });
        let result = execute(GameCommand::Journal, &mut state);
        assert!(result.messages.iter().any(|m| m.text.contains("Codex")));
        assert!(result.messages.iter().any(|m| m.text.contains("Locations")));
        assert!(result.messages.iter().any(|m| m.text.contains("Room A")));
        assert!(result.messages.iter().any(|m| m.text.contains("Bestiary")));
        assert!(result.messages.iter().any(|m| m.text.contains("Guard")));
    }

    #[test]
    fn test_examine_item_with_lore_adds_journal_entry() {
        let mut state = make_test_world();
        assert!(state.journal.is_empty());
        // Sword has lore "An ancient blade."
        execute(GameCommand::Look(Some("sword".into())), &mut state);
        let entry = state.journal.iter().find(|e| e.id == "item_sword");
        assert!(entry.is_some());
        let entry = entry.unwrap();
        assert_eq!(entry.category, JournalCategory::Item);
        assert_eq!(entry.content, "An ancient blade.");
    }

    #[test]
    fn test_examine_item_without_lore_no_journal_entry() {
        let mut state = make_test_world();
        // Potion has no lore
        execute(GameCommand::Look(Some("potion".into())), &mut state);
        assert!(state.journal.iter().find(|e| e.id == "item_potion").is_none());
    }

    #[test]
    fn secret_xyzzy_teleports() {
        let mut state = world_builder::build_thornhold();
        state.player.visited_locations.insert("great_hall".into());
        state.player.turns_elapsed = 1;
        let result = execute(GameCommand::Secret("xyzzy".into()), &mut state);
        assert!(result.messages.iter().any(|m| m.text.contains("world shifts")));
        assert_ne!(state.player.location, "courtyard");
        assert!(state.player.discovered_secrets.contains(&"xyzzy".to_string()));
    }

    #[test]
    fn secret_plugh_reveals_vault() {
        let mut state = world_builder::build_thornhold();
        state.player.location = "great_hall".into();
        let result = execute(GameCommand::Secret("plugh".into()), &mut state);
        assert!(result.messages.iter().any(|m| m.text.contains("hidden staircase")));
        let hall = state.locations.get("great_hall").unwrap();
        assert!(hall.exits.contains_key(&Direction::Down));
    }

    #[test]
    fn secret_abracadabra_heals() {
        let mut state = world_builder::build_thornhold();
        state.player.health = 50;
        execute(GameCommand::Secret("abracadabra".into()), &mut state);
        assert_eq!(state.player.health, 55);
    }
}
