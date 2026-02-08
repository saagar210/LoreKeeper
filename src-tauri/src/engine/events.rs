use crate::models::*;

pub fn process_events(
    trigger: &EventTrigger,
    location_id: &str,
    state: &mut WorldState,
) -> Vec<OutputLine> {
    let mut messages = Vec::new();

    let mut actions_to_apply: Vec<EventAction> = Vec::new();
    let mut events_to_mark_fired: Vec<usize> = Vec::new();

    for (idx, event) in state.events.iter().enumerate() {
        if event.location_id != location_id {
            continue;
        }
        if event.one_shot && event.fired {
            continue;
        }
        if &event.trigger != trigger {
            continue;
        }
        actions_to_apply.push(event.action.clone());
        if event.one_shot {
            events_to_mark_fired.push(idx);
        }
    }

    for idx in events_to_mark_fired {
        state.events[idx].fired = true;
    }

    for action in actions_to_apply {
        match action {
            EventAction::Damage(amount) => {
                state.player.health = (state.player.health - amount).max(0);
                messages.push(OutputLine {
                    text: crate::engine::templates::describe_event_damage(amount),
                    line_type: LineType::Combat,
                });
            }
            EventAction::SpawnNpc(npc_id) => {
                if let Some(loc) = state.locations.get_mut(location_id) {
                    if !loc.npcs.contains(&npc_id) {
                        loc.npcs.push(npc_id.clone());
                    }
                }
                messages.push(OutputLine {
                    text: "A presence manifests before you...".to_string(),
                    line_type: LineType::Narration,
                });
            }
            EventAction::Unlock(direction) => {
                if let Some(loc) = state.locations.get_mut(location_id) {
                    loc.locked_exits.remove(&direction);
                }
                messages.push(OutputLine {
                    text: format!("A passage {} has been revealed!", direction.display_name()),
                    line_type: LineType::System,
                });
            }
            EventAction::Message(msg) => {
                messages.push(OutputLine {
                    text: crate::engine::templates::describe_event_message(&msg),
                    line_type: LineType::Narration,
                });
            }
            EventAction::GiveItem(item_id) => {
                if state.player.inventory.len() < state.player.max_inventory {
                    state.player.inventory.push(item_id.clone());
                    if let Some(item) = state.items.get(&item_id) {
                        messages.push(OutputLine {
                            text: format!("You received: {}", item.name),
                            line_type: LineType::System,
                        });
                    }
                } else if let Some(item) = state.items.get(&item_id) {
                    messages.push(OutputLine {
                        text: format!(
                            "Your inventory is full! The {} falls to the ground.",
                            item.name
                        ),
                        line_type: LineType::System,
                    });
                    // Drop item in current location instead
                    if let Some(loc) =
                        state.locations.get_mut(&state.player.location)
                    {
                        loc.items.push(item_id.clone());
                    }
                }
            }
            EventAction::SetQuestFlag(flag) => {
                state.player.quest_flags.insert(flag.clone(), true);
            }
            EventAction::ApplyStatus(effect) => {
                let name = effect.name.clone();
                state.player.status_effects.push(effect);
                messages.push(OutputLine {
                    text: format!("You are now affected by: {}", name),
                    line_type: LineType::System,
                });
            }
            EventAction::RemoveStatus(name) => {
                state.player.status_effects.retain(|e| e.name != name);
                messages.push(OutputLine {
                    text: format!("{} has worn off.", name),
                    line_type: LineType::System,
                });
            }
            EventAction::ChangeDescription(loc_id, new_desc) => {
                if let Some(loc) = state.locations.get_mut(&loc_id) {
                    loc.description = new_desc;
                }
            }
        }
    }

    messages.retain(|m| !m.text.is_empty());
    messages
}

pub fn process_turn_events(state: &mut WorldState) -> Vec<OutputLine> {
    let mut messages = Vec::new();
    let current_turn = state.player.turns_elapsed;
    let location_id = state.player.location.clone();

    // Process OnTurn trigger events
    let mut actions_to_apply: Vec<EventAction> = Vec::new();
    let mut events_to_mark_fired: Vec<usize> = Vec::new();

    for (idx, event) in state.events.iter().enumerate() {
        if event.one_shot && event.fired {
            continue;
        }
        if let EventTrigger::OnTurn(turn) = &event.trigger {
            if *turn == current_turn && event.location_id == location_id {
                actions_to_apply.push(event.action.clone());
                if event.one_shot {
                    events_to_mark_fired.push(idx);
                }
            }
        }
    }

    for idx in events_to_mark_fired {
        state.events[idx].fired = true;
    }

    for action in actions_to_apply {
        match action {
            EventAction::Message(msg) => {
                messages.push(OutputLine {
                    text: msg,
                    line_type: LineType::Narration,
                });
            }
            EventAction::ApplyStatus(effect) => {
                let name = effect.name.clone();
                state.player.status_effects.push(effect);
                messages.push(OutputLine {
                    text: format!("You are now affected by: {}", name),
                    line_type: LineType::System,
                });
            }
            EventAction::RemoveStatus(name) => {
                state.player.status_effects.retain(|e| e.name != name);
                messages.push(OutputLine {
                    text: format!("{} has worn off.", name),
                    line_type: LineType::System,
                });
            }
            EventAction::ChangeDescription(loc_id, new_desc) => {
                if let Some(loc) = state.locations.get_mut(&loc_id) {
                    loc.description = new_desc;
                }
            }
            _ => {
                // Other actions handled by process_events
            }
        }
    }

    // Tick down status effects
    let mut expired = Vec::new();
    for effect in &mut state.player.status_effects {
        if effect.damage_per_turn != 0 {
            state.player.health = (state.player.health - effect.damage_per_turn)
                .max(0)
                .min(state.player.max_health);
            if effect.damage_per_turn > 0 {
                messages.push(OutputLine {
                    text: format!(
                        "{} deals {} damage! (HP: {})",
                        effect.name, effect.damage_per_turn, state.player.health
                    ),
                    line_type: LineType::Combat,
                });
            } else {
                messages.push(OutputLine {
                    text: format!(
                        "{} restores {} HP. (HP: {})",
                        effect.name, -effect.damage_per_turn, state.player.health
                    ),
                    line_type: LineType::System,
                });
            }
        }
        effect.turns_remaining -= 1;
        if effect.turns_remaining <= 0 {
            expired.push(effect.name.clone());
        }
    }
    for name in &expired {
        messages.push(OutputLine {
            text: format!("{} has worn off.", name),
            line_type: LineType::System,
        });
    }
    state.player.status_effects.retain(|e| e.turns_remaining > 0);

    messages
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_test_state() -> WorldState {
        let mut state = WorldState::default();
        state.locations.insert(
            "test_room".into(),
            Location {
                id: "test_room".into(),
                name: "Test".into(),
                description: "Test room.".into(),
                items: vec![],
                npcs: vec![],
                exits: HashMap::new(),
                locked_exits: HashMap::from([(Direction::North, "key1".into())]),
                visited: false,
                discovered_secrets: vec![],
                ambient_mood: Mood::Peaceful,
                examine_details: None,
                revisit_description: None,
            },
        );
        state.player.location = "test_room".into();
        state
    }

    #[test]
    fn process_damage_event() {
        let mut state = make_test_state();
        state.events.push(GameEvent {
            trigger: EventTrigger::OnEnter,
            action: EventAction::Damage(10),
            one_shot: false,
            fired: false,
            location_id: "test_room".into(),
        });

        let msgs = process_events(&EventTrigger::OnEnter, "test_room", &mut state);
        assert_eq!(state.player.health, 90);
        assert!(!msgs.is_empty());
    }

    #[test]
    fn one_shot_event_only_fires_once() {
        let mut state = make_test_state();
        state.events.push(GameEvent {
            trigger: EventTrigger::OnEnter,
            action: EventAction::Message("Trap!".into()),
            one_shot: true,
            fired: false,
            location_id: "test_room".into(),
        });

        let msgs1 = process_events(&EventTrigger::OnEnter, "test_room", &mut state);
        assert_eq!(msgs1.len(), 1);
        let msgs2 = process_events(&EventTrigger::OnEnter, "test_room", &mut state);
        assert_eq!(msgs2.len(), 0);
    }

    #[test]
    fn unlock_event_removes_locked_exit() {
        let mut state = make_test_state();
        state.events.push(GameEvent {
            trigger: EventTrigger::OnUse("scroll".into()),
            action: EventAction::Unlock(Direction::North),
            one_shot: true,
            fired: false,
            location_id: "test_room".into(),
        });

        let trigger = EventTrigger::OnUse("scroll".into());
        process_events(&trigger, "test_room", &mut state);
        let loc = state.locations.get("test_room").unwrap();
        assert!(!loc.locked_exits.contains_key(&Direction::North));
    }

    #[test]
    fn test_turn_event_fires() {
        let mut state = make_test_state();
        state.player.turns_elapsed = 5;
        state.events.push(GameEvent {
            trigger: EventTrigger::OnTurn(5),
            action: EventAction::Message("The ground trembles!".into()),
            one_shot: true,
            fired: false,
            location_id: "test_room".into(),
        });

        let msgs = process_turn_events(&mut state);
        assert_eq!(msgs.len(), 1);
        assert!(msgs[0].text.contains("The ground trembles!"));
        assert!(state.events[0].fired);

        // Should not fire again (one_shot)
        let msgs2 = process_turn_events(&mut state);
        assert!(msgs2.is_empty());
    }

    #[test]
    fn test_status_effect_tick_down() {
        let mut state = make_test_state();
        state.player.status_effects.push(StatusEffect {
            effect_type: StatusEffectType::Poison,
            name: "Poison".into(),
            turns_remaining: 3,
            damage_per_turn: 5,
            attack_modifier: 0,
            defense_modifier: 0,
        });

        let msgs = process_turn_events(&mut state);
        // Should deal 5 damage
        assert_eq!(state.player.health, 95);
        assert!(msgs.iter().any(|m| m.text.contains("Poison deals 5 damage")));
        // Turns remaining should be decremented to 2
        assert_eq!(state.player.status_effects.len(), 1);
        assert_eq!(state.player.status_effects[0].turns_remaining, 2);

        // Tick again
        process_turn_events(&mut state);
        assert_eq!(state.player.health, 90);
        assert_eq!(state.player.status_effects[0].turns_remaining, 1);

        // Tick again — should expire
        let msgs3 = process_turn_events(&mut state);
        assert_eq!(state.player.health, 85);
        assert!(msgs3.iter().any(|m| m.text.contains("has worn off")));
        assert!(state.player.status_effects.is_empty());
    }
}
